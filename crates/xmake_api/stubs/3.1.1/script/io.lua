---@meta

--- The `io` module of Lua, extended by xmake.
---@class io
---@see https://xmake.io/api/scripts/builtin-modules/io.html
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
---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-readfile
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
---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-writefile
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
---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-replace
function io.replace(path, pattern, replacement, opt) end

---@param path string
---@param mode? "r"|"w"|"a"|"r+"|"w+"|"a+"|"rb"|"wb"|"ab"|"r+b"|"w+b"|"a+b"
---@return file
---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-open
function io.open(path, mode) end

---@param path string
---@return fun(): string
---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-lines
function io.lines(path) end

---@param path string
---@return table
---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-load
function io.load(path) end

---@param path string
---@param data table
---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-save
function io.save(path, data) end

---@param path string
---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-cat
function io.cat(path) end

---@param path string
---@param lines integer
---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-tail
function io.tail(path, lines) end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-convert
function io.convert() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-flush
function io.flush() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-gsub
function io.gsub() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-insert
function io.insert() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-isatty
function io.isatty() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-openlock
function io.openlock() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-print
function io.print() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-printf
function io.printf() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-read
function io.read() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-readable
function io.readable() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-stdfile
function io.stdfile() end

---@see https://xmake.io/api/scripts/builtin-modules/io.html#io-write
function io.write() end
