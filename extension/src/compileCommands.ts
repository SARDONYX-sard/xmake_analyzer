/**
 * `compile_commands.json` for C/C++ language servers (clangd, ...), generated
 * by xmake after an `xmake.lua` (or a file it includes) is saved:
 *
 * ```sh
 * xmake project -k compile_commands --lsp=clangd <directory>
 * ```
 *
 * Settings: `xmake-analyzer.compileCommands.{auto,directory,lsp,configureClangd}`;
 * the command `xmake-analyzer.generateCompileCommands` runs it on demand.
 *
 * clangd finds `compile_commands.json` in the directories above a source and
 * their `build/`, not for the headers of the packages, outside the project:
 * `--compile-commands-dir` in `clangd.arguments` gives it for every file.
 */

import { spawn } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";
import * as vscode from "vscode";

const SECTION = "xmake-analyzer";
/** Saves within this delay make one run. */
const DEBOUNCE_MS = 500;
/** A run longer than this (a package download?) is stopped. */
const TIMEOUT_MS = 10 * 60 * 1000;
const DEFAULT_DIRECTORY = "build";
const CLANGD_EXTENSION = "llvm-vs-code-extensions.vscode-clangd";
const COMPILE_COMMANDS_DIR = "--compile-commands-dir=";
/** The `--compile-commands-dir=...` this extension added to `clangd.arguments`. */
const CLANGD_ARG_KEY = "clangdCompileCommandsDir";

/** The state of the runs of a project directory. */
interface Run {
  running: boolean;
  /** A save came during the run: run again after it. */
  again: boolean;
  timer?: NodeJS.Timeout;
}

const runs = new Map<string, Run>();
let output: vscode.LogOutputChannel;
let status: vscode.StatusBarItem;
let state: vscode.Memento;

export function registerCompileCommands(
  context: vscode.ExtensionContext,
  log: vscode.LogOutputChannel,
): void {
  output = log;
  state = context.workspaceState;
  status = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left);
  status.text = "$(sync~spin) compile_commands.json";
  status.tooltip = "xmake is generating compile_commands.json";
  context.subscriptions.push(
    status,
    vscode.commands.registerCommand(`${SECTION}.generateCompileCommands`, generateNow),
    vscode.workspace.onDidSaveTextDocument((document) => {
      const auto = vscode.workspace
        .getConfiguration(SECTION, document.uri)
        .get<string>("compileCommands.auto", "onSave");
      if (auto !== "onSave" || !isDescription(document)) return;
      const dir = projectDir(document.uri);
      if (dir) schedule(dir, document.uri);
    }),
    // Another directory or language server: the file is written again.
    vscode.workspace.onDidChangeConfiguration((e) => {
      for (const folder of vscode.workspace.workspaceFolders ?? []) {
        const affects = (key: string) =>
          e.affectsConfiguration(`${SECTION}.compileCommands.${key}`, folder.uri);
        if (!["directory", "lsp", "auto", "configureClangd"].some(affects)) continue;
        const config = vscode.workspace.getConfiguration(SECTION, folder.uri);
        const dir = driveUpper(folder.uri.fsPath);
        if (
          config.get<string>("compileCommands.auto", "onSave") === "onSave" &&
          fs.existsSync(path.join(dir, "xmake.lua"))
        ) {
          schedule(dir, folder.uri);
        }
      }
    }),
  );
}

/**
 * `D:\...` for `d:\...`: VS Code lowers the drive letter, and xmake writes
 * the paths of the commands from the working directory.
 */
function driveUpper(fsPath: string): string {
  return fsPath.replace(/^[a-z](?=:)/, (drive) => drive.toUpperCase());
}

/** `xmake.lua`, or a file it includes (the `xmake` language). */
function isDescription(document: vscode.TextDocument): boolean {
  return (
    document.uri.scheme === "file" &&
    (document.languageId === "xmake" ||
      path.basename(document.uri.fsPath) === "xmake.lua")
  );
}

/**
 * The project of `uri`: the highest directory holding an `xmake.lua` between
 * the file and its workspace folder.
 */
function projectDir(uri: vscode.Uri): string | undefined {
  const workspaceFolder = vscode.workspace.getWorkspaceFolder(uri)?.uri.fsPath;
  const folder = workspaceFolder && driveUpper(workspaceFolder);
  let found: string | undefined;
  let dir = driveUpper(path.dirname(uri.fsPath));
  for (;;) {
    if (fs.existsSync(path.join(dir, "xmake.lua"))) found = dir;
    const parent = path.dirname(dir);
    if (dir === folder || parent === dir) break;
    dir = parent;
  }
  return found;
}

/** `xmake-analyzer.generateCompileCommands`: the project of the active file, else of every folder. */
async function generateNow(): Promise<void> {
  const active = vscode.window.activeTextEditor?.document.uri;
  const fromActive = active?.scheme === "file" ? projectDir(active) : undefined;
  const dirs = fromActive
    ? [fromActive]
    : (vscode.workspace.workspaceFolders ?? [])
        .map((f) => driveUpper(f.uri.fsPath))
        .filter((dir) => fs.existsSync(path.join(dir, "xmake.lua")));
  if (dirs.length === 0) {
    void vscode.window.showWarningMessage("No xmake.lua found to generate compile_commands.json for.");
    return;
  }
  for (const dir of dirs) {
    await generate(dir, vscode.Uri.file(path.join(dir, "xmake.lua")));
  }
}

