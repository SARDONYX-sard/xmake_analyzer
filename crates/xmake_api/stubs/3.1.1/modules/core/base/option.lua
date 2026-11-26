---@meta

--- The command line options of xmake and of tasks.
---@class core.base.option
---@see https://xmake.io/api/scripts/extension-modules/core/base/option.html
local option = {}

--- Returns the value of a command line option.
---
--- # Examples
---
--- ```lua
--- import("core.base.option")
--- if option.get("verbose") then
---     print("verbose mode")
--- end
--- ```
---@param name string
---@return any
---@see https://xmake.io/api/scripts/extension-modules/core/base/option.html#option-get
function option.get(name) end

function option.boolean() end

function option.default() end

function option.defaults() end

function option.options() end

function option.parse() end

function option.raw_parse() end

function option.restore() end

function option.save() end

function option.set() end

function option.show_logo() end

function option.show_options() end

function option.taskmenu() end

function option.taskname() end
