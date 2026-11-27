import * as fs from "node:fs";
import * as path from "node:path";
import * as vscode from "vscode";
import type * as lsp from "vscode-languageclient/node";
import {
  LanguageClient,
  type LanguageClientOptions,
  NotificationType,
  RequestType0,
  type ServerOptions,
} from "vscode-languageclient/node";

import { registerCompileCommands } from "./compileCommands";
import { registerTasks } from "./tasks";

/** Asks the server to run xmake again for its version and API lists. */
const ReloadRequest = new RequestType0<null, void>("xmake-analyzer/reload");
/** Deletes the cache of the server; answers its directory. */
const ClearCacheRequest = new RequestType0<string, void>("xmake-analyzer/clearCache");

/** The `.lua` files loaded by `includes()` of `xmake.lua` files. */
const IncludedFilesNotification = new NotificationType<{ uris: string[] }>(
  "xmake-analyzer/includedFiles",
);

const SECTION = "xmake-analyzer";
const LANGUAGE = "xmake";
/** Where the patterns this extension wrote to the settings are remembered. */
const CLAIMED_KEY = "claimedPatterns";

let client: LanguageClient | undefined;
let output: vscode.LogOutputChannel | undefined;
let extensionContext: vscode.ExtensionContext | undefined;

/** File system paths of the included files, compared case-insensitively on Windows. */
let included = new Set<string>();

const key = (uri: vscode.Uri): string =>
  process.platform === "win32" ? uri.fsPath.toLowerCase() : uri.fsPath;

export async function activate(
  context: vscode.ExtensionContext,
): Promise<void> {
  extensionContext = context;
  output = vscode.window.createOutputChannel("xmake Analyzer", { log: true });
  context.subscriptions.push(
    output,
    vscode.commands.registerCommand(`${SECTION}.restartServer`, () =>
      restart(context),
    ),
    vscode.commands.registerCommand(`${SECTION}.reloadXmake`, reloadXmake),
    vscode.commands.registerCommand(`${SECTION}.clearCache`, clearCache),
    vscode.commands.registerCommand(`${SECTION}.runTask`, runTask),
    vscode.commands.registerCommand(`${SECTION}.showReferences`, showReferences),
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (!e.affectsConfiguration(SECTION) || !client?.isRunning()) return;
      if (e.affectsConfiguration(`${SECTION}.server.path`)) {
        void restart(context);
      } else {
        void client.sendNotification("workspace/didChangeConfiguration", {
          settings: settings(),
        });
      }
    }),
    vscode.workspace.onDidOpenTextDocument(claimIfIncluded),
    // Settings edited or reverted outside: claim the files again.
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (
        e.affectsConfiguration("files.associations") ||
        e.affectsConfiguration("Lua.workspace.ignoreDir") ||
        e.affectsConfiguration(`${SECTION}.claimIncludedFiles`) ||
        e.affectsConfiguration(`${SECTION}.luaIgnoreDir`)
      ) {
        // Our own writes come here too: the files of the server, not the
        // stored claims (stale while a write is in flight), else two
        // writes undo each other forever.
        void queueClaims(claimedFiles ?? previouslyIncluded());
      }
    }),
  );
  // The included files known last time, before the server reports them:
  // documents restored or opened early (the `git:` side of a diff too)
  // switch language at once.
  const known = previouslyIncluded();
  included = new Set(known.map(key));
  vscode.workspace.textDocuments.forEach(claimIfIncluded);
  await queueClaims(known);
  void removeStubsLibrary();
  registerCompileCommands(context, output);
  registerTasks(context);
  await start(context);
  void warnAboutOfficialExtension(context);
}

/** The id of the official extension of xmake. */
const OFFICIAL_EXTENSION = "tboox.xmake-vscode";

/**
 * The official extension completes and checks `xmake.lua` too: both lists
 * mix in the editor. Says so once, until dismissed for good.
 */
