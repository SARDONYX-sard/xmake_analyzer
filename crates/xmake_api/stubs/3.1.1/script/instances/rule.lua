---@meta

--- A rule.
---@class rule
---@see https://xmake.io/api/description/custom-rule.html#rule
rule = {}

---@return string
function rule:name() end

---@param name string
---@return any
function rule:get(name) end

function rule:add() end

function rule:clone() end

function rule:dep() end

function rule:deps() end

function rule:extraconf() end

function rule:extraconf_set() end

function rule:fullname() end

function rule:kind() end

function rule:name_set() end

function rule:namespace() end

function rule:orderdeps() end

function rule:script() end

function rule:set() end
