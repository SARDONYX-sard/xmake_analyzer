---@meta

--- A toolchain, given to the scripts of `toolchain(...)` definitions.
---@class toolchain
---@see https://xmake.io/api/description/custom-toolchain.html#toolchain
toolchain = {}

---@return string
function toolchain:name() end

---@return string
function toolchain:plat() end

---@return string
function toolchain:arch() end

---@param name string
---@return any
function toolchain:config(name) end

---@param name string
---@param value any
function toolchain:config_set(name, value) end

---@param name string
---@param ... any
function toolchain:add(name, ...) end

---@param name string
---@param ... any
function toolchain:set(name, ...) end

---@return string?
function toolchain:sdkdir() end

---@return string?
function toolchain:bindir() end

---@param ... string
---@return boolean
function toolchain:is_plat(...) end

function toolchain:arch_set() end

function toolchain:cachekey() end

function toolchain:check() end

function toolchain:check_cross_toolchain() end

function toolchain:configs_save() end

function toolchain:cross() end

function toolchain:formats() end

function toolchain:fullname() end

function toolchain:get() end

function toolchain:info() end

function toolchain:is_arch() end

function toolchain:is_builtin() end

function toolchain:is_cross() end

function toolchain:is_cross_toolchain() end

function toolchain:is_global() end

function toolchain:is_standalone() end

function toolchain:kind() end

function toolchain:load() end

function toolchain:load_cross_toolchain() end

function toolchain:memcache() end

function toolchain:namespace() end

function toolchain:new() end

function toolchain:packages() end

function toolchain:plat_set() end

function toolchain:runenvs() end

function toolchain:savefile() end

function toolchain:script() end

function toolchain:tool() end
