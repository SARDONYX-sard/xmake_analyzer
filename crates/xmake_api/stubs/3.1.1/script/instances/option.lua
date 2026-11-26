---@meta
---@diagnostic disable: unused-local

--- An option, given to the scripts of `option(...)` definitions.
---@class option
---@see https://xmake.io/api/description/configuration-option.html#option
option = {}

---@return string
---@see https://xmake.io/api/scripts/option-instance.html#option-name
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
function option:value() end

---@param value any
---@see https://xmake.io/api/scripts/option-instance.html#option-set-value
function option:set_value(value) end

---@param name string
---@return any
---@see https://xmake.io/api/scripts/option-instance.html#option-get
function option:get(name) end

---@param name string
---@param ... any
---@see https://xmake.io/api/scripts/option-instance.html#option-add
function option:add(name, ...) end

function option:cachekey() end

function option:check() end

---@see https://xmake.io/api/scripts/option-instance.html#option-clear
function option:clear() end

function option:del() end

---@see https://xmake.io/api/scripts/option-instance.html#option-dep
function option:dep() end

---@see https://xmake.io/api/scripts/option-instance.html#option-deps
function option:deps() end

---@see https://xmake.io/api/scripts/option-instance.html#option-description
function option:description() end

---@see https://xmake.io/api/scripts/option-instance.html#option-extraconf
function option:extraconf() end

---@see https://xmake.io/api/scripts/option-instance.html#option-fullname
function option:fullname() end

function option:info() end

---@see https://xmake.io/api/scripts/option-instance.html#option-namespace
function option:namespace() end

function option:new() end

---@see https://xmake.io/api/scripts/option-instance.html#option-orderdeps
function option:orderdeps() end

---@see https://xmake.io/api/scripts/option-instance.html#option-remove
function option:remove() end

function option:script() end

---@see https://xmake.io/api/scripts/option-instance.html#option-set
function option:set() end

function option:showmenu() end

function option:sourceinfo() end

function option:type() end
