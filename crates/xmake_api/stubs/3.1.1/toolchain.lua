---@meta

--- A toolchain, given to the scripts of `toolchain(...)` definitions.
---@class toolchain
---@see https://xmake.io/api/description/custom-toolchain.html#toolchain
local toolchain = {}

---@return string
---@xmake.context instance
function toolchain:name() end

---@return string
---@xmake.context instance
function toolchain:plat() end

---@return string
---@xmake.context instance
function toolchain:arch() end

---@param name string
---@return any
---@xmake.context instance
function toolchain:config(name) end

---@param name string
---@param value any
---@xmake.context instance
function toolchain:config_set(name, value) end

---@param name string
---@param ... any
---@xmake.context instance
function toolchain:add(name, ...) end

---@param name string
---@param ... any
---@xmake.context instance
function toolchain:set(name, ...) end

---@return string?
---@xmake.context instance
function toolchain:sdkdir() end

---@return string?
---@xmake.context instance
function toolchain:bindir() end

---@param ... string
---@return boolean
---@xmake.context instance
function toolchain:is_plat(...) end

---@xmake.context instance
function toolchain:arch_set(...) end

---@xmake.context instance
function toolchain:cachekey(...) end

---@xmake.context instance
function toolchain:check(...) end

---@xmake.context instance
function toolchain:check_cross_toolchain(...) end

---@xmake.context instance
function toolchain:configs_save(...) end

---@xmake.context instance
function toolchain:cross(...) end

---@xmake.context instance
function toolchain:formats(...) end

---@xmake.context instance
function toolchain:fullname(...) end

---@xmake.context instance
function toolchain:get(...) end

---@xmake.context instance
function toolchain:info(...) end

---@xmake.context instance
function toolchain:is_arch(...) end

---@xmake.context instance
function toolchain:is_builtin(...) end

---@xmake.context instance
function toolchain:is_cross(...) end

---@xmake.context instance
function toolchain:is_cross_toolchain(...) end

---@xmake.context instance
function toolchain:is_global(...) end

---@xmake.context instance
function toolchain:is_standalone(...) end

---@xmake.context instance
function toolchain:kind(...) end

---@xmake.context instance
function toolchain:load(...) end

---@xmake.context instance
function toolchain:load_cross_toolchain(...) end

---@xmake.context instance
function toolchain:memcache(...) end

---@xmake.context instance
function toolchain:namespace(...) end

---@xmake.context instance
function toolchain:new(...) end

---@xmake.context instance
function toolchain:packages(...) end

---@xmake.context instance
function toolchain:plat_set(...) end

---@xmake.context instance
function toolchain:runenvs(...) end

---@xmake.context instance
function toolchain:savefile(...) end

---@xmake.context instance
function toolchain:script(...) end

---@xmake.context instance
function toolchain:tool(...) end

