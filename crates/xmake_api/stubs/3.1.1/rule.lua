---@meta

--- A rule.
---@class rule
---@see https://xmake.io/api/description/custom-rule.html#rule
local rule = {}

---@return string
---@xmake.context instance
function rule:name() end

---@param name string
---@return any
---@xmake.context instance
function rule:get(name) end

---@xmake.context instance
function rule:add(...) end

---@xmake.context instance
function rule:clone(...) end

---@xmake.context instance
function rule:dep(...) end

---@xmake.context instance
function rule:deps(...) end

---@xmake.context instance
function rule:extraconf(...) end

---@xmake.context instance
function rule:extraconf_set(...) end

---@xmake.context instance
function rule:fullname(...) end

---@xmake.context instance
function rule:kind(...) end

---@xmake.context instance
function rule:name_set(...) end

---@xmake.context instance
function rule:namespace(...) end

---@xmake.context instance
function rule:orderdeps(...) end

---@xmake.context instance
function rule:script(...) end

---@xmake.context instance
function rule:set(...) end

