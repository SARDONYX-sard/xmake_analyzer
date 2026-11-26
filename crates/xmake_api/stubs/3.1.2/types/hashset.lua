---@meta
---@diagnostic disable: unused-local

--- A set, from `core.base.hashset`.
---@class hashset
hashset = {}

--- Adds a value; returns whether it was new.
---@param value any
---@return boolean
function hashset:insert(value) end

--- Whether the set has the value.
---@param value any
---@return boolean
function hashset:has(value) end

--- Removes a value; returns whether it was there.
---@param value any
---@return boolean
function hashset:remove(value) end

--- The number of values.
---@return integer
function hashset:size() end

--- Whether the set is empty.
---@return boolean
function hashset:empty() end

--- The values, as a list.
---@return any[]
function hashset:to_array() end

--- Iterates over the values.
---@return fun(): any
function hashset:items() end
