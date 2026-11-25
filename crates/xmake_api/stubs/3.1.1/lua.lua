---@meta

--- String functions of Lua, extended by xmake. Also methods of strings:
--- `s:split(",")`.
---@class string
---@see https://xmake.io/api/scripts/builtin-modules/string.html
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
---@xmake.context description script
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
---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-split
---@xmake.context description script
function string.split(s, separator, opt) end

--- Removes the blanks at both ends (xmake).
---@param s string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-trim
---@xmake.context description script
function string.trim(s) end

--- Whether the string starts with `prefix` (xmake).
---@param s string
---@param prefix string
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-startswith
---@xmake.context description script
function string.startswith(s, prefix) end

--- Whether the string ends with `suffix` (xmake).
---@param s string
---@param suffix string
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-endswith
---@xmake.context description script
function string.endswith(s, suffix) end

--- Replaces matches of a pattern (or a plain string with `{plain = true}`, xmake).
---@param s string
---@param pattern string
---@param replacement string
---@param opt? {plain?: boolean}
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-replace
---@xmake.context description script
function string.replace(s, pattern, replacement, opt) end

--- Returns the string in lower case.
---@param s string
---@return string
---@xmake.context description script
function string.lower(s) end

--- Returns the string in upper case.
---@param s string
---@return string
---@xmake.context description script
function string.upper(s) end

--- Finds a pattern, returns the start and end indexes.
---@param s string
---@param pattern string
---@param init? integer
---@param plain? boolean
---@return integer? start
---@return integer? end
---@xmake.context description script
function string.find(s, pattern, init, plain) end

--- Returns the captures of the first match of a pattern.
---@param s string
---@param pattern string
---@return string?
---@xmake.context description script
function string.match(s, pattern) end

--- Iterates over the matches of a pattern.
---@param s string
---@param pattern string
---@return fun(): string
---@xmake.context description script
function string.gmatch(s, pattern) end

--- Replaces the matches of a pattern.
---@param s string
---@param pattern string
---@param replacement string|table|function
---@return string
---@return integer count
---@xmake.context description script
function string.gsub(s, pattern, replacement) end

--- Returns a substring.
---@param s string
---@param i integer
---@param j? integer
---@return string
---@xmake.context description script
function string.sub(s, i, j) end

--- Returns the length.
---@param s string
---@return integer
---@xmake.context description script
function string.len(s) end

---@xmake.context description script
function string.byte(...) end

---@xmake.context description script
function string.char(...) end

---@xmake.context description script
function string.convert(...) end

---@xmake.context description script
function string.decode(...) end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-deserialize
---@xmake.context description script
function string.deserialize(...) end

---@xmake.context description script
function string.dump(...) end

---@xmake.context description script
function string.encode(...) end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-ipattern
---@xmake.context description script
function string.ipattern(...) end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-lastof
---@xmake.context description script
function string.lastof(...) end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-levenshtein
---@xmake.context description script
function string.levenshtein(...) end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-ltrim
---@xmake.context description script
function string.ltrim(...) end

---@xmake.context description script
function string.pack(...) end

---@xmake.context description script
function string.packsize(...) end

---@xmake.context description script
function string.rep(...) end

---@xmake.context description script
function string.reverse(...) end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-rtrim
---@xmake.context description script
function string.rtrim(...) end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-serialize
---@xmake.context description script
function string.serialize(...) end

---@xmake.context description script
function string.tryformat(...) end

---@xmake.context description script
function string.unpack(...) end

---@xmake.context description script
function string.vformat(...) end

---@xmake.context description script
function string.wcswidth(...) end

---@xmake.context description script
function string.wcwidth(...) end

--- Table functions of Lua, extended by xmake.
---@class table
---@see https://xmake.io/api/scripts/builtin-modules/table.html
table = {}

--- Inserts a value, at the end or at `pos`.
---@param t table
---@param value any
---@xmake.context description script
function table.insert(t, value) end

