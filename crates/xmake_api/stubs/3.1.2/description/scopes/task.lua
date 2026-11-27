---@meta
---@diagnostic disable: unused-local

---@class xmake.task.script
local task_script = {}

---@param script fun()|string Task execution function
---@see https://xmake.io/api/description/plugin-and-task.html#on-run
function task_script.on_run(script) end

---@class xmake.task.values
local task_values = {}

---@param category xmake.value.category Category name string
---@see https://xmake.io/api/description/plugin-and-task.html#set-category
function task_values.set_category(category) end

---@class xmake.task.dictionary
local task_dictionary = {}

---@param options xmake.options.menu Menu options table
---@see https://xmake.io/api/description/plugin-and-task.html#set-menu
function task_dictionary.set_menu(options) end
