---@meta
---@diagnostic disable: unused-local

--- Tasks and plugins.
---@class core.base.task
---@see https://xmake.io/api/scripts/extension-modules/core/base/task.html
local task = {}

--- Runs a task or a plugin, as `xmake <name>` would.
---
--- # Examples
---
--- ```lua
--- import("core.base.task")
--- task.run("install")
--- task.run("project", {kind = "compile_commands"})
--- ```
---@param name string
---@param opt? table The options of its menu.
---@param ... any
---@return any ...
---@see https://xmake.io/api/scripts/extension-modules/core/base/task.html#task-run
function task.run(name, opt, ...) end

function task.names() end