--- Removes and returns the value at `pos` (the last one by default).
---@param t table
---@param pos? integer
---@return any
---@xmake.context description script
function table.remove(t, pos) end

--- Joins the strings of a list.
---@param t string[]
---@param separator? string
---@return string
---@xmake.context description script
function table.concat(t, separator) end

--- Sorts a list in place.
---@param t table
---@param less? fun(a: any, b: any): boolean
---@xmake.context description script
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
---@xmake.context description script
function table.join(...) end

--- Appends the values of the other tables to the first one (xmake).
---@param t table
---@param ... any
---@return table
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-join2
---@xmake.context description script
function table.join2(t, ...) end

--- Whether a list contains any of the values (xmake).
---@param t table
---@param ... any
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-contains
---@xmake.context description script
function table.contains(t, ...) end

--- Returns the list without duplicates (xmake).
---@param t table
---@return table
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-unique
---@xmake.context description script
function table.unique(t) end

--- Wraps a value in a list unless it is one (xmake).
---@param value any
---@return table
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-wrap
---@xmake.context description script
function table.wrap(value) end

--- Returns the keys of a table (xmake).
---@param t table
---@return any[]
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-keys
---@xmake.context description script
function table.keys(t) end

--- Whether a table is empty (xmake).
---@param t table
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-empty
---@xmake.context description script
function table.empty(t) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-append
---@xmake.context description script
function table.append(...) end