async function warnAboutOfficialExtension(
  context: vscode.ExtensionContext,
): Promise<void> {
  const dismissed = "officialExtensionWarningDismissed";
  if (
    !vscode.extensions.getExtension(OFFICIAL_EXTENSION) ||
    context.globalState.get<boolean>(dismissed)
  ) {
    return;
  }
  const show = "Show Extension";
  const never = "Don't Show Again";
  const choice = await vscode.window.showWarningMessage(
    "The xmake extension (tboox.xmake-vscode) is enabled too: its completion " +
      "mixes with that of xmake Analyzer in xmake.lua. Disable it (for this " +
      "workspace at least) to keep only one.",
    show,
    never,
  );
  if (choice === show) {
    await vscode.commands.executeCommand(
      "workbench.extensions.search",
      `@id:${OFFICIAL_EXTENSION}`,
    );
  } else if (choice === never) {
    await context.globalState.update(dismissed, true);
  }
}

export async function deactivate(): Promise<void> {
  await client?.stop();
  client = undefined;
}

/** The server settings, sent as `initializationOptions` and on change. */
function settings(): unknown {
  const config = vscode.workspace.getConfiguration(SECTION);
  return {
    xmakePath: config.get<string>("xmakePath"),
    queryXmake: config.get<boolean>("queryXmake"),
    luaVersion: config.get<string>("luaVersion"),
    diagnostics: { enable: config.get<boolean>("diagnostics.enable") },
    format: {
      indentScopes: config.get<boolean>("format.indentScopes"),
      maxBlankLines: config.get<number>("format.maxBlankLines"),
    },
    // This extension provides `xmake-analyzer.runTask`.
    // This extension provides `xmake-analyzer.runTask` and `xmake-analyzer.showReferences`.
    codeLens: { runTask: config.get<boolean>("codeLens.runTask"), references: true },
    lints: config.get<Record<string, string>>("lints"),
  };
}

/** The configured server, else the one bundled next to the extension. */
function serverPath(context: vscode.ExtensionContext): string {
  const configured = vscode.workspace
    .getConfiguration(SECTION)
    .get<string>("server.path")
    ?.trim();
  if (configured) return configured;
  const exe =
    process.platform === "win32" ? "xmake_analyzer.exe" : "xmake_analyzer";
  return path.join(context.extensionPath, "build", exe);
}

async function start(context: vscode.ExtensionContext): Promise<void> {
  const command = serverPath(context);
  if (!fs.existsSync(command)) {
    void vscode.window.showErrorMessage(
      `xmake language server not found: ${command}. Build it with \`cargo build --release\` or set \`${SECTION}.server.path\`.`,
    );
    return;
  }
  const serverOptions: ServerOptions = {
    run: { command },
    debug: { command, options: { env: { ...process.env, RUST_LOG: "debug" } } },
  };
  const clientOptions: LanguageClientOptions = {
    // `xmake.lua` and the files it includes, once given the `xmake` language.
    documentSelector: [{ scheme: "file", language: LANGUAGE }],
    initializationOptions: settings(),
    outputChannel: output,
  };
  client = new LanguageClient(
    SECTION,
    "xmake Analyzer",
    serverOptions,
    clientOptions,
  );
  client.onNotification(
    IncludedFilesNotification,
    ({ uris }) => void onIncludedFiles(uris),
  );
  try {
    await client.start();
  } catch (error) {
    output?.error(
      `Failed to start the xmake language server: ${String(error)}`,
    );
    void vscode.window.showErrorMessage(
      "Failed to start the xmake language server. See the output for details.",
    );
  }
}

async function restart(context: vscode.ExtensionContext): Promise<void> {
  await client?.stop();
  client = undefined;
  await start(context);
}

async function reloadXmake(): Promise<void> {
  if (!client?.isRunning()) {
    void vscode.window.showErrorMessage(
      "The xmake language server is not running.",
    );
    return;
  }
  await client.sendRequest(ReloadRequest);
  void vscode.window.setStatusBarMessage("xmake API lists reloaded", 3000);
}

