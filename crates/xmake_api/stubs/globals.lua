---@meta

--- Prints with colors: `${color}` markers, reset at the end of the line.
---
--- # Examples
---
--- ```lua
--- cprint("${green}ok${clear}: %s", target:name())
--- cprint("${bright red}error${clear}: something failed")
--- ```
---@param format string With `${color}` markers.
---@param ... any
function cprint(format, ...) end

--- Imports an extension module and returns it. In a function body, the module
--- is also bound to the last part of its name.
---
--- # Examples
---
--- ```lua
--- on_build(function (target)
---     import("core.base.json")
---     import("lib.detect.find_tool")
---     import("utils.archive", {alias = "ar"})
---
---     local data = json.decode(io.readfile("data.json"))
---     local git = find_tool("git")
---     ar.extract("pkg.zip", "out")
--- end)
--- ```
---
--- # Notes
---
--- - Only in scripts: description code (`xmake.lua` itself) has no `import`.
--- - Modules next to the script are found by name: `import("modules.utils")`
---   loads `modules/utils.lua`.
---@param name string The module: `core.base.json`, `lib.detect.find_tool`, or a path next to the script.
---@param opt? {alias?: string, rootdir?: string, try?: boolean, anonymous?: boolean}
---@return any module
function import(name, opt) end

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
function print(...) end

--- Stops the script with an error message, formatted like `string.format`.
---
--- # Examples
---
--- ```lua
--- on_load(function (target)
---     if not os.isfile("config.json") then
---         raise("config.json not found in %s", os.curdir())
---     end
--- end)
--- ```
---
--- # Notes
---
--- Catch errors with `try { function () ... end, catch { function (errors) ... end } }`.
---@param format string A `string.format` format, or the error value.
---@param ... any
function raise(format, ...) end

--- Runs a function and catches its errors, with optional `catch` and `finally` blocks.
---
--- # Examples
---
--- ```lua
--- local ok = try {
---     function ()
---         os.vrunv("git", {"pull"})
---         return true
---     end,
---     catch {
---         function (errors)
---             print("git pull failed: %s", errors)
---         end
---     },
---     finally {
---         function (ok, errors)
---             print("done")
---         end
---     }
--- }
--- ```
---@param block {[1]: function, catch?: table, finally?: table}
---@return any
function try(block) end

--- Prints only in verbose mode (`xmake -v`).
---
--- # Examples
---
--- ```lua
--- vprint("compiling %s", sourcefile)
--- ```
---@param ... any
function vprint(...) end

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
---@generic V
---@param t V[]
---@return fun(): integer, V
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
function pairs(t) end

--- Returns the next key and value of a table after `key`; `nil` at the end.
---
--- # Examples
---
--- ```lua
--- local empty = next(t) == nil
--- ```
---@param t table
---@param key? any
---@return any key
---@return any value
function next(t, key) end

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

--- Raises an error when `value` is false or `nil`; returns its arguments
--- otherwise.
---
--- # Examples
---
--- ```lua
--- local git = assert(find_tool("git"), "git not found")
--- ```
---@param value any
---@param message? string
---@return any
function assert(value, message) end

--- Raises an error.
---
--- # Examples
---
--- ```lua
--- error("invalid configuration")
--- ```
---
--- # Notes
---
--- In xmake scripts, `raise` also formats its message.
---@param message any
---@param level? integer
function error(message, level) end

--- Calls a function and catches its errors: `true` and its results, or
--- `false` and the error.
---
--- # Examples
---
--- ```lua
--- local ok, err = pcall(os.rm, "build")
--- if not ok then
---     print("cannot remove: %s", err)
--- end
--- ```
---
--- # Notes
---
--- In xmake scripts, `try { ... }` reads better.
---@param f function
---@param ... any
---@return boolean ok
---@return any result
function pcall(f, ...) end

--- Returns the arguments after the `n`-th, or their count with `"#"`.
---
--- # Examples
---
--- ```lua
--- local count = select("#", ...)
--- local second = select(2, ...)
--- ```
---@param n integer|string
---@param ... any
---@return any
function select(n, ...) end

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
function format(format, ...) end
