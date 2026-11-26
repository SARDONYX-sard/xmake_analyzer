---@meta

--- The global configuration of xmake (`xmake g`).
---@class core.base.global
---@see https://xmake.io/api/scripts/extension-modules/core/base/global.html
local global = {}

--- Returns a global configuration value.
---@param name string
---@return any
---@see https://xmake.io/api/scripts/extension-modules/core/base/global.html#global-get
function global.get(name) end

function global.cachedir() end

function global.clear() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/global.html#global-directory
function global.directory() end

---@see https://xmake.io/api/scripts/extension-modules/core/base/global.html#global-dump
function global.dump() end

function global.filepath() end

function global.options() end

function global.readonly() end

function global.save() end

function global.set() end
