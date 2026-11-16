---@meta

--- JSON encoding and decoding.
---
--- # Examples
---
--- ```lua
--- import("core.base.json")
--- local data = json.loadfile("package.json")
--- json.savefile("out.json", {name = data.name})
--- ```
---@class core.base.json
local json = {}

--- Decodes a JSON string.
---@param text string
---@return any value
function json.decode(text) end

--- Encodes a value as JSON.
---
--- # Notes
---
--- Empty tables encode as objects; mark arrays with `json.mark_as_array`.
---@param value any
---@return string
function json.encode(value) end

--- Reads and decodes a JSON file.
---@param path string
---@return any value
function json.loadfile(path) end

--- Encodes a value and writes it to a file.
---@param path string
---@param value any
function json.savefile(path, value) end

--- Marks a table to encode as an array, even when empty.
---@param value table
---@return table
function json.mark_as_array(value) end

--- The command line options of xmake and of tasks.
---@class core.base.option
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
function option.get(name) end

--- The configuration of the project (`xmake f`).
---@class core.project.config
local config = {}

--- Returns a configuration value, e.g. `plat`, `arch`, `mode`.
---
--- # Examples
---
--- ```lua
--- import("core.project.config")
--- config.load()
--- print(config.get("mode"))
--- ```
---@param name string
---@return any
function config.get(name) end

--- Sets a configuration value.
---@param name string
---@param value any
---@param opt? {readonly?: boolean, force?: boolean}
function config.set(name, value, opt) end

--- Loads the configuration of the project.
function config.load() end

--- Returns the build directory.
---@return string
function config.builddir() end

--- Returns the platform being configured.
---@return string
function config.plat() end

--- Returns the architecture being configured.
---@return string
function config.arch() end

--- Returns the build mode being configured.
---@return string
function config.mode() end

--- The project: its targets, options and requirements.
---@class core.project.project
local project = {}

--- Returns the target `name`.
---
--- # Examples
---
--- ```lua
--- import("core.project.project")
--- local app = project.target("app")
--- print(app:targetfile())
--- ```
---@param name string
---@return target?
function project.target(name) end

--- Returns the targets of the project, by name.
---@return table<string, target>
function project.targets() end

--- Returns the root directory of the project.
---@return string
function project.directory() end

--- Finds a program and returns its path and version.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_tool")
--- local git = find_tool("git", {version = true})
--- if git then
---     print(git.program, git.version)
--- end
--- ```
---@class lib.detect.find_tool
---@overload fun(name: string, opt?: {version?: boolean, paths?: string[], check?: string}): {program: string, version?: string}?
local find_tool = {}

--- Finds a program and returns its path.
---@class lib.detect.find_program
---@overload fun(name: string, opt?: {paths?: string[], check?: string}): string?
local find_program = {}