/**
 * `xmake-analyzer.clearCache`: deletes the outputs of xmake and the stubs
 * written for each xmake version, then asks xmake again.
 */
async function clearCache(): Promise<void> {
  if (!client?.isRunning()) {
    void vscode.window.showErrorMessage("The xmake language server is not running.");
    return;
  }
  try {
    const dir = await client.sendRequest(ClearCacheRequest);
    void vscode.window.showInformationMessage(
      dir ? `xmake Analyzer: deleted the cache ${dir}.` : "xmake Analyzer: no cache.",
    );
  } catch (error) {
    void vscode.window.showErrorMessage(`xmake Analyzer: ${String(error)}`);
  }
}

// === Included files ===

/**
 * The included files and the imported modules (`import("utils")`) are xmake
 * files: give them the `xmake` language and keep Lua language servers away,
 * which would report the xmake functions as undefined globals (and miss that
 * `raise` does not return).
 */
async function onIncludedFiles(uris: string[]): Promise<void> {
  const files = uris.map((uri) => vscode.Uri.parse(uri));
  included = new Set(files.map(key));
  claimedFiles = files;
  vscode.workspace.textDocuments.forEach(claimIfIncluded);
  await queueClaims(files);
}

/** Where `Lua.workspace.library` got the stubs, per workspace folder (0.3.0). */
const LIBRARY_KEY = "luaLibrary";

/**
 * 0.3.0 added the stubs to `Lua.workspace.library`: an absolute path in the
 * settings of the project, of no use elsewhere. Removes it: the entry it
 * wrote, and any directory of stubs of this extension (its cache, its
 * `build/stubs`); the user's entries stay.
 */
async function removeStubsLibrary(): Promise<void> {
  const state = extensionContext?.workspaceState;
  if (!state) return;
  const ours = (p: string) => /xmake_analyzer[\\/]cache[\\/]|xmake-analyzer[^\\/]*[\\/]build[\\/]stubs/i.test(p);
  for (const folder of vscode.workspace.workspaceFolders ?? []) {
    const key = `${LIBRARY_KEY}:${folder.uri.toString()}`;
    const previous = state.get<string>(key);
    const config = vscode.workspace.getConfiguration("Lua", folder.uri);
    const current = config.inspect<string[]>("workspace.library")?.workspaceFolderValue;
    if (!current) {
      if (previous) await state.update(key, undefined);
      continue;
    }
    const kept = current.filter((p) => p !== previous && !ours(p));
    if (kept.length !== current.length) {
      try {
        await config.update(
          "workspace.library",
          kept.length > 0 ? kept : undefined,
          vscode.ConfigurationTarget.WorkspaceFolder,
        );
        output?.info(`${folder.name}: removed the stubs from Lua.workspace.library`);
      } catch (error) {
        output?.warn(`could not update Lua.workspace.library: ${String(error)}`);
      }
    }
    await state.update(key, undefined);
  }
}

/** The files the server reported last. */
let claimedFiles: vscode.Uri[] | undefined;
/** The writes of the claims, one after the other. */
let claims: Promise<void> = Promise.resolve();

/** Writes the claims after the writes already queued. */
function queueClaims(files: vscode.Uri[]): Promise<void> {
  claims = claims.then(() => writeClaims(files)).catch((error: unknown) => {
    output?.warn(`could not claim the included files: ${String(error)}`);
  });
  return claims;
}

/** An open document: switch its language now. */
function claimIfIncluded(document: vscode.TextDocument): void {
  // Any scheme: the `git:` side of a diff is the same file at another
  // revision, and `Lua.workspace.ignoreDir` only hides the files on disk.
  const isXmake =
    included.has(key(document.uri)) ||
    path.basename(document.uri.path) === "xmake.lua";
  if (document.languageId === "lua" && isXmake) {
    void vscode.languages.setTextDocumentLanguage(document, LANGUAGE);
  }
}

