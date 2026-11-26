---@meta
---@diagnostic disable: unused-local

--- A set of values.
---
--- # Examples
---
--- ```lua
--- import("core.base.hashset")
--- local seen = hashset.new()
--- seen:insert("a")
--- print(seen:has("a"), seen:size())
--- ```
---@class core.base.hashset
---@see https://xmake.io/api/scripts/extension-modules/core/base/hashset.html
local hashset = {}

--- Creates an empty set.
---@return hashset
---@see https://xmake.io/api/scripts/extension-modules/core/base/hashset.html#hashset-new
function hashset.new() end

--- Creates a set of the arguments.
---@param ... any
---@return hashset
---@see https://xmake.io/api/scripts/extension-modules/core/base/hashset.html#hashset-of
function hashset.of(...) end

--- Creates a set of the values of a list.
---@param list any[]
---@return hashset
---@see https://xmake.io/api/scripts/extension-modules/core/base/hashset.html#hashset-from
function hashset.from(list) end

function hashset.clear() end

function hashset.clone() end

function hashset.data() end

function hashset.empty() end

function hashset.has() end

function hashset.insert() end

function hashset.items() end

function hashset.keys() end

function hashset.orderitems() end

function hashset.orderkeys() end

function hashset.remove() end

function hashset.size() end

function hashset.to_array() end
