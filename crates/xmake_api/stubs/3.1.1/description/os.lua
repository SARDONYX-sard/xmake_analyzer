---@meta
---@diagnostic disable: unused-local

--- The `os` module: files, processes and the environment.
---@class os
---@see https://xmake.io/api/scripts/builtin-modules/os.html
os = {}

--- Whether `path` is an existing file.
---
--- # Examples
---
--- ```lua
--- if os.isfile("config.json") then
---     target:add("defines", "HAS_CONFIG")
--- end
--- ```
---@param path string
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-isfile
function os.isfile(path) end

--- Whether `path` is an existing directory.
---
--- # Examples
---
--- ```lua
--- if not os.isdir("third_party") then
---     raise("run git submodule update --init first")
--- end
--- ```
---@param path string
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-isdir
function os.isdir(path) end

--- Whether `path` exists, file or directory.
---
--- # Examples
---
--- ```lua
--- if os.exists("dist") then
---     os.rm("dist")
--- end
--- ```
---@param path string
---@return boolean
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-exists
function os.exists(path) end

--- Returns the files matching a glob.
---
--- # Examples
---
--- ```lua
--- for _, file in ipairs(os.files("src/**.c")) do
---     print(file)
--- end
--- ```
---
--- # Notes
---
--- `os.dirs` returns directories; `os.filedirs` both.
---@param pattern string A glob: `*`, `**`.
---@return string[]
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-files
function os.files(pattern) end

--- Returns the directories matching a glob.
---
--- # Examples
---
--- ```lua
--- for _, dir in ipairs(os.dirs("plugins/*")) do
---     print(path.filename(dir))
--- end
--- ```
---@param pattern string
---@return string[]
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-dirs
function os.dirs(pattern) end

---@param pattern string
---@return string[]
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-filedirs
function os.filedirs(pattern) end

--- Returns the current directory.
---
--- # Examples
---
--- ```lua
--- print(os.curdir())
--- ```
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-curdir
function os.curdir() end

--- Returns the root directory of the project.
---
--- # Examples
---
--- ```lua
--- local config = path.join(os.projectdir(), "config.json")
--- ```
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-projectdir
function os.projectdir() end

--- Returns the directory of the running script.
---
--- # Examples
---
--- ```lua
--- local data = io.readfile(path.join(os.scriptdir(), "data.txt"))
--- ```
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-scriptdir
function os.scriptdir() end

---@return string
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-programdir
function os.programdir() end

--- Returns the temporary directory of xmake.
---
--- # Examples
---
--- ```lua
--- local file = path.join(os.tmpdir(), "out.txt")
--- ```
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-tmpdir
function os.tmpdir() end

--- Returns the value of an environment variable, or `nil`.
---
--- # Examples
---
--- ```lua
--- local home = os.getenv("HOME") or os.getenv("USERPROFILE")
--- ```
---@param name string
---@return string?
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-getenv
function os.getenv(name) end

--- Returns the system xmake runs on: `windows`, `linux`, `macosx`, ...
---
--- # Examples
---
--- ```lua
--- if os.host() == "windows" then
---     os.run("cmd /c ver")
--- end
--- ```
---@return "windows"|"linux"|"macosx"|"bsd"|"haiku"|"solaris"
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-host
function os.host() end

--- Returns the architecture of the system xmake runs on.
---
--- # Examples
---
--- ```lua
--- print(os.arch()) -- x64, x86_64, arm64, ...
--- ```
---@return string
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-arch
function os.arch() end

---@param format string
---@return string
function os.date(format) end

---@return integer
function os.time() end

---@param path string
---@return integer
function os.mtime(path) end

---@param path string
---@return integer
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-filesize
function os.filesize(path) end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-cpuinfo
function os.cpuinfo() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-default-njob
function os.default_njob() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-mclock
function os.mclock() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-programfile
function os.programfile() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-projectfile
function os.projectfile() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-subarch
function os.subarch() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-subhost
function os.subhost() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-term
function os.term() end
