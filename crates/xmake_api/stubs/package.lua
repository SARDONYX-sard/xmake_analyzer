---@meta

--- A package, given to the scripts of `package(...)` definitions.
---@class package
local package = {}

---@return string
function package:name() end

--- Returns the install directory of the package, or one of its subdirectories.
---
--- # Examples
---
--- ```lua
--- on_install(function (package)
---     os.cp("include/*.h", package:installdir("include"))
---     os.cp("lib/*.a", package:installdir("lib"))
--- end)
--- ```
---@param ... string Subdirectories.
---@return string
function package:installdir(...) end

--- Returns the version of the package, a semver object to compare.
---
--- # Examples
---
--- ```lua
--- if package:version():ge("2.0") then
---     package:add("defines", "MYLIB_V2")
--- end
--- ```
---@return semver
function package:version() end

--- Returns the version of the package as a string.
---
--- # Examples
---
--- ```lua
--- print(package:version_str()) -- 1.2.13
--- ```
---@return string
function package:version_str() end

--- Returns a configuration of the package, as set by `add_requires(..., {configs = {...}})`.
---
--- # Examples
---
--- ```lua
--- on_install(function (package)
---     local configs = {"-DBUILD_SHARED_LIBS=" .. (package:config("shared") and "ON" or "OFF")}
---     import("package.tools.cmake").install(package, configs)
--- end)
--- ```
---@param name string
---@return any
function package:config(name) end

--- Adds values to a setting of the package for its users: `links`, `defines`, ...
---
--- # Examples
---
--- ```lua
--- on_load(function (package)
---     if not package:config("shared") then
---         package:add("defines", "MYLIB_STATIC")
---     end
--- end)
--- ```
---@param name string
---@param ... any
function package:add(name, ...) end

--- Whether the platform of the package is one of the arguments.
---
--- # Examples
---
--- ```lua
--- if package:is_plat("windows") then
---     package:add("syslinks", "advapi32")
--- end
--- ```
---@param ... string
---@return boolean
function package:is_plat(...) end

---@param ... string
---@return boolean
function package:is_arch(...) end

---@return string
function package:plat() end

---@return string
function package:arch() end

---@return boolean
function package:debug() end

---@param name string
---@return package?
function package:dep(name) end

---@return string
function package:cachedir() end

---@return string
function package:sourcedir() end

