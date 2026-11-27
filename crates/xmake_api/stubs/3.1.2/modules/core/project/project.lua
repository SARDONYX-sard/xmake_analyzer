---@meta
---@diagnostic disable: unused-local

--- The project: its targets, options and requirements.
---@class core.project.project
---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html
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
---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-target
function project.target(name) end

--- Returns the targets of the project, by name.
---@return table<string, target>
---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-targets
function project.targets() end

--- Returns the root directory of the project.
--- Returns the directory of the project.
---@return string
---@return string
function project.directory() end

--- Returns every description file of the project.
---@return string[]
function project.allfiles() end

function project.allowed_archs() end

function project.allowed_modes() end

function project.allowed_plats() end

function project.apis() end

function project.chdir() end

function project.check_options() end

function project.default_arch() end

function project.extraconf() end

function project.filelock() end

function project.get() end

function project.is_loaded() end

function project.load_targets() end

function project.lock() end

--- Returns the build modes of the project (`mode.debug`, ...).
---@return string[]
function project.modes() end

function project.mtimes() end

--- Returns the name of the project (`set_project`).
---@return string?
---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-name
function project.name() end

function project.namespaces() end

--- Returns an option of the project.
---@param name string
---@return option?
---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-option
function project.option(name) end

--- Returns the options of the project, by name.
---@return table<string, option>
---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-options
function project.options() end

--- Returns the targets in the order of their dependencies.
---@return target[]
function project.ordertargets() end

function project.policy() end

function project.policy_set() end

function project.rcfiles() end

function project.requireconfs_str() end

function project.required_package() end

function project.required_packages() end

function project.requires_str() end

function project.requireslock() end

function project.requireslock_version() end

--- Returns the root `xmake.lua` of the project.
---@return string
function project.rootfile() end

function project.rule() end

function project.rules() end

function project.scope() end

function project.target_add() end

--- Returns the temporary directory of the project.
---@return string
function project.tmpdir() end

--- Returns a temporary file of the project for `key`.
---@param key string
---@return string
function project.tmpfile(key) end

function project.toolchain() end

function project.toolchains() end

function project.trylock() end

function project.unlock() end

--- Returns the version of the project (`set_version`).
---@return string?
---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-version
function project.version() end
