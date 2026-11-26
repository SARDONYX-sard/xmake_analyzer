---@meta
---@diagnostic disable: unused-local

--- The `os` module: files, processes and the environment.
---@class os
---@see https://xmake.io/api/scripts/builtin-modules/os.html
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-cp
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-mv
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-rm
function os.rm(path) end

--- Creates directories, with their parents.
---
--- # Examples
---
--- ```lua
--- os.mkdir(path.join(target:autogendir(), "include"))
--- ```
---@param path string
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-mkdir
function os.mkdir(path) end

---@param path string
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-rmdir
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-run
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-runv
function os.runv(program, argv, opt) end

---@param cmd string
---@param ... any
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-vrun
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-vrunv
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-exec
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
---@return integer? code
---@return string? errors
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-execv
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
---@return integer? code
---@return string? errors
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-iorun
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-iorunv
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-cd
function os.cd(dir) end

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

---@return string
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-tmpfile
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-getenv
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
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-setenv
function os.setenv(name, value) end

---@param name string
---@param ... string
---@return boolean ok
---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-addenv
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

---@param milliseconds integer
function os.sleep(milliseconds) end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-addenvp
function os.addenvp() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-addenvs
function os.addenvs() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-args
function os.args() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-argv
function os.argv() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-atexit
function os.atexit() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-cpuinfo
function os.cpuinfo() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-default-njob
function os.default_njob() end

function os.emptydir() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-exit
function os.exit() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-features
function os.features() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-fscase
function os.fscase() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-getenvs
function os.getenvs() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-getpid
function os.getpid() end

function os.getwinsize() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-is-arch
function os.is_arch() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-is-host
function os.is_host() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-is-subarch
function os.is_subarch() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-is-subhost
function os.is_subhost() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-isexec
function os.isexec() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-islink
function os.islink() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-isroot
function os.isroot() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-joinenvs
function os.joinenvs() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-ln
function os.ln() end

function os.match() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-mclock
function os.mclock() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-meminfo
function os.meminfo() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-nuldev
function os.nuldev() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-pbcopy
function os.pbcopy() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-pbpaste
function os.pbpaste() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-programfile
function os.programfile() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-projectfile
function os.projectfile() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-raise
function os.raise() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-readlink
function os.readlink() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-setenvp
function os.setenvp() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-setenvs
function os.setenvs() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-shell
function os.shell() end

function os.strerror() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-subarch
function os.subarch() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-subhost
function os.subhost() end

function os.syserror() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-term
function os.term() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-touch
function os.touch() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-trycp
function os.trycp() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-trymv
function os.trymv() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-tryrm
function os.tryrm() end

function os.vcp() end

function os.vexec() end

function os.vln() end

function os.vmv() end

function os.vrm() end

---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-workingdir
function os.workingdir() end

function os.xmakever() end