---@xmake.context description script
function table.clear(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-clone
---@xmake.context description script
function table.clone(...) end

---@xmake.context description script
function table.copy(...) end

---@xmake.context description script
function table.copy2(...) end

---@xmake.context description script
function table.create(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-find
---@xmake.context description script
function table.find(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-find-first
---@xmake.context description script
function table.find_first(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-find-first-if
---@xmake.context description script
function table.find_first_if(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-find-if
---@xmake.context description script
function table.find_if(...) end

---@xmake.context description script
function table.getn(...) end

---@xmake.context description script
function table.imap(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-inherit
---@xmake.context description script
function table.inherit(...) end

---@xmake.context description script
function table.inherit2(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-is-array
---@xmake.context description script
function table.is_array(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-is-dictionary
---@xmake.context description script
function table.is_dictionary(...) end

---@xmake.context description script
function table.map(...) end

---@xmake.context description script
function table.maxn(...) end

---@xmake.context description script
function table.move(...) end

---@xmake.context description script
function table.new(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-orderkeys
---@xmake.context description script
function table.orderkeys(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-orderpairs
---@xmake.context description script
function table.orderpairs(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-pack
---@xmake.context description script
function table.pack(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-remove-if
---@xmake.context description script
function table.remove_if(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-reverse
---@xmake.context description script
function table.reverse(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-reverse-unique
---@xmake.context description script
function table.reverse_unique(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-shallow-join
---@xmake.context description script
function table.shallow_join(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-shallow-join2
---@xmake.context description script
function table.shallow_join2(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-slice
---@xmake.context description script
function table.slice(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-swap
---@xmake.context description script
function table.swap(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-to-array
---@xmake.context description script
function table.to_array(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-unpack
---@xmake.context description script
function table.unpack(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-unwrap
---@xmake.context description script
function table.unwrap(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-values
---@xmake.context description script
function table.values(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-wrap-lock
---@xmake.context description script
function table.wrap_lock(...) end

---@see https://xmake.io/api/scripts/builtin-modules/table.html#table-wrap-unlock
---@xmake.context description script
function table.wrap_unlock(...) end

--- Math functions of Lua.
---@class math
math = {}

--- Rounds down.
---@param x number
---@return integer
---@xmake.context description script
function math.floor(x) end

--- Rounds up.
---@param x number
---@return integer
---@xmake.context description script
function math.ceil(x) end

--- The largest of the arguments.
---@param x number
---@param ... number
---@return number
---@xmake.context description script
function math.max(x, ...) end

--- The smallest of the arguments.
---@param x number
---@param ... number
---@return number
---@xmake.context description script
function math.min(x, ...) end

--- The absolute value.
---@param x number
---@return number
---@xmake.context description script
function math.abs(x) end

--- The square root.
---@param x number
---@return number
---@xmake.context description script
function math.sqrt(x) end

--- A random number: in `[0, 1)` without arguments, in `[1, m]` or `[m, n]` otherwise.
---@param m? integer
---@param n? integer
---@return number
---@xmake.context description script
function math.random(m, n) end

--- Converts to an integer when the value is integral (Lua 5.3+).
---@param x number
---@return integer?
---@xmake.context description script
function math.tointeger(x) end

---@xmake.context description script
function math.acos(...) end

---@xmake.context description script
function math.asin(...) end

---@xmake.context description script
function math.atan(...) end

---@xmake.context description script
function math.cos(...) end

---@xmake.context description script
function math.deg(...) end

---@xmake.context description script
function math.exp(...) end

---@xmake.context description script
function math.fmod(...) end

---@xmake.context description script
function math.frexp(...) end

---@xmake.context description script
function math.isinf(...) end

---@xmake.context description script
function math.isint(...) end

---@xmake.context description script
function math.isnan(...) end

---@xmake.context description script
function math.ldexp(...) end

---@xmake.context description script
function math.log(...) end

---@xmake.context description script
function math.modf(...) end

---@xmake.context description script
function math.rad(...) end

---@xmake.context description script
function math.randomseed(...) end

---@xmake.context description script
function math.sin(...) end

---@xmake.context description script
function math.tan(...) end

---@xmake.context description script
function math.type(...) end

---@xmake.context description script
function math.ult(...) end

--- Information about xmake itself.
---@class xmake
---@see https://xmake.io/api/scripts/extension-modules/package/tools.html#xmake
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
---@xmake.context description script
function xmake.version() end

--- The directory of the xmake program and its scripts.
---@return string
---@xmake.context description script
function xmake.programdir() end

--- The path of the xmake executable.
---@return string
---@xmake.context description script
function xmake.programfile() end

--- Whether xmake runs on LuaJIT.
---@return boolean
---@xmake.context description script
function xmake.luajit() end

---@xmake.context script
function xmake.arch(...) end

---@xmake.context script
function xmake.argv(...) end

---@xmake.context script
function xmake.branch(...) end

---@xmake.context script
function xmake.in_main_thread(...) end

---@xmake.context script
function xmake.is_embed(...) end

--- Hashes of files and strings (xmake).
---@class hash
---@see https://xmake.io/api/scripts/builtin-modules/hash.html
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
---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-sha256
---@xmake.context description script
function hash.sha256(file) end

--- The SHA-1 of a file, in hexadecimal.
---@param file string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-sha1
---@xmake.context description script
function hash.sha1(file) end

--- The MD5 of a file, in hexadecimal.
---@param file string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-md5
---@xmake.context description script
function hash.md5(file) end

--- A random UUID, or one derived from `name`.
---@param name? string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-uuid
---@xmake.context description script
function hash.uuid(name) end

--- A 32 bits hash of a string, in hexadecimal.
---@param s string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-strhash32
---@xmake.context description script
function hash.strhash32(s) end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-rand128
---@xmake.context description script
function hash.rand128(...) end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-rand32
---@xmake.context description script
function hash.rand32(...) end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-rand64
---@xmake.context description script
function hash.rand64(...) end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-strhash128
---@xmake.context description script
function hash.strhash128(...) end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-strhash64
---@xmake.context description script
function hash.strhash64(...) end

---@xmake.context description script
function hash.uuid4(...) end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-xxhash128
---@xmake.context description script
function hash.xxhash128(...) end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-xxhash32
---@xmake.context description script
function hash.xxhash32(...) end

---@see https://xmake.io/api/scripts/builtin-modules/hash.html#hash-xxhash64
---@xmake.context description script
function hash.xxhash64(...) end
