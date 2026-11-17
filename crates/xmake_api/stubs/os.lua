---@meta

--- The `os` module: files, processes and the environment.
---@class os
os = {}

--- Copies files or directories. Globs are supported.
---
--- # Examples
---
--- ```lua
--- os.cp("README.md", "dist/")
--- os.cp("$(scriptdir)/include/*.h", "$(projectdir)/out/include")
--- os.cp("assets", "dist/assets")
--- ```
---
--- # Notes
---
--- `{rootdir = "src"}` keeps the paths of the matches relative to `rootdir`.
---@param src string The files or directories, globs allowed.
---@param dst string The destination.
---@param opt? {rootdir?: string, symlink?: boolean}
function os.cp(src, dst, opt) end

--- Moves or renames files or directories.
---
--- # Examples
---
--- ```lua
--- os.mv("build/app", "dist/app")
--- ```
---@param src string
---@param dst string
function os.mv(src, dst) end

--- Removes files or directories, recursively. Globs are supported.
---
--- # Examples
---
--- ```lua
--- os.rm("build/*.o")
--- os.rm("$(projectdir)/dist")
--- ```
---@param path string Files or directories, globs allowed.
function os.rm(path) end

--- Creates directories, with their parents.
---
--- # Examples
---
--- ```lua
--- os.mkdir(path.join(target:autogendir(), "include"))
--- ```
---@param path string
function os.mkdir(path) end

---@param path string
function os.rmdir(path) end

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
function os.dirs(pattern) end

---@param pattern string
---@return string[]
function os.filedirs(pattern) end

--- Runs a command line quietly; formats it like `string.format`. Raises an error on failure.
---
--- # Examples
---
--- ```lua
--- os.run("git describe --tags")
--- os.run("%s --version", program)
--- ```
---
--- # Notes
---
--- `os.runv(program, argv)` takes the arguments as a list, without quoting issues. `os.vrun` echoes the command in verbose mode.
---@param cmd string A command line, formatted like `string.format`.
---@param ... any
function os.run(cmd, ...) end

--- Runs a program with a list of arguments, quietly. Raises an error on failure.
---
--- # Examples
---
--- ```lua
--- os.runv("git", {"clone", "--depth", "1", url, dir})
--- ```
---@param program string
---@param argv string[]
---@param opt? {envs?: table, curdir?: string}
function os.runv(program, argv, opt) end

---@param cmd string
---@param ... any
function os.vrun(cmd, ...) end

--- Like `os.runv`, but prints the command and its output in verbose mode (`xmake -v`).
---
--- # Examples
---
--- ```lua
--- os.vrunv("cmake", {"--build", "build"})
--- ```
---@param program string
---@param argv string[]
---@param opt? {envs?: table, curdir?: string}
function os.vrunv(program, argv, opt) end

--- Runs a command line, its output shown. Raises an error on failure.
---
--- # Examples
---
--- ```lua
--- os.exec("ctest --output-on-failure")
--- ```
---
--- # Notes
---
--- `os.execv(program, argv)` takes the arguments as a list.
---@param cmd string
---@param ... any
function os.exec(cmd, ...) end

--- Runs a program with a list of arguments, its output shown. Raises an error on failure.
---
--- # Examples
---
--- ```lua
--- os.execv(target:targetfile(), {"--help"})
--- ```
---
--- # Notes
---
--- A failing program raises an error, unless `opt.try` is set: then the exit
--- code and the error are returned.
---
--- ```lua
--- local code, errors = os.execv("git", {"status"}, {try = true})
--- if code ~= 0 then
---     print("git failed: %s", errors)
--- end
--- ```
---@param program string
---@param argv string[]
---@param opt? {envs?: table, curdir?: string, try?: boolean, stdin?: string, stdout?: string, stderr?: string, detach?: boolean, timeout?: integer, dryrun?: boolean}
---@return integer? code the exit code; `nil` when the program cannot start
---@return string? errors why it failed
function os.execv(program, argv, opt) end

--- `os.execv`, echoing the command with `xmake -v`.
---
--- # Examples
---
--- ```lua
--- os.vexecv("cmake", {"--build", "build"})
--- ```
---@param program string
---@param argv string[]
---@param opt? {envs?: table, curdir?: string, try?: boolean, stdin?: string, stdout?: string, stderr?: string, detach?: boolean, timeout?: integer, dryrun?: boolean}
---@return integer? code the exit code; `nil` when the program cannot start
---@return string? errors why it failed
function os.vexecv(program, argv, opt) end

--- Runs a command line and returns its standard output and error.
---
--- # Examples
---
--- ```lua
--- local out, err = os.iorun("git rev-parse --short HEAD")
--- target:add("defines", "GIT_COMMIT=\"" .. out:trim() .. "\"")
--- ```
---
--- # Notes
---
--- `os.iorunv(program, argv)` takes the arguments as a list.
---@param cmd string
---@param ... any
---@return string outdata
---@return string errdata
function os.iorun(cmd, ...) end

--- Runs a program with a list of arguments and returns its standard output and error.
---
--- # Examples
---
--- ```lua
--- local out = os.iorunv("pkg-config", {"--cflags", "gtk+-3.0"})
--- ```
---@param program string
---@param argv string[]
---@param opt? {envs?: table, curdir?: string}
---@return string outdata
---@return string errdata
function os.iorunv(program, argv, opt) end

--- Changes the current directory and returns the previous one.
---
--- # Examples
---
--- ```lua
--- local old = os.cd("build")
--- os.run("make")
--- os.cd(old)
--- ```
---@param dir string
---@return string previous
function os.cd(dir) end

--- Returns the current directory.
---
--- # Examples
---
--- ```lua
--- print(os.curdir())
--- ```
---@return string
function os.curdir() end

--- Returns the root directory of the project.
---
--- # Examples
---
--- ```lua
--- local config = path.join(os.projectdir(), "config.json")
--- ```
---@return string
function os.projectdir() end

--- Returns the directory of the running script.
---
--- # Examples
---
--- ```lua
--- local data = io.readfile(path.join(os.scriptdir(), "data.txt"))
--- ```
---@return string
function os.scriptdir() end

---@return string
function os.programdir() end

--- Returns the temporary directory of xmake.
---
--- # Examples
---
--- ```lua
--- local file = path.join(os.tmpdir(), "out.txt")
--- ```
---@return string
function os.tmpdir() end

---@return string
function os.tmpfile() end

--- Returns the value of an environment variable, or `nil`.
---
--- # Examples
---
--- ```lua
--- local home = os.getenv("HOME") or os.getenv("USERPROFILE")
--- ```
---@param name string
---@return string?
function os.getenv(name) end

--- Sets an environment variable for the commands run later.
---
--- # Examples
---
--- ```lua
--- os.setenv("CC", "clang")
--- ```
---
--- # Notes
---
--- `os.addenv(name, value)` appends to a list variable such as `PATH`.
---@param name string
---@param value string
---@return boolean ok
function os.setenv(name, value) end

---@param name string
---@param ... string
---@return boolean ok
function os.addenv(name, ...) end

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
function os.host() end

--- Returns the architecture of the system xmake runs on.
---
--- # Examples
---
--- ```lua
--- print(os.arch()) -- x64, x86_64, arm64, ...
--- ```
---@return string
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
function os.filesize(path) end

---@param milliseconds integer
function os.sleep(milliseconds) end

