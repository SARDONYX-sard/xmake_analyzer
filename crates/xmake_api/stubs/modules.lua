---@meta

--- Semantic versions: parse, compare and select.
---
--- # Examples
---
--- ```lua
--- import("core.base.semver")
--- local v = semver.new("1.2.3")
--- if v:ge("1.2.0") then
---     print(v:major(), v:minor(), v:patch())
--- end
--- print(semver.satisfies("1.2.3", ">=1.0 <2.0")) -- true
--- ```
---@class core.base.semver
local semver_module = {}

--- Parses a version.
---@param version string
---@return semver
function semver_module.new(version) end

--- Compares two versions: negative, zero or positive.
---@param a string
---@param b string
---@return integer
function semver_module.compare(a, b) end

--- Whether a version satisfies a range such as `>=1.0 <2.0` or `1.2.x`.
---@param version string
---@param range string
---@return boolean
function semver_module.satisfies(version, range) end

--- Selects the newest version satisfying a range among lists of versions,
--- tags and branches.
---@param range string
---@param versions string[]
---@param tags? string[]
---@param branches? string[]
---@return string? version
---@return string? source
function semver_module.select(range, versions, tags, branches) end

--- A parsed version, from `semver.new` or `xmake.version()`.
---@class semver
local semver = {}

--- The major number.
---@return integer
function semver:major() end

--- The minor number.
---@return integer
function semver:minor() end

--- The patch number.
---@return integer
function semver:patch() end

--- `major.minor.patch`.
---@return string
function semver:shortstr() end

--- The version as written.
---@return string
function semver:rawstr() end

--- Whether the version is greater than or equal to `other`.
---@param other string|semver
---@return boolean
function semver:ge(other) end

--- Whether the version is greater than `other`.
---@param other string|semver
---@return boolean
function semver:gt(other) end

--- Whether the version is less than or equal to `other`.
---@param other string|semver
---@return boolean
function semver:le(other) end

--- Whether the version is less than `other`.
---@param other string|semver
---@return boolean
function semver:lt(other) end

--- Whether the version equals `other`.
---@param other string|semver
---@return boolean
function semver:eq(other) end

--- A set of values.
---
--- # Examples
---
--- ```lua
--- import("core.base.hashset")
--- local seen = hashset.new()
--- seen:insert("a")
--- print(seen:has("a"), seen:size())
--- ```
---@class core.base.hashset
local hashset_module = {}

--- Creates an empty set.
---@return hashset
function hashset_module.new() end

--- Creates a set of the arguments.
---@param ... any
---@return hashset
function hashset_module.of(...) end

--- Creates a set of the values of a list.
---@param list any[]
---@return hashset
function hashset_module.from(list) end

--- A set, from `core.base.hashset`.
---@class hashset
local hashset = {}

--- Adds a value; returns whether it was new.
---@param value any
---@return boolean
function hashset:insert(value) end

--- Whether the set has the value.
---@param value any
---@return boolean
function hashset:has(value) end

--- Removes a value; returns whether it was there.
---@param value any
---@return boolean
function hashset:remove(value) end

--- The number of values.
---@return integer
function hashset:size() end

--- Whether the set is empty.
---@return boolean
function hashset:empty() end

--- The values, as a list.
---@return any[]
function hashset:to_array() end

--- Iterates over the values.
---@return fun(): any
function hashset:items() end

--- The global configuration of xmake (`xmake g`).
---@class core.base.global
local global = {}

--- Returns a global configuration value.
---@param name string
---@return any
function global.get(name) end

--- Archives: extract and create `.zip`, `.7z`, `.tar.gz`, ...
---
--- # Examples
---
--- ```lua
--- import("utils.archive")
--- archive.extract("sdk.zip", "build/sdk")
--- ```
---@class utils.archive
local archive = {}

--- Extracts an archive into a directory.
---@param archivefile string
---@param outputdir string
---@param opt? {excludes?: string[], extension?: string}
---@return boolean
function archive.extract(archivefile, outputdir, opt) end

