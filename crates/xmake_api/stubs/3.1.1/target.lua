---@meta

--- A target of the project, given to the scripts of targets and rules.
---@class target
---@see https://xmake.io/api/description/project-target.html#target
local target = {}

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
---@xmake.context instance
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
---@xmake.context instance
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
---@xmake.context instance
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
---@xmake.context instance
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
---@xmake.context instance
function target:objectfiles() end

--- Returns the source files of the target.
---
--- # Examples
---
--- ```lua
--- print("%d sources", #target:sourcefiles())
--- ```
---@return string[]
---@xmake.context instance
function target:sourcefiles() end

---@return string[]
---@see https://xmake.io/api/scripts/target-instance.html#target-headerfiles
---@xmake.context instance
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
---@xmake.context instance
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
---@xmake.context instance
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
---@xmake.context instance
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
---@param ... any
---@param opt? table
---@see https://xmake.io/api/scripts/target-instance.html#target-add
---@xmake.context instance
function target:add(name, ..., opt) end

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
---@xmake.context instance
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
---@xmake.context instance
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
---@xmake.context instance
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
---@xmake.context instance
function target:deps() end

---@return target[]
---@xmake.context instance
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
---@xmake.context instance
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
---@xmake.context instance
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
---@xmake.context instance
function target:plat() end

--- Returns the architecture the target is built for.
---
--- # Examples
---
--- ```lua
--- print(target:arch())
--- ```
---@return string
---@xmake.context instance
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
---@xmake.context instance
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
---@xmake.context instance
function target:is_arch(...) end

---@param ... string
---@return boolean
function target:is_kind(...) end

---@param name string
---@return rule?
---@xmake.context instance
function target:rule(name) end

---@return toolchain[]
---@xmake.context instance
function target:toolchains() end

---@param kind string
---@return string program
---@return string toolname
---@xmake.context instance
function target:tool(kind) end

---@param name string
---@return any
---@xmake.context instance
function target:data(name) end

---@param name string
---@param value any
---@xmake.context instance
function target:data_set(name, value) end

--- Returns the file name of the output: `app.exe`, `libcore.a`, ...
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-filename
---@xmake.context instance
function target:filename() end

--- Returns the name of the output without prefix and extension.
---@return string
---@see https://xmake.io/api/scripts/target-instance.html#target-basename
---@xmake.context instance
function target:basename() end

--- Returns the name to link the output with: `core` for `libcore.a`.
---@return string?
---@xmake.context instance
function target:linkname() end

--- Returns the version of `set_version` and its build string.
---@return string? version
---@return string? buildversion
---@xmake.context instance
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
---@xmake.context instance
function target:objectfile(sourcefile) end

--- Returns the configuration of a source file: the options of its `add_files`.
---@param sourcefile string
---@return table?
---@xmake.context instance
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
---@xmake.context instance
function target:extraconf(name, item, key) end

--- Returns the files `xmake install` copies and where to.
---@param outputdir? string
---@return string[] srcfiles
---@return string[] dstfiles
---@xmake.context instance
function target:installfiles(outputdir) end

--- Whether the target was rebuilt by the current build.
---@return boolean
---@xmake.context instance
function target:is_rebuilt() end

---@see https://xmake.io/api/scripts/target-instance.html#target-artifactfile
---@xmake.context instance
function target:artifactfile(...) end

---@xmake.context instance
function target:autogenfile(...) end

---@xmake.context instance
function target:bindir(...) end

---@xmake.context instance
function target:cachekey(...) end

---@xmake.context instance
function target:check_alignof(...) end

