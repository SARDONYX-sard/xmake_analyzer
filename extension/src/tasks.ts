/**
 * VS Code tasks of type `xmake` for the formatter and the linter xmake ships
 * with (`Tasks: Run Task` > `xmake`):
 *
 * - `format`: `xmake format` (clang-format; `.clang-format` of the project)
 * - `lint`: `xmake check clang.tidy` (clang-tidy; `.clang-tidy` of the project)
 *
 * In `tasks.json`, `args` adds arguments: `{"type": "xmake", "task": "lint",
 * "args": ["--fix"]}`.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import * as vscode from "vscode";

const SECTION = "xmake-analyzer";
const TYPE = "xmake";

interface XmakeTaskDefinition extends vscode.TaskDefinition {
  task: keyof typeof COMMANDS;
  args?: string[];
}

/** The xmake arguments of each task, and its problem matchers. */
const COMMANDS = {
  format: { args: ["format"], matchers: [] as string[], detail: "xmake format (clang-format)" },
  lint: { args: ["check", "clang.tidy"], matchers: ["$xmake-clang-tidy"], detail: "xmake check clang.tidy" },
};

export function registerTasks(context: vscode.ExtensionContext): void {
  context.subscriptions.push(
    vscode.tasks.registerTaskProvider(TYPE, {
      provideTasks: () =>
        (vscode.workspace.workspaceFolders ?? [])
          .filter((folder) => fs.existsSync(path.join(folder.uri.fsPath, "xmake.lua")))
          .flatMap((folder) =>
            (Object.keys(COMMANDS) as (keyof typeof COMMANDS)[]).map((task) =>
              createTask({ type: TYPE, task }, folder),
            ),
          ),
      // A task of tasks.json: its definition is kept, the execution added.
      resolveTask: (task) => {
        const definition = task.definition as XmakeTaskDefinition;
        if (!(definition.task in COMMANDS)) return undefined;
        const folder =
          typeof task.scope === "object" ? task.scope : vscode.workspace.workspaceFolders?.[0];
        return folder ? createTask(definition, folder, task.name) : undefined;
      },
    }),
  );
}

function createTask(
  definition: XmakeTaskDefinition,
  folder: vscode.WorkspaceFolder,
  name: string = definition.task,
): vscode.Task {
  const command = COMMANDS[definition.task];
  const xmake =
    vscode.workspace.getConfiguration(SECTION, folder.uri).get<string>("xmakePath")?.trim() ||
    "xmake";
  const execution = new vscode.ProcessExecution(xmake, [...command.args, ...(definition.args ?? [])], {
    cwd: folder.uri.fsPath,
  });
  const task = new vscode.Task(definition, folder, name, TYPE, execution, command.matchers);
  task.detail = command.detail;
  return task;
}
