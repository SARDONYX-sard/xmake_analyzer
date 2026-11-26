// The tasks of the repository.
//
// usage: node tasks.cjs <task> [options]; `node tasks.cjs help` lists them.
const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");

/** Rust target triples and the matching VS Code platforms. */
const PLATFORMS = {
  "x86_64-pc-windows-msvc": "win32-x64",
  "aarch64-pc-windows-msvc": "win32-arm64",
  "x86_64-unknown-linux-gnu": "linux-x64",
  "aarch64-unknown-linux-gnu": "linux-arm64",
  "x86_64-apple-darwin": "darwin-x64",
  "aarch64-apple-darwin": "darwin-arm64",
};

/** The value after `name` in `args`. */
const option = (args, name) => {
  const i = args.indexOf(name);
  return i >= 0 ? args[i + 1] : undefined;
};

const run = (cmd) => {
  console.log(`> ${cmd}`);
  execSync(cmd, { cwd: __dirname, stdio: "inherit" });
};

/** The trimmed output of `cmd`. */
const capture = (cmd) => execSync(cmd, { cwd: __dirname, encoding: "utf8" }).trim();

/** The stubs of the APIs (`crates/xmake_api/stubs/<xmake version>/`). */
const STUBS = path.join(__dirname, "crates", "xmake_api", "stubs");

/** Copies the stubs to `dst`, which the server reads next to itself. */
function copyStubs(dst) {
  fs.rmSync(dst, { recursive: true, force: true });
  fs.cpSync(STUBS, dst, { recursive: true });
  console.log(`copied ${STUBS} -> ${dst}`);
}

/**
 * Copies the language server and the stubs next to the compiled extension in build/, and
 * optionally packages a platform specific VSIX.
 */
function build(args) {
  const target = option(args, "--target");
  const profile = option(args, "--profile") ?? "release";
  const windows = target ? target.includes("windows") : process.platform === "win32";
  const exe = windows ? "xmake_analyzer.exe" : "xmake_analyzer";
  const src = option(args, "--binary") ?? path.join(__dirname, "target", ...(target ? [target] : []), profile, exe);

  const buildDir = path.join(__dirname, "build");
  fs.mkdirSync(buildDir, { recursive: true });
  // Only one server binary per package: remove the one of another platform.
  for (const old of ["xmake_analyzer", "xmake_analyzer.exe"]) {
    if (old !== exe) fs.rmSync(path.join(buildDir, old), { force: true });
  }
  const dst = path.join(buildDir, exe);
  fs.copyFileSync(src, dst);
  if (!windows) fs.chmodSync(dst, 0o755);
  console.log(`copied ${src} -> ${dst}`);
  copyStubs(path.join(buildDir, "stubs"));

  if (!args.includes("--vsix")) return;
  run("npm run check");
  run("npm run compile");
  const platform = target ? PLATFORMS[target] : undefined;
  if (target && !platform) throw new Error(`no VS Code platform for target ${target}`);
  const out = `build/xmake-analyzer${platform ? `-${platform}` : ""}.vsix`;
  // Vite bundles the dependencies into build/extension.cjs. vsce only looks for
  // one `LICENSE[.md|.txt]` file; LICENSE-MIT and LICENSE-APACHE are packaged
  // through .vscodeignore, so its check (and its y/N prompt) is skipped.
  run(`npx vsce package --no-dependencies --skip-license ${platform ? `--target ${platform}` : ""} --out ${out}`);
  console.log(`packaged ${out}`);
}

/** The files that name the version, besides package.json and Cargo.lock. */
const VERSION_FILES = ["crates/xmake_analyzer/Cargo.toml", "readme.md", "action.yaml"];

/** `version` bumped by `kind`: `patch`, `minor` or `major`. */
function bump(version, kind) {
  const [major, minor, patch] = version.split(".").map(Number);
  switch (kind) {
    case "major":
      return `${major + 1}.0.0`;
    case "minor":
      return `${major}.${minor + 1}.0`;
    case "patch":
      return `${major}.${minor}.${patch + 1}`;
    default:
      throw new Error(`expected patch, minor or major, found ${kind}`);
  }
}

