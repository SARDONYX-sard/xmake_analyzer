---@meta
---@diagnostic disable: unused-local

--- A target of the project, given to the scripts of targets and rules.
---@class target
---@see https://xmake.io/api/description/project-target.html#target
target = {}

--- Returns the name of the target.
---
--- # Examples
---
--- ```lua
--- on_build(function (target)
---     print("building %s", target:name())
--- end)
--- ```
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-name
function target:name() end

--- Returns the kind of the target: `binary`, `static`, `shared`, ...
---
--- # Examples
---
--- ```lua
--- if target:kind() == "shared" then
---     target:add("defines", "BUILD_DLL")
--- end
--- ```
---@return "binary"|"static"|"shared"|"object"|"headeronly"|"moduleonly"|"phony"
---@see https://xmake.io/api/scripts/target-instance.html#target-kind
function target:kind() end

--- Returns the path of the output file.
---
--- # Examples
---
--- ```lua
--- after_build(function (target)
---     os.cp(target:targetfile(), "dist/")
--- end)
--- ```
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-targetfile
function target:targetfile() end

--- Returns the directory of the output file.
---
--- # Examples
---
--- ```lua
--- os.cp("assets", target:targetdir())
--- ```
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-targetdir
function target:targetdir() end

--- Returns the object files of the target.
---
--- # Examples
---
--- ```lua
--- for _, obj in ipairs(target:objectfiles()) do
---     print(obj)
--- end
--- ```
---@return string[]
---@see https://xmake.io/api/scripts/target-instance.html#target-objectfiles
function target:objectfiles() end

--- Returns the source files of the target.
---
--- # Examples
---
--- ```lua
--- print("%d sources", #target:sourcefiles())
--- ```
---@return string[]
function target:sourcefiles() end

---@return string[]
---@see https://xmake.io/api/scripts/target-instance.html#target-headerfiles
function target:headerfiles() end

--- Returns a directory of the build for generated files.
---
--- # Examples
---
--- ```lua
--- local header = path.join(target:autogendir(), "config.h")
--- ```
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-autogendir
function target:autogendir() end

--- Returns the directory of the `xmake.lua` defining the target.
---
--- # Examples
---
--- ```lua
--- local data = path.join(target:scriptdir(), "data")
--- ```
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-scriptdir
function target:scriptdir() end

--- Returns the install directory (`xmake install -o <dir>`).
---
--- # Examples
---
--- ```lua
--- on_install(function (target)
---     os.cp(target:targetfile(), path.join(target:installdir(), "bin"))
--- end)
--- ```
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-installdir
function target:installdir() end

--- Adds values to a setting, like the `add_*` API: `target:add("defines", "X")`.
---
--- # Examples
---
--- ```lua
--- on_load(function (target)
---     target:add("defines", "HOST_" .. os.host():upper())
---     target:add("files", "src/extra.c")
---     target:add("includedirs", "include", {public = true})
--- end)
--- ```
---
--- # Notes
---
--- Settings changed in `on_load` affect the build; later, only what has not been read yet.
---@param name string The setting, e.g. `defines`.
---@param ... any|table
---@see https://xmake.io/api/scripts/target-instance.html#target-add
function target:add(name, ...) end

--- Replaces the values of a setting, like the `set_*` API.
---
--- # Examples
---
--- ```lua
--- on_load(function (target)
---     target:set("kind", is_mode("debug") and "shared" or "static")
--- end)
--- ```
---@param name string
---@param ... any
---@see https://xmake.io/api/scripts/target-instance.html#target-set
function target:set(name, ...) end

--- Returns the values of a setting: a value or a list.
---
--- # Examples
---
--- ```lua
--- local defines = target:get("defines")
--- print(defines)
--- ```
---@param name string
---@return any
---@see https://xmake.io/api/scripts/target-instance.html#target-get
function target:get(name) end

--- Returns the dependency named `name`, a target instance.
---
--- # Examples
---
--- ```lua
--- on_config(function (target)
---     local core = target:dep("core")
---     print(core:targetfile())
--- end)
--- ```
---@param name string
---@return target?
function target:dep(name) end

--- Returns the dependencies of the target, by name.
---
--- # Examples
---
--- ```lua
--- for name, dep in pairs(target:deps()) do
---     print(name, dep:kind())
--- end
--- ```
---@return table<string, target>
function target:deps() end

---@return target[]
function target:orderdeps() end

--- Returns the package `name` the target uses, a package instance.
---
--- # Examples
---
--- ```lua
--- local zlib = target:pkg("zlib")
--- if zlib then
---     print(zlib:installdir())
--- end
--- ```
---@param name string
---@return package?
function target:pkg(name) end

--- Returns the packages the target uses, by name.
---
--- # Examples
---
--- ```lua
--- for name, pkg in pairs(target:pkgs()) do
---     print(name, pkg:version_str())
--- end
--- ```
---@return table<string, package>
function target:pkgs() end

--- Returns the platform the target is built for.
---
--- # Examples
---
--- ```lua
--- if target:plat() == "windows" then
---     target:add("syslinks", "user32")
--- end
--- ```
---@return string
function target:plat() end

--- Returns the architecture the target is built for.
---
--- # Examples
---
--- ```lua
--- print(target:arch())
--- ```
---@return string
function target:arch() end

--- Whether the platform of the target is one of the arguments.
---
--- # Examples
---
--- ```lua
--- if target:is_plat("linux", "macosx") then
---     target:add("syslinks", "pthread")
--- end
--- ```
---@param ... string
---@return boolean
---@see https://xmake.io/api/scripts/target-instance.html#target-is-plat
function target:is_plat(...) end

