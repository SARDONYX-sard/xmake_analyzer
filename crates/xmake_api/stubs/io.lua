---@meta

--- The `io` module of Lua, extended by xmake.
---@class io
io = {}

--- Returns the contents of a file.
---
--- # Examples
---
--- ```lua
--- local version = io.readfile("VERSION"):trim()
--- ```
---@param path string
---@param opt? {encoding?: string}
---@return string
function io.readfile(path, opt) end

--- Writes a string to a file, replacing it.
---
--- # Examples
---
--- ```lua
--- io.writefile(path.join(target:autogendir(), "version.h"), "#define VERSION \"1.0\"\n")
--- ```
---@param path string
---@param data string
---@param opt? {encoding?: string}
function io.writefile(path, data, opt) end

--- Replaces text in a file, with Lua patterns (or `{plain = true}`).
---
--- # Examples
---
--- ```lua
--- io.replace("CMakeLists.txt", "add_subdirectory%(tests%)", "", {plain = false})
--- ```
---@param path string
---@param pattern string
---@param replacement string
---@param opt? {plain?: boolean}
function io.replace(path, pattern, replacement, opt) end

---@param path string
---@param mode? "r"|"w"|"a"|"r+"|"w+"|"a+"|"rb"|"wb"|"ab"|"r+b"|"w+b"|"a+b"
---@return file
function io.open(path, mode) end

---@param path string
---@return fun(): string
function io.lines(path) end

---@param path string
---@return table
function io.load(path) end

---@param path string
---@param data table
function io.save(path, data) end

---@param path string
function io.cat(path) end

---@param path string
---@param lines integer
function io.tail(path, lines) end

