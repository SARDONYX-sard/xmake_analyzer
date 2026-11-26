---@meta
---@diagnostic disable: unused-local

--- Table functions of Lua, extended by xmake.
---@class table
---@see https://xmake.io/api/scripts/builtin-modules/table.html
table = {}

--- Inserts a value, at the end or at `pos`.
---@param t table
---@param value any
function table.insert(t, value) end

--- Removes and returns the value at `pos` (the last one by default).
---@param t table
---@param pos? integer
---@return any
function table.remove(t, pos) end

--- Joins the strings of a list.
---@param t string[]
---@param separator? string
---@return string
function table.concat(t, separator) end

--- Sorts a list in place.
---@param t table
---@param less? fun(a: any, b: any): boolean
function table.sort(t, less) end

--- Joins tables into a new list (xmake).
---
--- # Examples
---
--- ```lua
--- local all = table.join({1, 2}, {3}, 4) -- {1, 2, 3, 4}
--- ```
---@param ... any
---@return table
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-join
function table.join(...) end

--- Appends the values of the other tables to the first one (xmake).
---@param t table
---@param ... any
---@return table
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-join2
function table.join2(t, ...) end

--- Whether a list contains any of the values (xmake).
---@param t table
---@param ... any
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-contains
function table.contains(t, ...) end

--- Returns the list without duplicates (xmake).
---@param t table
---@return table
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-unique
function table.unique(t) end

--- Wraps a value in a list unless it is one (xmake).
---@param value any
---@return table
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-wrap
function table.wrap(value) end

--- Returns the keys of a table (xmake).
---@param t table
---@return any[]
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-keys
function table.keys(t) end

--- Whether a table is empty (xmake).
---@param t table
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-empty
function table.empty(t) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-append
function table.append() end

function table.clear() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-clone
function table.clone() end

function table.copy() end

function table.copy2() end

function table.create() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-find
function table.find() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-find-first
function table.find_first() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-find-first-if
function table.find_first_if() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-find-if
function table.find_if() end

function table.getn() end

function table.imap() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-inherit
function table.inherit() end

function table.inherit2() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-is-array
function table.is_array() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-is-dictionary
function table.is_dictionary() end

function table.map() end

function table.maxn() end

function table.move() end

function table.new() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-orderkeys
function table.orderkeys() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-orderpairs
function table.orderpairs() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-pack
function table.pack() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-remove-if
function table.remove_if() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-reverse
function table.reverse() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-reverse-unique
function table.reverse_unique() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-shallow-join
function table.shallow_join() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-shallow-join2
function table.shallow_join2() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-slice
function table.slice() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-swap
function table.swap() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-to-array
function table.to_array() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-unpack
function table.unpack() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-unwrap
function table.unwrap() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-values
function table.values() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-wrap-lock
function table.wrap_lock() end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-wrap-unlock
function table.wrap_unlock() end