---@xmake.context instance
function target:check_bigendian(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-check-csnippets
---@xmake.context instance
function target:check_csnippets(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-check-cxxsnippets
---@xmake.context instance
function target:check_cxxsnippets(...) end

---@xmake.context instance
function target:check_msnippets(...) end

---@xmake.context instance
function target:check_mxxsnippets(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-check-sizeof
---@xmake.context instance
function target:check_sizeof(...) end

---@xmake.context instance
function target:clone(...) end

---@xmake.context instance
function target:compiler(...) end

---@xmake.context instance
function target:configdir(...) end

---@xmake.context instance
function target:configfiles(...) end

---@xmake.context instance
function target:data_add(...) end

---@xmake.context instance
function target:del(...) end

---@xmake.context instance
function target:dependfile(...) end

---@xmake.context instance
function target:dependfiles(...) end

---@xmake.context instance
function target:dependir(...) end

---@xmake.context instance
function target:extraconf_from(...) end

---@xmake.context instance
function target:extraconf_set(...) end

---@xmake.context instance
function target:extrafiles(...) end

---@xmake.context instance
function target:fileconfig_add(...) end

---@xmake.context instance
function target:fileconfig_set(...) end

---@xmake.context instance
function target:filerules(...) end

---@xmake.context instance
function target:fullname(...) end

---@xmake.context instance
function target:get_from(...) end

---@xmake.context instance
function target:get_from_deps(...) end

---@xmake.context instance
function target:get_from_opts(...) end

---@xmake.context instance
function target:get_from_pkgs(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cflags
---@xmake.context instance
function target:has_cflags(...) end

---@xmake.context instance
function target:has_cfuncs(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cincludes
---@xmake.context instance
function target:has_cincludes(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-ctypes
---@xmake.context instance
function target:has_ctypes(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cxxflags
---@xmake.context instance
function target:has_cxxflags(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cxxfuncs
---@xmake.context instance
function target:has_cxxfuncs(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cxxincludes
---@xmake.context instance
function target:has_cxxincludes(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-cxxtypes
---@xmake.context instance
function target:has_cxxtypes(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-has-features
---@xmake.context instance
function target:has_features(...) end

---@xmake.context instance
function target:has_runtime(...) end

---@xmake.context instance
function target:has_sourcekind(...) end

---@xmake.context instance
function target:has_tool(...) end

---@xmake.context instance
function target:includedir(...) end

---@xmake.context instance
function target:info(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-is-arch64
---@xmake.context instance
function target:is_arch64(...) end

---@xmake.context instance
function target:is_binary(...) end

---@xmake.context instance
function target:is_cross(...) end

---@xmake.context instance
function target:is_default(...) end

---@xmake.context instance
function target:is_enabled(...) end

---@xmake.context instance
function target:is_headeronly(...) end

---@xmake.context instance
function target:is_library(...) end

---@xmake.context instance
function target:is_moduleonly(...) end

---@xmake.context instance
function target:is_object(...) end

---@xmake.context instance
function target:is_phony(...) end

---@xmake.context instance
function target:is_shared(...) end

---@xmake.context instance
function target:is_static(...) end

---@xmake.context instance
function target:libdir(...) end

---@xmake.context instance
function target:license(...) end

---@xmake.context instance
function target:linkargv(...) end

---@xmake.context instance
function target:linkcmd(...) end

---@xmake.context instance
function target:linker(...) end

---@xmake.context instance
function target:linkflags(...) end

---@xmake.context instance
function target:memcache(...) end

---@xmake.context instance
function target:name_set(...) end

---@xmake.context instance
function target:namespace(...) end

---@xmake.context instance
function target:new(...) end

---@xmake.context instance
function target:objectdir(...) end

---@xmake.context instance
function target:opt(...) end

---@xmake.context instance
function target:opts(...) end

---@xmake.context instance
function target:orderopts(...) end

---@xmake.context instance
function target:orderpkgs(...) end

---@xmake.context instance
function target:orderules(...) end

---@xmake.context instance
function target:packagedir(...) end

---@xmake.context instance
function target:pcheaderfile(...) end

---@xmake.context instance
function target:pcheaderfile_set(...) end

---@xmake.context instance
function target:pcoutputfile(...) end

---@xmake.context instance
function target:pkgconfig(...) end

---@xmake.context instance
function target:pkgenvs(...) end

---@xmake.context instance
function target:platform(...) end

---@xmake.context instance
function target:policy(...) end

---@xmake.context instance
function target:prefixdir(...) end

---@xmake.context instance
function target:remove(...) end

---@xmake.context instance
function target:rule_add(...) end

---@xmake.context instance
function target:rule_enable(...) end

---@xmake.context instance
function target:rule_is_enabled(...) end

---@xmake.context instance
function target:rules(...) end

---@xmake.context instance
function target:rundir(...) end

---@xmake.context instance
function target:runtimes(...) end

---@xmake.context instance
function target:script(...) end

---@xmake.context instance
function target:soname(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-sourcebatches
---@xmake.context instance
function target:sourcebatches(...) end

---@xmake.context instance
function target:sourcecount(...) end

---@xmake.context instance
function target:sourceinfo(...) end

---@xmake.context instance
function target:sourcekind_of(...) end

---@xmake.context instance
function target:sourcekinds(...) end

---@see https://xmake.io/api/scripts/target-instance.html#target-symbolfile
---@xmake.context instance
function target:symbolfile(...) end

---@xmake.context instance
function target:targetkind(...) end

---@xmake.context instance
function target:toolchain(...) end

---@xmake.context instance
function target:toolconfig(...) end

---@xmake.context instance
function target:type(...) end

---@xmake.context instance
function target:values(...) end

---@xmake.context instance
function target:values_add(...) end

---@xmake.context instance
function target:values_set(...) end