--- Whether the architecture of the target is one of the arguments.
---
--- # Examples
---
--- ```lua
--- if target:is_arch("arm64") then
---     target:add("defines", "ARM64")
--- end
--- ```
---@param ... string
---@return boolean
---@see https://xmake.io/api/scripts/target-instance.html#target-is-arch
function target:is_arch(...) end

---@param ... string
---@return boolean
function target:is_kind(...) end

---@param name string
---@return rule?
function target:rule(name) end

---@return toolchain[]
function target:toolchains() end

---@param kind string
---@return string program
---@return string toolname
function target:tool(kind) end

---@param name string
---@return any
function target:data(name) end

---@param name string
---@param value any
function target:data_set(name, value) end

--- Returns the file name of the output: `app.exe`, `libcore.a`, ...
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-filename
function target:filename() end

--- Returns the name of the output without prefix and extension.
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-basename
function target:basename() end

--- Returns the name to link the output with: `core` for `libcore.a`.
---@return string?
function target:linkname() end

--- Returns the version of `set_version` and its build string.
---@return string? version
---@return string? buildversion
function target:version() end

--- Returns the object file of a source file.
---
--- # Examples
---
--- ```lua
--- on_build_file(function (target, sourcefile, opt)
---     local objectfile = target:objectfile(sourcefile)
---     os.mkdir(path.directory(objectfile))
--- end)
--- ```
---@param sourcefile string
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-objectfile
function target:objectfile(sourcefile) end

--- Returns the configuration of a source file: the options of its `add_files`.
---@param sourcefile string
---@return table?
function target:fileconfig(sourcefile) end

--- Returns the options given with a value: `{public = true}`, or the
--- configuration of a rule.
---
--- # Examples
---
--- ```lua
--- -- add_rules("myrule", {name = "x"})
--- local name = target:extraconf("rules", "myrule", "name")
--- ```
---@param name string The setting: `rules`, `includedirs`, ...
---@param item? string The value of the setting.
---@param key? string The option.
---@return any
function target:extraconf(name, item, key) end

--- Returns the files `xmake install` copies and where to.
---@param outputdir? string
---@return string[] srcfiles
---@return string[] dstfiles
function target:installfiles(outputdir) end

--- Whether the target was rebuilt by the current build.
---@return boolean
function target:is_rebuilt() end

---@see https://xmake.io/api/scripts/target-instance.html#target-artifactfile
function target:artifactfile() end

function target:autogenfile() end

function target:bindir() end

function target:cachekey() end

function target:check_alignof() end

function target:check_bigendian() end

---@see https://xmake.io/api/scripts/target-instance.html#target-check-csnippets
function target:check_csnippets() end

---@see https://xmake.io/api/scripts/target-instance.html#target-check-cxxsnippets
function target:check_cxxsnippets() end

function target:check_msnippets() end

function target:check_mxxsnippets() end

---@see https://xmake.io/api/scripts/target-instance.html#target-check-sizeof
function target:check_sizeof() end

function target:clone() end

function target:compiler() end

function target:configdir() end

function target:configfiles() end

function target:data_add() end

function target:del() end

function target:dependfile() end

function target:dependfiles() end

function target:dependir() end

function target:extraconf_from() end

function target:extraconf_set() end

function target:extrafiles() end

function target:fileconfig_add() end

function target:fileconfig_set() end

function target:filerules() end

function target:fullname() end

function target:get_from() end

function target:get_from_deps() end

function target:get_from_opts() end

function target:get_from_pkgs() end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cflags
function target:has_cflags() end

function target:has_cfuncs() end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cincludes
function target:has_cincludes() end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-ctypes
function target:has_ctypes() end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cxxflags
function target:has_cxxflags() end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cxxfuncs
function target:has_cxxfuncs() end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cxxincludes
function target:has_cxxincludes() end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cxxtypes
function target:has_cxxtypes() end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-features
function target:has_features() end

function target:has_runtime() end

function target:has_sourcekind() end

function target:has_tool() end

function target:includedir() end

function target:info() end

---@see https://xmake.io/api/scripts/target-instance.html#target-is-arch64
function target:is_arch64() end

function target:is_binary() end

function target:is_cross() end

function target:is_default() end

function target:is_enabled() end

function target:is_headeronly() end

function target:is_library() end

function target:is_moduleonly() end

function target:is_object() end

function target:is_phony() end

function target:is_shared() end

function target:is_static() end

function target:libdir() end

function target:license() end

function target:linkargv() end

function target:linkcmd() end

function target:linker() end

function target:linkflags() end

function target:memcache() end

function target:name_set() end

function target:namespace() end

function target:new() end

function target:objectdir() end

function target:opt() end

function target:opts() end

function target:orderopts() end

function target:orderpkgs() end

function target:orderules() end

function target:packagedir() end

function target:pcheaderfile() end

function target:pcheaderfile_set() end

function target:pcoutputfile() end

function target:pkgconfig() end

function target:pkgenvs() end

function target:platform() end

function target:policy() end

function target:prefixdir() end

function target:remove() end

function target:rule_add() end

function target:rule_enable() end

function target:rule_is_enabled() end

function target:rules() end

function target:rundir() end

function target:runtimes() end

function target:script() end

function target:soname() end

---@see https://xmake.io/api/scripts/target-instance.html#target-sourcebatches
function target:sourcebatches() end

function target:sourcecount() end

function target:sourceinfo() end

function target:sourcekind_of() end

function target:sourcekinds() end

---@see https://xmake.io/api/scripts/target-instance.html#target-symbolfile
function target:symbolfile() end

function target:targetkind() end

function target:toolchain() end

function target:toolconfig() end

function target:type() end

function target:values() end

function target:values_add() end

function target:values_set() end
