---@meta

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
---@return string
function project.directory() end

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

function project.modes() end

function project.mtimes() end

---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-name
function project.name() end

function project.namespaces() end

---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-option
function project.option() end

---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-options
function project.options() end

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

function project.rootfile() end

function project.rule() end

function project.rules() end

function project.scope() end

function project.target_add() end

function project.tmpdir() end

function project.tmpfile() end

function project.toolchain() end

function project.toolchains() end

function project.trylock() end

function project.unlock() end

---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-version
function project.version() end
