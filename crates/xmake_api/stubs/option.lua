---@meta

--- An option, given to the scripts of `option(...)` definitions.
---@class option
local option = {}

---@return string
function option:name() end

--- Whether the option is enabled.
---
--- # Examples
---
--- ```lua
--- on_check(function (option)
---     print(option:enabled())
--- end)
--- ```
---@return boolean
function option:enabled() end

--- Enables or disables the option.
---
--- # Examples
---
--- ```lua
--- on_check(function (option)
---     option:enable(os.isfile("/usr/include/openssl/ssl.h"))
--- end)
--- ```
---@param enabled boolean
function option:enable(enabled) end

--- Returns the value of the option.
---
--- # Examples
---
--- ```lua
--- on_check(function (option)
---     print(option:value())
--- end)
--- ```
---@return any
function option:value() end

---@param value any
function option:set_value(value) end

---@param name string
---@return any
function option:get(name) end

---@param name string
---@param ... any
function option:add(name, ...) end

