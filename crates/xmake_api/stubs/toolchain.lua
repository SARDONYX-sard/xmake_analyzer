---@meta

--- A toolchain, given to the scripts of `toolchain(...)` definitions.
---@class toolchain
local toolchain = {}

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