/**
 * Bumps the version, commits, tags and pushes: the tag builds the release
 * (.github/workflows/build.yaml).
 */
function release(args) {
  const [kind] = args;
  const push = !args.includes("--no-push");
  const branch = capture("git branch --show-current");
  if (branch !== "main") throw new Error(`release from main, not ${branch || "a detached HEAD"}`);
  if (capture("git status --porcelain")) throw new Error("the working tree has changes: commit or stash them");
  run("git fetch origin main --tags");
  if (capture("git rev-parse HEAD") !== capture("git rev-parse origin/main")) {
    throw new Error("main and origin/main differ: pull or push first");
  }

  const pkg = JSON.parse(fs.readFileSync(path.join(__dirname, "package.json"), "utf8"));
  const old = pkg.version;
  const next = bump(old, kind);
  if (capture(`git tag --list ${next}`)) throw new Error(`the tag ${next} exists`);
  console.log(`release ${old} -> ${next}`);

  run("cargo test --workspace --quiet");
  run("npm run check");

  run(`npm version ${next} --no-git-tag-version`);
  const escaped = old.replace(/\./g, "\\.");
  const patterns = [
    // Cargo.toml: `version = "0.1.0"` of the package.
    [new RegExp(`^(version\\s*=\\s*")${escaped}"`, "m"), `$1${next}"`],
    // readme.md, action.yaml: `@0.1.0` and `` `0.1.0` ``.
    [new RegExp(`@${escaped}\\b`, "g"), `@${next}`],
    [new RegExp(`\`${escaped}\``, "g"), `\`${next}\``],
  ];
  for (const file of VERSION_FILES) {
    const full = path.join(__dirname, file);
    const text = fs.readFileSync(full, "utf8");
    const updated = patterns.reduce((t, [re, to]) => t.replace(re, to), text);
    if (updated === text) throw new Error(`${file} does not name the version ${old}`);
    fs.writeFileSync(full, updated);
  }
  run("cargo update --workspace --offline");

  run(`git add package.json package-lock.json Cargo.lock ${VERSION_FILES.join(" ")}`);
  run(`git commit -m "chore: release ${next}"`);
  run(`git tag -a ${next} -m ${next}`);
  if (push) {
    run(`git push --atomic origin main ${next}`);
    console.log(`released ${next}: the Build workflow attaches the assets to the GitHub release`);
  } else {
    console.log(`tagged ${next}; push with: git push --atomic origin main ${next}`);
  }
}

const TASKS = {
  build: {
    usage: "[--vsix] [--target <rust triple>] [--profile <name>] [--binary <path>]",
    help: `Copies the server next to the extension in build/.
  --target   the Rust target the server was built for, e.g. x86_64-pc-windows-msvc
             (default: the host build in target/<profile>/)
  --profile  the Cargo profile (default: release)
  --binary   the server binary to copy, instead of looking in target/
  --vsix     also compile the extension and package build/xmake-analyzer-<platform>.vsix`,
    run: build,
  },
  release: {
    usage: "<patch|minor|major> [--no-push]",
    help: `Bumps the version (package.json, Cargo.toml, readme.md, action.yaml), runs the
tests, commits, tags and pushes main and the tag: the tag builds the release.
  --no-push  commit and tag only`,
    run: release,
  },
  help: {
    usage: "",
    help: "Lists the tasks.",
    run: () => {
      for (const [name, task] of Object.entries(TASKS)) {
        console.log(`node tasks.cjs ${name} ${task.usage}\n${task.help}\n`);
      }
    },
  },
};

const [name, ...rest] = process.argv.slice(2);
const task = TASKS[name ?? "help"];
if (!task) {
  console.error(`unknown task ${name}; see node tasks.cjs help`);
  process.exit(2);
}
task.run(rest);