--- Creates an archive of files or directories.
---@param archivefile string
---@param inputfiles string|string[]
---@param opt? {curdir?: string, compress?: string, recurse?: boolean}
---@return boolean
function archive.archive(archivefile, inputfiles, opt) end

--- HTTP downloads.
---
--- # Examples
---
--- ```lua
--- import("net.http")
--- http.download("https://example.com/sdk.zip", "build/sdk.zip")
--- ```
---@class net.http
local http = {}

--- Downloads a URL to a file.
---@param url string
---@param outputfile string
---@param opt? {headers?: string[], timeout?: integer, insecure?: boolean}
function http.download(url, outputfile, opt) end

--- Git commands.
---
--- # Examples
---
--- ```lua
--- import("devel.git")
--- git.clone("https://github.com/xmake-io/xmake.git", {depth = 1, outputdir = "xmake"})
--- ```
---@class devel.git
local git = {}

--- Clones a repository.
---@param url string
---@param opt? {depth?: integer, branch?: string, outputdir?: string, recursive?: boolean}
function git.clone(url, opt) end

--- Pulls the current branch.
---@param opt? {remote?: string, branch?: string, repodir?: string}
function git.pull(opt) end

--- Checks out a commit, a tag or a branch.
---@param commit string
---@param opt? {repodir?: string}
function git.checkout(commit, opt) end

--- Runs jobs in parallel.
---
--- # Examples
---
--- ```lua
--- import("async.runjobs")
--- runjobs("compile", function (index)
---     print("job %d", index)
--- end, {total = 8, comax = 4})
--- ```
---@class async.runjobs
---@overload fun(name: string, jobfunc: fun(index: integer), opt?: {total?: integer, comax?: integer, timeout?: integer})
local runjobs = {}

--- Finds a file in directories.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_file")
--- local header = find_file("zlib.h", {"/usr/include", "/usr/local/include"})
--- ```
---@class lib.detect.find_file
---@overload fun(name: string, paths: string[], opt?: {suffixes?: string[]}): string?
local find_file = {}

--- Finds the directory holding a file.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_path")
--- local includedir = find_path("zlib.h", {"/usr/include", "/usr/local/include"})
--- ```
---@class lib.detect.find_path
---@overload fun(name: string, paths: string[], opt?: {suffixes?: string[]}): string?
local find_path = {}

--- Finds a library: its directory, link name and file name.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_library")
--- local zlib = find_library("z", {"/usr/lib", "/usr/local/lib"})
--- if zlib then
---     print(zlib.linkdir, zlib.link)
--- end
--- ```
---@class lib.detect.find_library
---@overload fun(name: string, paths: string[], opt?: {kind?: string, suffixes?: string[]}): {linkdir: string, link: string, filename: string}?
local find_library = {}

--- Finds a package on the system: its includes, links and defines.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_package")
--- local zlib = find_package("zlib")
--- if zlib then
---     print(zlib.includedirs, zlib.links)
--- end
--- ```
---@class lib.detect.find_package
---@overload fun(name: string, opt?: {version?: boolean, system?: boolean}): {links?: string[], linkdirs?: string[], includedirs?: string[], defines?: string[], version?: string}?
local find_package = {}

--- Returns the version of a program.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_programver")
--- print(find_programver("cmake"))
--- ```
---@class lib.detect.find_programver
---@overload fun(program: string, opt?: {command?: string, parse?: string}): string?
local find_programver = {}

--- Tasks and plugins.
---@class core.base.task
local task = {}

--- Runs a task or a plugin, as `xmake <name>` would.
---
--- # Examples
---
--- ```lua
--- import("core.base.task")
--- task.run("install")
--- task.run("project", {kind = "compile_commands"})
--- ```
---@param name string
---@param opt? table The options of its menu.
---@param ... any
---@return any ... what the `on_run` of the task returns
function task.run(name, opt, ...) end
