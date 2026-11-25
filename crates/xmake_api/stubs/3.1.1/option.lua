---@meta

--- An option, given to the scripts of `option(...)` definitions.
---@class option
---@see https://xmake.io/api/description/configuration-option.html#option
local option = {}

---@return string
---@see https://xmake.io/api/scripts/option-instance.html#option-name
---@xmake.context instance
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
---@see https://xmake.io/api/scripts/option-instance.html#option-enabled
---@xmake.context instance
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
---@see https://xmake.io/api/scripts/option-instance.html#option-enable
---@xmake.context instance
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
---@see https://xmake.io/api/scripts/option-instance.html#option-value
---@xmake.context instance
function option:value() end

---@param value any
---@see https://xmake.io/api/scripts/option-instance.html#option-set-value
---@xmake.context instance
function option:set_value(value) end

---@param name string
---@return any
---@see https://xmake.io/api/scripts/option-instance.html#option-get
---@xmake.context instance
function option:get(name) end

---@param name string
---@param ... any
---@see https://xmake.io/api/scripts/option-instance.html#option-add
---@xmake.context instance
function option:add(name, ...) end

---@xmake.context instance
function option:cachekey(...) end

---@xmake.context instance
function option:check(...) end

---@see https://xmake.io/api/scripts/option-instance.html#option-clear
---@xmake.context instance
function option:clear(...) end

---@xmake.context instance
function option:del(...) end

---@see https://xmake.io/api/scripts/option-instance.html#option-dep
---@xmake.context instance
function option:dep(...) end

---@see https://xmake.io/api/scripts/option-instance.html#option-deps
---@xmake.context instance
function option:deps(...) end

---@see https://xmake.io/api/scripts/option-instance.html#option-description
---@xmake.context instance
function option:description(...) end

---@see https://xmake.io/api/scripts/option-instance.html#option-extraconf
---@xmake.context instance
function option:extraconf(...) end

---@see https://xmake.io/api/scripts/option-instance.html#option-fullname
---@xmake.context instance
function option:fullname(...) end

---@xmake.context instance
function option:info(...) end

---@see https://xmake.io/api/scripts/option-instance.html#option-namespace
---@xmake.context instance
function option:namespace(...) end

---@xmake.context instance
function option:new(...) end

---@see https://xmake.io/api/scripts/option-instance.html#option-orderdeps
---@xmake.context instance
function option:orderdeps(...) end

---@see https://xmake.io/api/scripts/option-instance.html#option-remove
---@xmake.context instance
function option:remove(...) end

---@xmake.context instance
function option:script(...) end

---@see https://xmake.io/api/scripts/option-instance.html#option-set
---@xmake.context instance
function option:set(...) end

---@xmake.context instance
function option:showmenu(...) end

---@xmake.context instance
function option:sourceinfo(...) end

---@xmake.context instance
function option:type(...) end

