---@meta

--- The `path` module: path manipulations, without touching the disk.
---@class path
---@see https://xmake.io/api/scripts/builtin-modules/path.html
path = {}

--- Joins path segments with the separator of the system.
---
--- # Examples
---
--- ```lua
--- local header = path.join(target:autogendir(), "include", "config.h")
--- ```
---@param ... string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-join
---@xmake.context description script
function path.join(...) end

--- Returns the file name without directory and extension.
---
--- # Examples
---
--- ```lua
--- print(path.basename("src/main.c")) -- main
--- ```
---@param path string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-basename
---@xmake.context description script
function path.basename(path) end

--- Returns the file name without directory.
---
--- # Examples
---
--- ```lua
--- print(path.filename("src/main.c")) -- main.c
--- ```
---@param path string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-filename
---@xmake.context description script
function path.filename(path) end

--- Returns the extension, with its dot.
---
--- # Examples
---
--- ```lua
--- print(path.extension("src/main.c")) -- .c
--- ```
---@param path string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-extension
---@xmake.context description script
function path.extension(path) end

--- Returns the directory of a path.
---
--- # Examples
---
--- ```lua
--- print(path.directory("src/main.c")) -- src
--- ```
---@param path string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-directory
---@xmake.context description script
function path.directory(path) end

--- Returns the absolute path, relative to the current or given directory.
---
--- # Examples
---
--- ```lua
--- local abs = path.absolute("src/main.c", os.projectdir())
--- ```
---@param path string
---@param rootdir? string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-absolute
---@xmake.context description script
function path.absolute(path, rootdir) end

--- Returns the path relative to the current or given directory.
---
--- # Examples
---
--- ```lua
--- print(path.relative("/home/me/project/src/a.c", "/home/me/project")) -- src/a.c
--- ```
---@param path string
---@param rootdir? string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-relative
---@xmake.context description script
function path.relative(path, rootdir) end

---@param path string
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-is-absolute
---@xmake.context description script
function path.is_absolute(path) end

---@param path string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-translate
---@xmake.context description script
function path.translate(path) end

---@param path string
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-unix
---@xmake.context description script
function path.unix(path) end

---@param path string
---@return string[]
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-split
---@xmake.context description script
function path.split(path) end

---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-sep
---@xmake.context description script
function path.sep() end

---@return string
---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-envsep
---@xmake.context description script
function path.envsep() end

---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-cygwin
---@xmake.context description script
function path.cygwin(...) end

---@xmake.context description script
function path.cygwin_path(...) end

---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-instance-of
---@xmake.context description script
function path.instance_of(...) end

---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-islastsep
---@xmake.context description script
function path.islastsep(...) end

---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-joinenv
---@xmake.context description script
function path.joinenv(...) end

---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-new
---@xmake.context description script
function path.new(...) end

---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-normalize
---@xmake.context description script
function path.normalize(...) end

---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-pattern
---@xmake.context description script
function path.pattern(...) end

---@see https://xmake.io/api/scripts/builtin-modules/path.html#path-splitenv
---@xmake.context description script
function path.splitenv(...) end

