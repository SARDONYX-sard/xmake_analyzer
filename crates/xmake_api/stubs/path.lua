---@meta

--- The `path` module: path manipulations, without touching the disk.
---@class path
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
function path.relative(path, rootdir) end

---@param path string
---@return boolean
function path.is_absolute(path) end

---@param path string
---@return string
function path.translate(path) end

---@param path string
---@return string
function path.unix(path) end

---@param path string
---@return string[]
function path.split(path) end

---@return string
function path.sep() end

---@return string
function path.envsep() end

