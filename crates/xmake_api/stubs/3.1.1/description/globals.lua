---@meta

--- Prints its arguments; with a format string first, formats like `string.format`.
---
--- # Examples
---
--- ```lua
--- print("hello")
--- print("%s has %d files", target:name(), #target:sourcefiles())
--- print({a = 1}) -- tables are dumped
--- ```
---@param ... any Values, or a format and its arguments.
---@see https://xmake.io/api/scripts/builtin-modules/print.html
function print(...) end

--- Iterates over the array part of a table: `1, t[1]`, `2, t[2]`, ... up to
--- the first `nil`.
---
--- # Examples
---
--- ```lua
--- for i, file in ipairs(os.files("src/*.c")) do
---     print(i, file)
--- end
--- ```
---
--- # Notes
---
--- The order is the index order; use `pairs` for the other keys.
---@param t V[]
---@return fun(): integer, V
---@see https://xmake.io/api/scripts/builtin-modules/ipairs.html
function ipairs(t) end

--- Iterates over every key and value of a table, in no particular order.
---
--- # Examples
---
--- ```lua
--- for name, dep in pairs(target:deps()) do
---     print(name, dep:kind())
--- end
--- ```
---
--- # Notes
---
--- Sort the keys first when the order matters: `table.keys` then `table.sort`.
---@param t table
---@return fun(): any, any
---@see https://xmake.io/api/scripts/builtin-modules/pairs.html
function pairs(t) end

--- Returns the type of a value: `"nil"`, `"boolean"`, `"number"`, `"string"`,
--- `"table"`, `"function"`, `"thread"` or `"userdata"`.
---
--- # Examples
---
--- ```lua
--- if type(value) == "table" then
---     value = table.concat(value, " ")
--- end
--- ```
---@param value any
---@return string
function type(value) end

--- Converts a value to a string.
---
--- # Examples
---
--- ```lua
--- print("count: " .. tostring(#list))
--- ```
---@param value any
---@return string
function tostring(value) end

--- Converts a string to a number, `nil` when it is not one.
---
--- # Examples
---
--- ```lua
--- local jobs = tonumber(os.getenv("JOBS")) or 4
--- print(tonumber("ff", 16)) -- 255
--- ```
---@param value any
---@param base? integer
---@return number?
function tonumber(value, base) end

--- Returns the values of a list as separate values.
---
--- # Examples
---
--- ```lua
--- local args = {"--fast", "--quiet"}
--- os.execv(program, args)       -- a list
--- print(unpack(args))           -- separate values
--- ```
---@param list any[]
---@param i? integer
---@param j? integer
---@return any ...
function unpack(list, i, j) end

--- Formats values like `string.format` (xmake).
---
--- # Examples
---
--- ```lua
--- local name = format("%s-%s", target:name(), target:arch())
--- ```
---@param format string
---@param ... any
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/format.html
function format(format, ...) end

function add_subdirs() end

function add_subfiles() end

function getenv() end

---@param paths string|string[] Sub-project file or directory path, supports pattern matching like "**/xmake.lua"
---@param ... string|string[] Variable parameters, can pass multiple paths
---@param opts? {rootdir?: any}
---@see https://xmake.io/api/description/global-interfaces.html#includes
function includes(paths, ...) end

---@param subhost string Subsystem name string
---@param ... string Variable parameters, can pass multiple subsystem names
---@see https://xmake.io/api/description/conditions.html#is-subhost
function is_subhost(subhost, ...) end

---@param name string Namespace name string
---@param script fun() Namespace script function
---@see https://xmake.io/api/description/global-interfaces.html#namespace
function namespace(name, script) end

---@see https://xmake.io/api/description/global-interfaces.html#namespace-end
function namespace_end() end

---@see https://xmake.io/api/scripts/builtin-modules/printf.html
function printf() end

---@param version string Minimum required xmake version string, e.g. "2.1.0"
---@see https://xmake.io/api/description/global-interfaces.html#set-xmakever
function set_xmakever(version) end
