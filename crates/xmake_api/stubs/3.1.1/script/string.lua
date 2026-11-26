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
function string.split(s, separator, opt) end

--- Removes the blanks at both ends (xmake).
---@param s string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-trim
function string.trim(s) end

--- Whether the string starts with `prefix` (xmake).
---@param s string
---@param prefix string
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-startswith
function string.startswith(s, prefix) end

--- Whether the string ends with `suffix` (xmake).
---@param s string
---@param suffix string
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-endswith
function string.endswith(s, suffix) end

--- Replaces matches of a pattern (or a plain string with `{plain = true}`, xmake).
---@param s string
---@param pattern string
---@param replacement string
---@param opt? {plain?: boolean}
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-replace
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

function string.byte() end

function string.char() end

function string.convert() end

function string.decode() end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-deserialize
function string.deserialize() end

function string.dump() end

function string.encode() end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-ipattern
function string.ipattern() end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-lastof
function string.lastof() end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-levenshtein
function string.levenshtein() end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-ltrim
function string.ltrim() end

function string.pack() end

function string.packsize() end

function string.rep() end

function string.reverse() end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-rtrim
function string.rtrim() end

---@see https://xmake.io/api/scripts/builtin-modules/string.html#string-serialize
function string.serialize() end

function string.tryformat() end

function string.unpack() end

function string.vformat() end

function string.wcswidth() end

function string.wcwidth() end
