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
---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html
local semver_module = {}

--- Parses a version.
---@param version string
---@return semver
---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-new
---@xmake.context import
function semver_module.new(version) end

--- Compares two versions: negative, zero or positive.
---@param a string
---@param b string
---@return integer
---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-compare
---@xmake.context import
function semver_module.compare(a, b) end

--- Whether a version satisfies a range such as `>=1.0 <2.0` or `1.2.x`.
---@param version string
---@param range string
---@return boolean
---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-satisfies
---@xmake.context import
function semver_module.satisfies(version, range) end

--- Selects the newest version satisfying a range among lists of versions,
--- tags and branches.
---@param range string
---@param versions string[]
---@param tags? string[]
---@param branches? string[]
---@return string? version
---@return string? source
---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-select
---@xmake.context import
function semver_module.select(range, versions, tags, branches) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-is-valid
---@xmake.context import
function semver_module.is_valid(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-is-valid-range
---@xmake.context import
function semver_module.is_valid_range(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-match
---@xmake.context import
function semver_module.match(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/semver.html#semver-try-parse
---@xmake.context import
function semver_module.try_parse(...) end

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
---@see https://xmake.io/api/scripts/extension-modules/core/base/hashset.html
local hashset_module = {}

--- Creates an empty set.
---@return hashset
---@see https://xmake.io/api/scripts/extension-modules/core/base/hashset.html#hashset-new
---@xmake.context import
function hashset_module.new() end

--- Creates a set of the arguments.
---@param ... any
---@return hashset
---@see https://xmake.io/api/scripts/extension-modules/core/base/hashset.html#hashset-of
---@xmake.context import
function hashset_module.of(...) end

--- Creates a set of the values of a list.
---@param list any[]
---@return hashset
---@see https://xmake.io/api/scripts/extension-modules/core/base/hashset.html#hashset-from
---@xmake.context import
function hashset_module.from(list) end

---@xmake.context import
function hashset_module.clear(...) end

---@xmake.context import
function hashset_module.clone(...) end

---@xmake.context import
function hashset_module.data(...) end

---@xmake.context import
function hashset_module.empty(...) end

---@xmake.context import
function hashset_module.has(...) end

---@xmake.context import
function hashset_module.insert(...) end

---@xmake.context import
function hashset_module.items(...) end

---@xmake.context import
function hashset_module.keys(...) end

---@xmake.context import
function hashset_module.orderitems(...) end

---@xmake.context import
function hashset_module.orderkeys(...) end

---@xmake.context import
function hashset_module.remove(...) end

---@xmake.context import
function hashset_module.size(...) end

---@xmake.context import
function hashset_module.to_array(...) end

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
---@see https://xmake.io/api/scripts/extension-modules/core/base/global.html
local global = {}

--- Returns a global configuration value.
---@param name string
---@return any
---@see https://xmake.io/api/scripts/extension-modules/core/base/global.html#global-get
---@xmake.context import
function global.get(name) end

---@xmake.context import
function global.cachedir(...) end

---@xmake.context import
function global.clear(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/global.html#global-directory
---@xmake.context import
function global.directory(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/global.html#global-dump
---@xmake.context import
function global.dump(...) end

---@xmake.context import
function global.filepath(...) end

---@xmake.context import
function global.options(...) end

---@xmake.context import
function global.readonly(...) end

---@xmake.context import
function global.save(...) end

---@xmake.context import
function global.set(...) end

--- Archives: extract and create `.zip`, `.7z`, `.tar.gz`, ...
---
--- # Examples
---
--- ```lua
--- import("utils.archive")
--- archive.extract("sdk.zip", "build/sdk")
--- ```
---@class utils.archive
---@see https://xmake.io/api/scripts/extension-modules/utils/archive.html
local archive = {}

--- Extracts an archive into a directory.
---@param archivefile string
---@param outputdir string
---@param opt? {excludes?: string[], extension?: string}
---@return boolean
---@see https://xmake.io/api/scripts/extension-modules/utils/archive.html#archive-extract
---@xmake.context import
function archive.extract(archivefile, outputdir, opt) end

--- Creates an archive of files or directories.
---@param archivefile string
---@param inputfiles string|string[]
---@param opt? {curdir?: string, compress?: string, recurse?: boolean}
---@return boolean
---@see https://xmake.io/api/scripts/extension-modules/utils/archive.html#archive-archive
---@xmake.context import
function archive.archive(archivefile, inputfiles, opt) end

---@xmake.context import
function archive.archive_xmz(...) end

---@xmake.context import
function archive.extension(...) end

---@xmake.context import
function archive.extract_xmz(...) end

---@xmake.context import
function archive.merge_staticlib(...) end

--- HTTP downloads.
---
--- # Examples
---
--- ```lua
--- import("net.http")
--- http.download("https://example.com/sdk.zip", "build/sdk.zip")
--- ```
---@class net.http
---@see https://xmake.io/api/scripts/extension-modules/net/http.html
local http = {}

--- Downloads a URL to a file.
---@param url string
---@param outputfile string
---@param opt? {headers?: string[], timeout?: integer, insecure?: boolean}
---@see https://xmake.io/api/scripts/extension-modules/net/http.html#http-download
---@xmake.context import
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
---@see https://xmake.io/api/scripts/extension-modules/devel/git.html
local git = {}

--- Clones a repository.
---@param url string
---@param opt? {depth?: integer, branch?: string, outputdir?: string, recursive?: boolean}
---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-clone
---@xmake.context import
function git.clone(url, opt) end

--- Pulls the current branch.
---@param opt? {remote?: string, branch?: string, repodir?: string}
---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-pull
---@xmake.context import
function git.pull(opt) end

--- Checks out a commit, a tag or a branch.
---@param commit string
---@param opt? {repodir?: string}
---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-checkout
---@xmake.context import
function git.checkout(commit, opt) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-apply
---@xmake.context import
function git.apply(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-asgiturl
---@xmake.context import
function git.asgiturl(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-branch
---@xmake.context import
function git.branch(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-branches
---@xmake.context import
function git.branches(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-checkurl
---@xmake.context import
function git.checkurl(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-clean
---@xmake.context import
function git.clean(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-init
---@xmake.context import
function git.init(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-lastcommit
---@xmake.context import
function git.lastcommit(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-ls-remote
---@xmake.context import
function git.ls_remote(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-push
---@xmake.context import
function git.push(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-refs
---@xmake.context import
function git.refs(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-reset
---@xmake.context import
function git.reset(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#git-tags
---@xmake.context import
function git.tags(...) end

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
---@see https://xmake.io/api/scripts/extension-modules/async/runjobs.html
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
---@see https://xmake.io/api/scripts/extension-modules/core/base/task.html
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
---@see https://xmake.io/api/scripts/extension-modules/core/base/task.html#task-run
---@xmake.context import
function task.run(name, opt, ...) end

---@xmake.context import
function task.names(...) end

---@class coroutine
local coroutine = {}

---@see https://xmake.io/api/scripts/builtin-modules/coroutine.html#coroutine-create
---@xmake.context script
function coroutine.create(...) end

---@see https://xmake.io/api/scripts/builtin-modules/coroutine.html#coroutine-resume
---@xmake.context script
function coroutine.resume(...) end

---@see https://xmake.io/api/scripts/builtin-modules/coroutine.html#coroutine-running
---@xmake.context script
function coroutine.running(...) end

---@see https://xmake.io/api/scripts/builtin-modules/coroutine.html#coroutine-status
---@xmake.context script
function coroutine.status(...) end

---@see https://xmake.io/api/scripts/builtin-modules/coroutine.html#coroutine-wrap
---@xmake.context script
function coroutine.wrap(...) end

---@see https://xmake.io/api/scripts/builtin-modules/coroutine.html#coroutine-yield
---@xmake.context script
function coroutine.yield(...) end

---@class debug
local debug = {}

---@xmake.context script
function debug.collectgarbage(...) end

---@xmake.context script
function debug.debug(...) end

---@xmake.context script
function debug.getfenv(...) end

---@xmake.context script
function debug.gethook(...) end

---@xmake.context script
function debug.getinfo(...) end

---@xmake.context script
function debug.getlocal(...) end

---@xmake.context script
function debug.getmetatable(...) end

---@xmake.context script
function debug.getregistry(...) end

---@xmake.context script
function debug.getupvalue(...) end

---@xmake.context script
function debug.getuservalue(...) end

---@xmake.context script
function debug.global(...) end

---@xmake.context script
function debug.rawequal(...) end

---@xmake.context script
function debug.rawget(...) end

---@xmake.context script
function debug.rawlen(...) end

---@xmake.context script
function debug.rawset(...) end

---@xmake.context script
function debug.require(...) end

---@xmake.context script
function debug.setfenv(...) end

---@xmake.context script
function debug.sethook(...) end

---@xmake.context script
function debug.setlocal(...) end

---@xmake.context script
function debug.setmetatable(...) end

---@xmake.context script
function debug.setupvalue(...) end

---@xmake.context script
function debug.setuservalue(...) end

---@xmake.context script
function debug.traceback(...) end

---@xmake.context script
function debug.upvalueid(...) end

---@xmake.context script
function debug.upvaluejoin(...) end

---@class linuxos
local linuxos = {}

---@see https://xmake.io/api/scripts/builtin-modules/linuxos.html#linuxos-kernelver
---@xmake.context description script
function linuxos.kernelver(...) end

---@see https://xmake.io/api/scripts/builtin-modules/linuxos.html#linuxos-name
---@xmake.context description script
function linuxos.name(...) end

---@see https://xmake.io/api/scripts/builtin-modules/linuxos.html#linuxos-version
---@xmake.context description script
function linuxos.version(...) end

---@class macos
local macos = {}

---@see https://xmake.io/api/scripts/builtin-modules/macos.html#macos-version
---@xmake.context description script
function macos.version(...) end

---@class utf8
local utf8 = {}

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-byte
---@xmake.context description script
function utf8.byte(...) end

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-char
---@xmake.context description script
function utf8.char(...) end

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-codepoint
---@xmake.context description script
function utf8.codepoint(...) end

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-codes
---@xmake.context description script
function utf8.codes(...) end

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-find
---@xmake.context description script
function utf8.find(...) end

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-lastof
---@xmake.context description script
function utf8.lastof(...) end

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-len
---@xmake.context description script
function utf8.len(...) end

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-offset
---@xmake.context description script
function utf8.offset(...) end

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-reverse
---@xmake.context description script
function utf8.reverse(...) end

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-sub
---@xmake.context description script
function utf8.sub(...) end

---@xmake.context description script
function utf8.wcswidth(...) end

---@xmake.context description script
function utf8.wcwidth(...) end

---@see https://xmake.io/api/scripts/builtin-modules/utf8.html#utf8-width
---@xmake.context description script
function utf8.width(...) end

---@class utils
local utils = {}

---@xmake.context script
function utils.assert(...) end

---@see https://xmake.io/api/description/builtin-rules.html#utils-bin2c
---@xmake.context script
function utils.bin2c(...) end

---@xmake.context script
function utils.confirm(...) end

---@xmake.context script
function utils.cprint(...) end

---@xmake.context script
function utils.cprintf(...) end

---@xmake.context script
function utils.dprint(...) end

---@xmake.context script
function utils.dprintf(...) end

---@xmake.context script
function utils.dump(...) end

---@xmake.context script
function utils.error(...) end

---@xmake.context script
function utils.print(...) end

---@xmake.context script
function utils.printf(...) end

---@xmake.context import
function utils.run_script(...) end

---@xmake.context script
function utils.trycall(...) end

---@xmake.context script
function utils.vprint(...) end

---@xmake.context script
function utils.vprintf(...) end

---@xmake.context script
function utils.warning(...) end

---@xmake.context script
function utils.wprint(...) end

---@class winos
local winos = {}

---@xmake.context script
function winos.ansi_cp(...) end

---@xmake.context script
function winos.cmdargv(...) end

---@xmake.context script
function winos.console_cp(...) end

---@xmake.context script
function winos.console_output_cp(...) end

---@xmake.context script
function winos.cp_info(...) end

---@xmake.context script
function winos.file_signature(...) end

---@xmake.context script
function winos.inherit_handles_safely(...) end

---@xmake.context description script
function winos.logical_drives(...) end

---@xmake.context script
function winos.oem_cp(...) end

---@see https://xmake.io/api/scripts/builtin-modules/winos.html#winos-processes
---@xmake.context script
function winos.processes(...) end

---@see https://xmake.io/api/scripts/builtin-modules/winos.html#winos-registry-keys
---@xmake.context description script
function winos.registry_keys(...) end

---@see https://xmake.io/api/scripts/builtin-modules/winos.html#winos-registry-query
---@xmake.context description script
function winos.registry_query(...) end

---@see https://xmake.io/api/scripts/builtin-modules/winos.html#winos-registry-values
---@xmake.context description script
function winos.registry_values(...) end

---@see https://xmake.io/api/scripts/builtin-modules/winos.html#winos-set-error-mode
---@xmake.context script
function winos.set_error_mode(...) end

---@xmake.context script
function winos.short_path(...) end

---@see https://xmake.io/api/scripts/builtin-modules/winos.html#winos-version
---@xmake.context description script
function winos.version(...) end
