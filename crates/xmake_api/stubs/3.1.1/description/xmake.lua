---@meta

--- Information about xmake itself.
---@class xmake
---@see https://xmake.io/api/scripts/extension-modules/package/tools.html#xmake
xmake = {}

--- The version of xmake.
---
--- # Examples
---
--- ```lua
--- if xmake.version():lt("2.8.0") then
---     raise("xmake 2.8.0 or newer is required")
--- end
--- ```
---@return semver
function xmake.version() end

--- The directory of the xmake program and its scripts.
---@return string
function xmake.programdir() end

--- The path of the xmake executable.
---@return string
function xmake.programfile() end

--- Whether xmake runs on LuaJIT.
---@return boolean
function xmake.luajit() end
