# xmake Analyzer

[Repository](https://github.com/SARDONYX-sard/xmake_analyzer) · [Issues](https://github.com/SARDONYX-sard/xmake_analyzer/issues)

A language server for [xmake](https://xmake.io) build scripts (`xmake.lua`), in Rust.

The official extension completes every API name everywhere. xmake Analyzer knows
where you are: which scope (`target`, `option`, `rule`, ...), description code or a
script callback, which instance a callback receives, and which Lua version xmake
runs.

## Features

|             |                                                                                                                               |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Completion  | the APIs of the current scope · `target:` methods in callbacks · `os.`, imported modules · values in strings (see below) · LuaCATS annotations (`---@param`, types) |
| Diagnostics | syntax · Lua version · API not available in this scope · unknown API, value, method, module function, target · `*_end()` |
| Quick fixes | typos (`set_knd` → `set_kind`), add `add_requires`/`import`, remove unused requires, `add_subdirs` → `includes`, allow a lint |
| Refactorings | `target("app")` ... `target_end()` ⇄ `target("app", function () ... end)`, for every scope (option, rule, package, ...) |
| Hover       | rustdoc-like pages: signature, summary, `# Examples`, `# Notes`, link to the manual (`crates/xmake_api/docs/`)        |
| Navigation  | hover · definition · declaration · references · highlights · rename (locals, targets, options, rules, toolchains)             |
| Paths       | go to the files of `includes("../")`, `add_files("src/**.c\|main.c")`, `import("modules.x")`                                  |
| Editing     | signature help · inlay hints (`function (target: target)`, parameter names, `end) -- on_build`) · formatting                 |
| Display     | semantic tokens · folding · document and workspace symbols · reference counts of targets (code lens)                          |

Packages: `add_requires` reads the package repositories cloned by xmake
(`~/.xmake/repositories`, `%LOCALAPPDATA%\.xmake` on Windows, or
`$XMAKE_GLOBALDIR`), offline: package names, then versions after a blank, the
keys of the options table and of `configs` (with those of `add_configs` of the
package); hover shows the description, versions and configurations; go to
definition opens the `package(...)` of the repository; `unknown-package` and
`unknown-package-version` check them.

Values completed in strings: `set_kind`, `set_languages`, `set_warnings`, `set_optimize`, ... ·
`add_rules` (builtin rules and the rules of the project) · `set_policy` · `set_toolchains` ·
`is_plat`, `is_arch`, `is_mode` · `add_deps` (targets) · `add_options`, `has_config` (options) ·
`add_packages` (`add_requires`) · `import` (modules) · file paths for `add_files`, `includes`, ...

### xmake and Lua versions

- **API lists**: at start, the server asks the installed xmake for its APIs
  (`xmake show -l apis --json`, rules, policies, toolchains, platforms, ...). The
  answers are cached per xmake build, so later starts only run `xmake --version`.
  Without xmake, the lists bundled in the server are used (generated from xmake
  3.1.1 by `tools/gen_builtin.py`).
- **Lua version**: xmake runs Lua 5.4 or LuaJIT. The parser accepts the syntax of
  Lua 5.1 to 5.5 and reports what the selected version lacks (`//` and bitwise
  operators before 5.3, `goto` in 5.1, `<const>` before 5.4, `global` before 5.5).
  `auto` follows the runtime of the installed xmake.
- `set_xmakever("x.y.z")` newer than the installed xmake is reported.

## Linter

Lints, each with a level (`allow`, `warn`, `deny`). What makes xmake fail is
an error by default (`deny`): unknown APIs, rules, toolchains, packages and
package versions, module functions and methods, APIs in the wrong scope,
`add_deps` cycles and a too new `set_xmakever`. What xmake skips or only warns
about (a missing `add_deps` target, an `add_files` matching nothing, ...) is a
warning.

| Lint                                   | Finds                                                                     |
| -------------------------------------- | ------------------------------------------------------------------------- |
| `undefined-global`                     | names defined nowhere; `import` in description code, `add_files` in scripts |
| `unused-local`                         | locals never read (prefix with `_` to keep them)                          |
| `unused-require` · `missing-require`   | `add_requires` without `add_packages`, and the reverse (aliases followed) |
| `self-dependency` · `dependency-cycle` | `add_deps` loops across the project                                       |
| `duplicate-definition`                 | two targets, options, rules, ... with the same name                       |
| `unknown-*`                            | APIs, values, targets, rules, toolchains, options, methods, module functions |
| `wrong-scope` · `mismatched-scope-end` | `set_kind` in an option, `option_end()` closing a target                  |
| `deprecated-api` · `xmake-version`     | `add_subdirs`; `set_xmakever` newer than the installed xmake              |
| `mismatched-type`                      | arguments of the wrong type, from the prototypes of the manual and the stubs: `set_default("false")`, `os.cp("a", 1)` |
| `unmatched-path` · `unresolved-include` | `add_files` globs matching nothing, missing files and directories, `includes` loading nothing |
| `unknown-package` · `unknown-package-version` | `add_requires` checked against the package repositories        |

```sh
xmake_analyzer lint [-A|-W|-D <lint>] [-D warnings] [--fix] [--xmake] [DIR]...
xmake_analyzer lint --list
```

Names (targets, options, rules, ...) are looked up in the project of the file:
the files connected to it by `includes()`, not other projects of the workspace.
When an `includes()` loads nothing known, the checks of unknown names stay quiet
for that project.

`--fix` applies the fixes that are certainly meant (deprecated APIs, scope
ends, unused and missing requires, self dependencies); typo corrections are
only offered in the editor.

```lua
-- xmake-analyzer: allow(unused-local)          -- this line and the next
-- xmake-analyzer: allow-file(undefined-global) -- the whole file
```

In VS Code: `"xmake-analyzer.lints": {"unused-local": "allow"}`.

### GitHub Action

`setup-xmake_analyzer` puts the executable of a release on the `PATH` (no Rust,
no build); what to run with it is up to the workflow.

```yaml
- uses: actions/checkout@v7
- uses: SARDONYX-sard/xmake_analyzer@0.1.1 # installs this release, unless `version` says another
  with:
    version: latest # or a tag: `0.1.1`
- run: xmake_analyzer lint -D warnings .
- run: xmake_analyzer fmt --check xmake.lua src
```

The input `version` defaults to the tag the action is used at, else `latest`;
the outputs `path` and `version` give the executable and the release installed.

Each release carries the bare executables (`xmake_analyzer-<target>[.exe]`) the
action downloads, for Linux, macOS and Windows, x64 and ARM64.

## Types and stubs

The script APIs are described by stubs in `crates/xmake_api/stubs/*.lua`, written
with LuaCATS annotations (`---@class`, `---@param`, `---@return`) and Markdown
documentation (`# Examples`, `# Notes`). Types flow from them:

```lua
on_load(function (target)
    local dep = target:dep("core")   -- target? → dep: target
    dep:targetfile()                 -- completion, hover, signature and checks of target methods
    local name = target:name()       -- string → name:split(",")
    import("core.base.json")         -- json: module core.base.json
end)
```

Tables show their keys (positional ones as `[1]`), types and values on hover:
`local config: { name: string = "app", [1]: integer = 42 }`,
`(field) config.name: string = "app"`.

Documentation comments above locals, functions and `target(...)` (`--`, `---`,
`--[[ ]]`) show in hovers as Markdown; `<const>` locals show their value.

## Formatter

```sh
xmake_analyzer fmt [--check] [--tabs N | --spaces N] [--lua VERSION] [--no-scope-indent] [PATH]...
```

- Indentation from the editor, then `.editorconfig` (`indent_style`, `indent_size`,
  `tab_width`, `end_of_line`, `insert_final_newline`); command line flags win.
  With tabs, a level is `indent_size / tab_width` tabs.
- The contents of `target(...)` ... `target_end()` are indented one level, as in the
  xmake manual (`format.indentScopes`).
- Spaces are normalized (`a = b + 1`, `f(a, b)`, `function (target)`, `{x = 1}`), line
  breaks are kept, blank lines are shortened, comments are never moved.
- Files with syntax errors are left alone, and the output is checked to have the
  same tokens as the input.

## VS Code

Install `build/xmake-analyzer.vsix` (`npm run package`).

Analyzed files: every `xmake.lua`, and the files they load with `includes()`
(`includes("../common.lua")`, `includes("**/xmake.lua")`, directories), followed
recursively. These files get the `xmake` language, as `xmake.lua` with the official
extension, so that a Lua language server does not report their xmake functions as
undefined globals: the extension switches open documents, and adds the files to
`files.associations` and (with the sumneko Lua extension) `Lua.workspace.ignoreDir`
of the workspace folder settings (`claimIncludedFiles`). Other `*.lua` files are left
to the Lua language server.

Tasks with a `set_menu` get a `▶ Run xmake <name>` code lens, running them in a
terminal.

`compile_commands.json`, for C/C++ language servers (clangd, ...), is written by
`xmake project -k compile_commands` when an `xmake.lua` or a file it includes is
saved, in the project directory (the highest directory with an `xmake.lua`):

| Setting                                    | Default    | Meaning                                                  |
| ------------------------------------------ | ---------- | -------------------------------------------------------- |
| `xmake-analyzer.compileCommands.auto`      | `onSave`   | `off`: only with the command                             |
| `xmake-analyzer.compileCommands.directory` | `.vscode`  | where it goes, relative to the project; `${workspaceFolder}` |
| `xmake-analyzer.compileCommands.lsp`       | `clangd`   | `--lsp` of `xmake project`: `clangd`, `cpptools`, `ccls`  |

The command `xmake Analyzer: Generate compile_commands.json` runs it on demand.

The official xmake extension (`tboox.xmake-vscode`) completes `xmake.lua` too, and
its items mix with these: when it is enabled, a warning offers to show it, to
disable it for the workspace.

| Setting                                | Default | Meaning                                                    |
| -------------------------------------- | ------- | ---------------------------------------------------------- |
| `xmake-analyzer.server.path`           | bundled | the `xmake_analyzer` executable                            |
| `xmake-analyzer.xmakePath`             | `xmake` | the xmake asked for its version and APIs                   |
| `xmake-analyzer.queryXmake`            | `true`  | use the API lists of the installed xmake                   |
| `xmake-analyzer.luaVersion`            | `auto`  | `5.1`, `luajit`, `5.2`, `5.3`, `5.4`, `5.5`                |
| `xmake-analyzer.diagnostics.enable`    | `true`  |                                                            |
| `xmake-analyzer.claimIncludedFiles`    | `true`  | keep Lua language servers away from included files         |
| `xmake-analyzer.codeLens.runTask`      | `true`  | `▶ Run` lens on tasks                                      |
| `xmake-analyzer.format.indentScopes`   | `true`  | indent the contents of `target(...)`, ...                  |
| `xmake-analyzer.format.maxBlankLines`  | `1`     |                                                            |
| `xmake-analyzer.lints`                 | `{}`    | lint levels: `{"unused-local": "allow"}`                   |

Commands: **Restart Language Server**, **Reload the API Lists of xmake**.

Tasks (**Tasks: Run Task** > `xmake`): `format` runs `xmake format` (clang-format) and
`lint` runs `xmake check clang.tidy` (clang-tidy, its warnings in the Problems view).
In `tasks.json`, `args` adds arguments:

```json
{ "type": "xmake", "task": "lint", "args": ["--fix"], "label": "xmake: lint --fix" }
```

Other editors: run `xmake_analyzer` over stdio and pass the settings above, without
the `xmake-analyzer.` prefix, as `initializationOptions`.

## Building

```sh
cargo build --release      # target/release/xmake_analyzer
npm install
npm run package            # build/xmake-analyzer.vsix with the server
npm run debug              # debug build for the launch configuration (F5)
python tools/gen_builtin.py  # regenerate the bundled API lists from the installed xmake
```

To release, from an up-to-date `main` with a clean working tree:

```sh
npm run release:patch      # or release:minor, release:major
```

It runs the tests, bumps the version (`package.json`, `Cargo.toml`, the examples of
this readme and `action.yaml`), commits, tags and pushes `main` and the tag. The tag
builds the VSIX and the executables and attaches them to the GitHub release.

| Crate            | Role                                                                     |
| ---------------- | ------------------------------------------------------------------------ |
| `xmake_syntax`   | lossless, resilient Lua parser, dialect aware (`Dialect`, `Feature`)     |
| `xmake_api`      | the API database: scopes, kinds, modules, instances, accepted values     |
| `xmake_project`  | analyzed files and the project: outlines, bindings, includes, packages   |
| `xmake_types`    | LuaCATS types (unions, `T?`, `T[]`, `table<K, V>`, `fun()`), inference   |
| `xmake_lint`     | diagnostics, lints with their levels, type checks, quick fixes           |
| `xmake_ide`      | completion, hover, navigation, inlay hints, ...                          |
| `xmake_fmt`      | the formatter and `.editorconfig`                                        |
| `xmake_analyzer` | the language server and the `fmt` and `lint` commands                    |

## License

MIT or Apache-2.0, at your option. The icons (`images/`) are original and under
the same licenses; they are not the logo of xmake.

## Acknowledgements and third-party material

xmake Analyzer is not affiliated with the xmake project. It uses or follows:

| Source | License | Used for |
| ------ | ------- | -------- |
| [xmake](https://github.com/xmake-io/xmake) | Apache-2.0 | The builtin API lists (`crates/xmake_api/src/generated.rs`, from `xmake show -l apis` by `tools/gen_builtin.py`), the tool names (`modules/core/tools`), and the behaviors the checks follow, read from its sources (script APIs, `$(...)` variables, `includes`, errors raised). The stubs describe its script APIs. |
| [xmake-docs](https://github.com/xmake-io/xmake-docs), the manual at [xmake.io](https://xmake.io) | No license stated in the repository; © its authors | The links to the manual, the prototypes and the parameter descriptions of `crates/xmake_api/src/doc_links.rs` (`tools/gen_doc_links.py`), quoted to document the APIs. |
| [xmake-vscode](https://github.com/xmake-io/xmake-vscode) | Apache-2.0 | Consulted for what the official extension does; no code copied. |
| [Lua 5.x reference manuals](https://www.lua.org/manual/) | MIT (Lua) | The stubs of `string`, `table`, `math` and the base functions. |
| [LuaCATS annotations](https://luals.github.io/wiki/annotations/) of [lua-language-server](https://github.com/LuaLS/lua-language-server) | MIT | The notation of the stubs and of the types. |
| [papyrus_analyzer](https://github.com/SARDONYX-sard/papyrus_analyzer) | MIT or Apache-2.0 (same author) | The design of the parser, the syntax tree and the server. |

The licenses of the Rust dependencies are listed in `LICENSES.toml` of each
release archive (`cargo license`, `cargo bundle-licenses`).
