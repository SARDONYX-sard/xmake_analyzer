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
---@see https://xmake.io/api/scripts/builtin-modules/cprint.html
---@xmake.context script
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
---@see https://xmake.io/api/scripts/builtin-modules/import.html
---@xmake.context script
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
---@see https://xmake.io/api/scripts/builtin-modules/print.html
---@xmake.context description script
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
---@see https://xmake.io/api/scripts/builtin-modules/raise.html
---@xmake.context script
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
---@xmake.context script
function try(block) end

--- Prints only in verbose mode (`xmake -v`).
---
--- # Examples
---
--- ```lua
--- vprint("compiling %s", sourcefile)
--- ```
---@param ... any
---@xmake.context script
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
---@see https://xmake.io/api/scripts/builtin-modules/ipairs.html
---@xmake.context description script
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
---@xmake.context description script
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
---@xmake.context description script
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
---@xmake.context description script
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
---@xmake.context description script
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
---@xmake.context script
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
---@xmake.context description script
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
---@xmake.context description script
function format(format, ...) end

---@xmake.context description
function add_subdirs(...) end

---@xmake.context description
function add_subfiles(...) end

---@xmake.context script
function catch(...) end

---@see https://xmake.io/api/scripts/builtin-modules/cprintf.html
---@xmake.context script
function cprintf(...) end

---@xmake.context script
function dprint(...) end

---@xmake.context script
function dprintf(...) end

---@xmake.context script
function finally(...) end

---@xmake.context script
function find_package(...) end

---@xmake.context script
function find_packages(...) end

---@xmake.prototype get_config(name: string)
---@xmake.param name Configuration option name string
---@see https://xmake.io/api/description/global-interfaces.html#get-config
---@xmake.context script
function get_config(...) end

---@xmake.context description
function getenv(...) end

---@xmake.prototype has_config(configs: string, ...)
---@xmake.param configs Configuration name string
---@xmake.param ... Variable parameters, can pass multiple configuration names
---@see https://xmake.io/api/description/conditions.html#has-config
---@xmake.context script
function has_config(...) end

---@xmake.prototype has_package(packages: string, ...)
---@xmake.param packages Package name string
---@xmake.param ... Variable parameters, can pass multiple package names
---@see https://xmake.io/api/description/conditions.html#has-package
---@xmake.context script
function has_package(...) end

---@xmake.prototype includes(paths: string|array, ..., {rootdir})
---@xmake.param paths Sub-project file or directory path, supports pattern matching like "**/xmake.lua"
---@xmake.param ... Variable parameters, can pass multiple paths
---@xmake.param rootdir Root directory for relative path resolution, optional
---@see https://xmake.io/api/description/global-interfaces.html#includes
---@xmake.context description
function includes(...) end

---@see https://xmake.io/api/scripts/builtin-modules/inherit.html
---@xmake.context script
function inherit(...) end

---@xmake.context script
function irpairs(...) end

---@xmake.prototype is_arch(arch: string, ...)
---@xmake.param arch Architecture name string
---@xmake.param ... Variable parameters, can pass multiple architecture names
---@see https://xmake.io/api/description/conditions.html#is-arch
---@xmake.context script
function is_arch(...) end

---@xmake.prototype is_config(name: string, values: string, ...)
---@xmake.param name Configuration option name string
---@xmake.param values Configuration value string
---@xmake.param ... Variable parameters, can pass multiple values
---@see https://xmake.io/api/description/conditions.html#is-config
---@xmake.context script
function is_config(...) end

---@xmake.prototype is_host(host: string, ...)
---@xmake.param host Host system name string
---@xmake.param ... Variable parameters, can pass multiple host names
---@see https://xmake.io/api/description/conditions.html#is-host
---@xmake.context script
function is_host(...) end

---@xmake.prototype is_mode(mode: string, ...)
---@xmake.param mode Compilation mode name string
---@xmake.param ... Variable parameters, can pass multiple mode names
---@see https://xmake.io/api/description/conditions.html#is-mode
---@xmake.context script
function is_mode(...) end

---@xmake.prototype is_plat(plat: string, ...)
---@xmake.param plat Platform name string
---@xmake.param ... Variable parameters, can pass multiple platform names
---@see https://xmake.io/api/description/conditions.html#is-plat
---@xmake.context script
function is_plat(...) end

---@xmake.prototype is_subhost(subhost: string, ...)
---@xmake.param subhost Subsystem name string
---@xmake.param ... Variable parameters, can pass multiple subsystem names
---@see https://xmake.io/api/description/conditions.html#is-subhost
---@xmake.context description script
function is_subhost(...) end

---@xmake.prototype namespace(name: string, script: function)
---@xmake.param name Namespace name string
---@xmake.param script Namespace script function
---@see https://xmake.io/api/description/global-interfaces.html#namespace
---@xmake.context description
function namespace(...) end

---@xmake.prototype namespace_end()
---@xmake.param (none) No parameters required
---@see https://xmake.io/api/description/global-interfaces.html#namespace-end
---@xmake.context description
function namespace_end(...) end

---@see https://xmake.io/api/scripts/builtin-modules/printf.html
---@xmake.context description script
function printf(...) end

---@xmake.prototype set_xmakever(version: string)
---@xmake.param version Minimum required xmake version string, e.g. "2.1.0"
---@see https://xmake.io/api/description/global-interfaces.html#set-xmakever
---@xmake.context description
function set_xmakever(...) end

---@xmake.context script
function todisplay(...) end

---@xmake.context script
function val(...) end

---@see https://xmake.io/api/scripts/builtin-modules/vformat.html
---@xmake.context script
function vformat(...) end

---@xmake.context script
function vprintf(...) end

---@xmake.context script
function wprint(...) end
