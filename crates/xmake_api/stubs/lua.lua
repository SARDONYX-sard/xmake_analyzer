---@meta

--- String functions of Lua, extended by xmake. Also methods of strings:
--- `s:split(",")`.
---@class string
string = {}

--- Formats values like C `printf`.
---
--- # Examples
---
--- ```lua
--- local s = string.format("%s-%d", "app", 2) -- app-2
--- ```
---@param format string
---@param ... any
---@return string
function string.format(format, ...) end

--- Splits a string by a separator (xmake).
---
--- # Examples
---
--- ```lua
--- local parts = ("a,b,,c"):split(",") -- {"a", "b", "c"}
--- local all = ("a,b,,c"):split(",", {strict = true}) -- {"a", "b", "", "c"}
--- ```
---@param s string
---@param separator string
---@param opt? {plain?: boolean, strict?: boolean, limit?: integer}
---@return string[]
function string.split(s, separator, opt) end

--- Removes the blanks at both ends (xmake).
---@param s string
---@return string
function string.trim(s) end

--- Whether the string starts with `prefix` (xmake).
---@param s string
---@param prefix string
---@return boolean
function string.startswith(s, prefix) end

--- Whether the string ends with `suffix` (xmake).
---@param s string
---@param suffix string
---@return boolean
function string.endswith(s, suffix) end

--- Replaces matches of a pattern (or a plain string with `{plain = true}`, xmake).
---@param s string
---@param pattern string
---@param replacement string
---@param opt? {plain?: boolean}
---@return string
function string.replace(s, pattern, replacement, opt) end

--- Returns the string in lower case.
---@param s string
---@return string
function string.lower(s) end

--- Returns the string in upper case.
---@param s string
---@return string
function string.upper(s) end

--- Finds a pattern, returns the start and end indexes.
---@param s string
---@param pattern string
---@param init? integer
---@param plain? boolean
---@return integer? start
---@return integer? end
function string.find(s, pattern, init, plain) end

--- Returns the captures of the first match of a pattern.
---@param s string
---@param pattern string
---@return string?
function string.match(s, pattern) end

--- Iterates over the matches of a pattern.
---@param s string
---@param pattern string
---@return fun(): string
function string.gmatch(s, pattern) end

--- Replaces the matches of a pattern.
---@param s string
---@param pattern string
---@param replacement string|table|function
---@return string
---@return integer count
function string.gsub(s, pattern, replacement) end

--- Returns a substring.
---@param s string
---@param i integer
---@param j? integer
---@return string
function string.sub(s, i, j) end

--- Returns the length.
---@param s string
---@return integer
function string.len(s) end

--- Table functions of Lua, extended by xmake.
---@class table
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
function table.join(...) end

--- Appends the values of the other tables to the first one (xmake).
---@param t table
---@param ... any
---@return table
function table.join2(t, ...) end

--- Whether a list contains any of the values (xmake).
---@param t table
---@param ... any
---@return boolean
function table.contains(t, ...) end

--- Returns the list without duplicates (xmake).
---@param t table
---@return table
function table.unique(t) end

--- Wraps a value in a list unless it is one (xmake).
---@param value any
---@return table
function table.wrap(value) end

--- Returns the keys of a table (xmake).
---@param t table
---@return any[]
function table.keys(t) end

--- Whether a table is empty (xmake).
---@param t table
---@return boolean
function table.empty(t) end

--- Math functions of Lua.
---@class math
math = {}

--- Rounds down.
---@param x number
---@return integer
function math.floor(x) end

--- Rounds up.
---@param x number
---@return integer
function math.ceil(x) end

--- The largest of the arguments.
---@param x number
---@param ... number
---@return number
function math.max(x, ...) end

--- The smallest of the arguments.
---@param x number
---@param ... number
---@return number
function math.min(x, ...) end

--- The absolute value.
---@param x number
---@return number
function math.abs(x) end

--- The square root.
---@param x number
---@return number
function math.sqrt(x) end

--- A random number: in `[0, 1)` without arguments, in `[1, m]` or `[m, n]` otherwise.
---@param m? integer
---@param n? integer
---@return number
function math.random(m, n) end

--- Converts to an integer when the value is integral (Lua 5.3+).
---@param x number
---@return integer?
function math.tointeger(x) end

--- Information about xmake itself.
---@class xmake
xmake = {}

--- The version of xmake.
---
--- # Examples
---
--- ```lua
--- if xmake.version():lt("2.8.0") then
---     raise("xmake 2.8.0 or newer is required")
--- end
--- ```
---@return semver
function xmake.version() end

--- The directory of the xmake program and its scripts.
---@return string
function xmake.programdir() end

--- The path of the xmake executable.
---@return string
function xmake.programfile() end

--- Whether xmake runs on LuaJIT.
---@return boolean
function xmake.luajit() end

--- Hashes of files and strings (xmake).
---@class hash
hash = {}

--- The SHA-256 of a file, in lower case hexadecimal.
---
--- # Examples
---
--- ```lua
--- local sum = hash.sha256("sdk.zip")
--- assert(sum == expected:lower(), "checksum mismatch")
--- ```
---@param file string
---@return string
function hash.sha256(file) end

--- The SHA-1 of a file, in hexadecimal.
---@param file string
---@return string
function hash.sha1(file) end

--- The MD5 of a file, in hexadecimal.
---@param file string
---@return string
function hash.md5(file) end

--- A random UUID, or one derived from `name`.
---@param name? string
---@return string
function hash.uuid(name) end

--- A 32 bits hash of a string, in hexadecimal.
---@param s string
---@return string
function hash.strhash32(s) end