/** What this extension wrote to the settings of a workspace folder. */
interface Claims {
  /** Keys of `files.associations`. */
  associations: string[];
  /** Patterns of `Lua.workspace.ignoreDir`. */
  ignored: string[];
}

/**
 * Writes, per workspace folder:
 * - the included files to `files.associations` (`xmake` language), so that
 *   they open as xmake directly;
 * - every `xmake.lua` and the included files to `Lua.workspace.ignoreDir` of
 *   the Lua extension (sumneko). It treats every `*.lua` file as Lua,
 *   whatever `files.associations` says, and diagnoses the whole workspace,
 *   open files or not: ignoring them is the only way to silence it.
 *
 * Only the entries this extension wrote before are replaced: the user's own
 * stay.
 */
async function writeClaims(files: vscode.Uri[]): Promise<void> {
  const state = extensionContext?.workspaceState;
  if (!state) return;
  // Each write can be turned off; the entries written before are then
  // removed, as the wanted ones are none.
  const config = vscode.workspace.getConfiguration(SECTION);
  const associate = config.get<boolean>("claimIncludedFiles", true);
  const ignoreLua = config.get<boolean>("luaIgnoreDir", true);
  const sumneko = vscode.extensions.getExtension("sumneko.lua") !== undefined;
  for (const folder of vscode.workspace.workspaceFolders ?? []) {
    const hasProject =
      (
        await vscode.workspace.findFiles(
          new vscode.RelativePattern(folder, "**/xmake.lua"),
          null,
          1,
        )
      ).length > 0;
    const relative = files
      .map((file) => path.relative(folder.uri.fsPath, file.fsPath))
      .filter((rel) => rel && !rel.startsWith("..") && !path.isAbsolute(rel))
      .map((rel) => rel.split(path.sep).join("/"))
      .sort();
    const wanted: Claims = {
      associations: associate ? relative.map((rel) => `**/${rel}`) : [],
      // gitignore syntax: `xmake.lua` matches in every directory.
      ignored:
        ignoreLua && sumneko && hasProject
          ? ["xmake.lua", ...relative.map((rel) => `/${rel}`)]
          : [],
    };
    const stateKey = `${CLAIMED_KEY}:${folder.uri.toString()}`;
    const previous = storedClaims(state, stateKey);
    // The settings may have been reverted since (`git checkout`, by hand):
    // what they hold counts, not what was written last time.
    const unchanged = JSON.stringify(previous) === JSON.stringify(wanted);
    if (unchanged && claimsPresent(folder, wanted, sumneko)) continue;

    try {
      await updateAssociations(
        folder,
        previous.associations,
        wanted.associations,
      );
      if (sumneko) {
        await updateLuaIgnore(folder, previous.ignored, wanted.ignored);
      }
      await state.update(stateKey, wanted);
      output?.info(
        `${folder.name}: ${relative.length} included files claimed, ${wanted.ignored.length} patterns ignored by the Lua extension`,
      );
    } catch (error) {
      output?.warn(
        `could not update the settings of ${folder.name}: ${String(error)}`,
      );
    }
  }
}

/** Whether the settings of `folder` hold every one of `claims`. */
function claimsPresent(
  folder: vscode.WorkspaceFolder,
  claims: Claims,
  sumneko: boolean,
): boolean {
  const associations =
    vscode.workspace
      .getConfiguration("files", folder.uri)
      .inspect<Record<string, string>>("associations")?.workspaceValue ?? {};
  const ignored =
    vscode.workspace
      .getConfiguration("Lua", folder.uri)
      .inspect<string[]>("workspace.ignoreDir")?.workspaceFolderValue ?? [];
  return (
    claims.associations.every((p) => associations[p] === LANGUAGE) &&
    (!sumneko || claims.ignored.every((p) => ignored.includes(p)))
  );
}

