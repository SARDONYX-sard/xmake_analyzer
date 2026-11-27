---@meta
---@diagnostic disable: unused-local

--- The configuration of the project (`xmake f`).
---@class core.project.config
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html
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
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-get
function config.get(name) end

--- Sets a configuration value.
---@param name string
---@param value any
---@param opt? {readonly?: boolean, force?: boolean}
function config.set(name, value, opt) end

--- Loads the configuration of the project.
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-load
function config.load() end

--- Returns the build directory.
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-builddir
function config.builddir() end

--- Returns the platform being configured.
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-plat
function config.plat() end

--- Returns the architecture being configured.
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-arch
function config.arch() end

--- Returns the build mode being configured.
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-mode
function config.mode() end

--- Returns the build directory (deprecated: `config.builddir`).
---@return string
function config.buildir() end

function config.clear() end

--- Returns the directory of the configuration (`.xmake/<host>/<arch>`).
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-directory
function config.directory() end

---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-dump
function config.dump() end

--- Returns the path of the configuration file (`xmake.conf`).
---@return string
function config.filepath() end

--- Returns the host platform.
---@return string
function config.host() end

function config.read() end

--- Whether a configuration value is read-only.
---@param name string
---@return boolean
function config.readonly(name) end

function config.save() end