/** Runs after the saves settle; once more if a save comes during a run. */
function schedule(dir: string, scope: vscode.Uri): void {
  const run = runs.get(dir) ?? { running: false, again: false };
  runs.set(dir, run);
  if (run.running) {
    run.again = true;
    return;
  }
  clearTimeout(run.timer);
  run.timer = setTimeout(() => void generate(dir, scope), DEBOUNCE_MS);
}

/** Where the file goes: the setting, relative to the project directory. */
function outputDir(dir: string, scope: vscode.Uri): string {
  const folder = vscode.workspace.getWorkspaceFolder(scope)?.uri.fsPath ?? dir;
  const setting = vscode.workspace
    .getConfiguration(SECTION, scope)
    .get<string>("compileCommands.directory", DEFAULT_DIRECTORY)
    .trim()
    .replace(/\$\{workspaceFolder\}/g, folder);
  return path.resolve(dir, setting || ".");
}

async function generate(dir: string, scope: vscode.Uri): Promise<void> {
  const run = runs.get(dir) ?? { running: false, again: false };
  runs.set(dir, run);
  if (run.running) {
    run.again = true;
    return;
  }
  run.running = true;
  run.again = false;
  status.show();
  try {
    const config = vscode.workspace.getConfiguration(SECTION, scope);
    const xmake = config.get<string>("xmakePath")?.trim() || "xmake";
    const lsp = config.get<string>("compileCommands.lsp", "clangd");
    const target = outputDir(dir, scope);
    const args = ["project", "-k", "compile_commands"];
    if (lsp) args.push(`--lsp=${lsp}`);
    args.push(target);
    output.info(`${dir}: ${xmake} ${args.join(" ")}`);
    const { code, text } = await exec(xmake, args, dir);
    if (code === 0) {
      output.info(`compile_commands.json written to ${target}`);
      if (lsp === "clangd" && config.get<boolean>("compileCommands.configureClangd", true)) {
        await configureClangd(target, scope);
      }
    } else {
      output.error(`xmake failed (${code ?? "killed"}):\n${text}`);
      const show = "Show Output";
      const choice = await vscode.window.showWarningMessage(
        "xmake could not generate compile_commands.json.",
        show,
      );
      if (choice === show) output.show();
    }
  } finally {
    run.running = false;
    if (![...runs.values()].some((r) => r.running)) status.hide();
    if (run.again) schedule(dir, scope);
  }
}

/**
 * Points clangd at `target` with `--compile-commands-dir` in the workspace
 * `clangd.arguments`, and restarts it when that changed. A directory set by
 * the user is left alone; the one added here follows the setting.
 */
async function configureClangd(target: string, scope: vscode.Uri): Promise<void> {
  if (!vscode.extensions.getExtension(CLANGD_EXTENSION)) return;
  const folder = vscode.workspace.getWorkspaceFolder(scope)?.uri.fsPath;
  const relative = folder && path.relative(driveUpper(folder), target);
  const dir =
    relative !== undefined && !relative.startsWith("..") && !path.isAbsolute(relative)
      ? ["${workspaceFolder}", ...relative.split(path.sep).filter(Boolean)].join("/")
      : target.split(path.sep).join("/");
  const wanted = `${COMPILE_COMMANDS_DIR}${dir}`;
  const clangd = vscode.workspace.getConfiguration("clangd");
  const current = clangd.inspect<string[]>("arguments")?.workspaceValue ?? [];
  const added = state.get<string>(CLANGD_ARG_KEY);
  const existing = current.find((arg) => arg.startsWith(COMPILE_COMMANDS_DIR));
  if (existing === wanted) return;
  // Set by the user, or in the user settings: theirs.
  if (existing !== undefined && existing !== added) return;
  const userValue = clangd.inspect<string[]>("arguments")?.globalValue ?? [];
  if (existing === undefined && userValue.some((arg) => arg.startsWith(COMPILE_COMMANDS_DIR))) return;

  // The workspace value replaces the user one: keep the user arguments.
  const base = existing === undefined && current.length === 0 ? userValue : current;
  const updated = [...base.filter((arg) => arg !== existing), wanted];
  await clangd.update("arguments", updated, vscode.ConfigurationTarget.Workspace);
  await state.update(CLANGD_ARG_KEY, wanted);
  output.info(`clangd.arguments: ${wanted}`);
  try {
    await vscode.commands.executeCommand("clangd.restart");
  } catch (error) {
    output.warn(`clangd.restart: ${String(error)}`);
  }
}

/** Runs `program` without a terminal; its output, colors off. */
function exec(
  program: string,
  args: string[],
  cwd: string,
): Promise<{ code: number | null; text: string }> {
  return new Promise((resolve) => {
    let text = "";
    const child = spawn(program, args, {
      cwd,
      env: { ...process.env, COLORTERM: "nocolor", XMAKE_COLORTERM: "nocolor" },
      // No input: xmake takes the default answer of its questions.
      stdio: ["ignore", "pipe", "pipe"],
      windowsHide: true,
    });
    const timer = setTimeout(() => child.kill(), TIMEOUT_MS);
    child.stdout.on("data", (chunk) => {
      text += chunk;
    });
    child.stderr.on("data", (chunk) => {
      text += chunk;
    });
    child.on("error", (error) => {
      clearTimeout(timer);
      resolve({ code: -1, text: String(error) });
    });
    child.on("close", (code) => {
      clearTimeout(timer);
      resolve({ code, text });
    });
  });
}