/** The claims of a folder; version 0.1 stored the relative paths only. */
function storedClaims(state: vscode.Memento, stateKey: string): Claims {
  const stored = state.get<Claims | string[]>(stateKey);
  if (Array.isArray(stored)) {
    return {
      associations: stored.map((rel) => `**/${rel}`),
      ignored: stored.map((rel) => `/${rel}`),
    };
  }
  return stored ?? { associations: [], ignored: [] };
}

/** The included files claimed in an earlier session, until the server tells. */
function previouslyIncluded(): vscode.Uri[] {
  const state = extensionContext?.workspaceState;
  if (!state) return [];
  return (vscode.workspace.workspaceFolders ?? []).flatMap((folder) =>
    storedClaims(
      state,
      `${CLAIMED_KEY}:${folder.uri.toString()}`,
    ).associations.map((pattern) =>
      vscode.Uri.joinPath(folder.uri, pattern.replace(/^\*\*\//, "")),
    ),
  );
}

async function updateAssociations(
  folder: vscode.WorkspaceFolder,
  previous: string[],
  wanted: string[],
) {
  // `files.associations` is a window setting: the workspace holds it, not
  // the folder (writing there fails).
  const config = vscode.workspace.getConfiguration("files", folder.uri);
  const current = {
    ...(config.inspect<Record<string, string>>("associations")
      ?.workspaceValue ?? {}),
  };
  for (const pattern of previous) {
    if (current[pattern] === LANGUAGE) delete current[pattern];
  }
  for (const pattern of wanted) current[pattern] = LANGUAGE;
  const value = Object.keys(current).length > 0 ? current : undefined;
  await config.update(
    "associations",
    value,
    vscode.ConfigurationTarget.Workspace,
  );
}

async function updateLuaIgnore(
  folder: vscode.WorkspaceFolder,
  previous: string[],
  wanted: string[],
) {
  const config = vscode.workspace.getConfiguration("Lua", folder.uri);
  const ours = new Set(previous);
  const current = (
    config.inspect<string[]>("workspace.ignoreDir")?.workspaceFolderValue ?? []
  ).filter((pattern) => !ours.has(pattern));
  const value = [
    ...current,
    ...wanted.filter((pattern) => !current.includes(pattern)),
  ];
  await config.update(
    "workspace.ignoreDir",
    value.length > 0 ? value : undefined,
    vscode.ConfigurationTarget.WorkspaceFolder,
  );
}

// === Tasks ===

/**
 * `xmake-analyzer.runTask`, from the code lens of a `task(...)` with a
 * `set_menu`: runs `xmake <name>` in a terminal, in the project directory.
 */
function runTask(name: string, fileUri: string): void {
  const file = vscode.Uri.parse(fileUri);
  const folder = vscode.workspace.getWorkspaceFolder(file);
  // The workspace folder when it holds the root xmake.lua, else the folder of the file.
  const projectDir =
    folder && fs.existsSync(path.join(folder.uri.fsPath, "xmake.lua"))
      ? folder.uri.fsPath
      : path.dirname(file.fsPath);
  const xmake =
    vscode.workspace
      .getConfiguration(SECTION)
      .get<string>("xmakePath")
      ?.trim() || "xmake";
  const quote = (arg: string) => (/\s/.test(arg) ? `"${arg}"` : arg);
  const terminal = vscode.window.createTerminal({
    name: `xmake ${name}`,
    cwd: projectDir,
  });
  terminal.show();
  terminal.sendText(`${quote(xmake)} ${quote(name)}`);
}

// === References ===

/**
 * `xmake-analyzer.showReferences`, from the `N references` code lenses. The
 * server sends protocol values; `editor.action.showReferences` takes editor
 * ones.
 */
async function showReferences(uri: string, position: lsp.Position, locations: lsp.Location[]): Promise<void> {
  if (!client) {
    return;
  }
  const c = client.protocol2CodeConverter;
  await vscode.commands.executeCommand(
    "editor.action.showReferences",
    c.asUri(uri),
    c.asPosition(position),
    locations.map((l) => c.asLocation(l)),
  );
}
