---@meta

--- A package, given to the scripts of `package(...)` definitions.
---@class package
---@see https://xmake.io/api/description/package-dependencies.html#package
local package = {}

---@return string
---@see https://xmake.io/api/scripts/package-instance.html#package-name
---@xmake.context instance
function package:name() end

--- Returns the install directory of the package, or one of its subdirectories.
---
--- # Examples
---
--- ```lua
--- on_install(function (package)
---     os.cp("include/*.h", package:installdir("include"))
---     os.cp("lib/*.a", package:installdir("lib"))
--- end)
--- ```
---@param ... string Subdirectories.
---@return string
---@see https://xmake.io/api/scripts/package-instance.html#package-installdir
---@xmake.context instance
function package:installdir(...) end

--- Returns the version of the package, a semver object to compare.
---
--- # Examples
---
--- ```lua
--- if package:version():ge("2.0") then
---     package:add("defines", "MYLIB_V2")
--- end
--- ```
---@return semver
---@see https://xmake.io/api/scripts/package-instance.html#package-version
---@xmake.context instance
function package:version() end

--- Returns the version of the package as a string.
---
--- # Examples
---
--- ```lua
--- print(package:version_str()) -- 1.2.13
--- ```
---@return string
---@see https://xmake.io/api/scripts/package-instance.html#package-version-str
---@xmake.context instance
function package:version_str() end

--- Returns a configuration of the package, as set by `add_requires(..., {configs = {...}})`.
---
--- # Examples
---
--- ```lua
--- on_install(function (package)
---     local configs = {"-DBUILD_SHARED_LIBS=" .. (package:config("shared") and "ON" or "OFF")}
---     import("package.tools.cmake").install(package, configs)
--- end)
--- ```
---@param name string
---@return any
---@see https://xmake.io/api/scripts/package-instance.html#package-config
---@xmake.context instance
function package:config(name) end

--- Adds values to a setting of the package for its users: `links`, `defines`, ...
---
--- # Examples
---
--- ```lua
--- on_load(function (package)
---     if not package:config("shared") then
---         package:add("defines", "MYLIB_STATIC")
---     end
--- end)
--- ```
---@param name string
---@param ... any
---@see https://xmake.io/api/scripts/package-instance.html#package-add
---@xmake.context instance
function package:add(name, ...) end

--- Whether the platform of the package is one of the arguments.
---
--- # Examples
---
--- ```lua
--- if package:is_plat("windows") then
---     package:add("syslinks", "advapi32")
--- end
--- ```
---@param ... string
---@return boolean
---@see https://xmake.io/api/scripts/package-instance.html#package-is-plat
---@xmake.context instance
function package:is_plat(...) end

---@param ... string
---@return boolean
---@see https://xmake.io/api/scripts/package-instance.html#package-is-arch
---@xmake.context instance
function package:is_arch(...) end

---@return string
---@see https://xmake.io/api/scripts/package-instance.html#package-plat
---@xmake.context instance
function package:plat() end

---@return string
---@see https://xmake.io/api/scripts/package-instance.html#package-arch
---@xmake.context instance
function package:arch() end

---@return boolean
---@see https://xmake.io/api/scripts/package-instance.html#package-debug
---@xmake.context instance
function package:debug() end

---@param name string
---@return package?
---@see https://xmake.io/api/scripts/package-instance.html#package-dep
---@xmake.context instance
function package:dep(name) end

---@return string
---@see https://xmake.io/api/scripts/package-instance.html#package-cachedir
---@xmake.context instance
function package:cachedir() end

---@return string
---@xmake.context instance
function package:sourcedir() end

---@see https://xmake.io/api/scripts/package-instance.html#package-addenv
---@xmake.context instance
function package:addenv(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-alias
---@xmake.context instance
function package:alias(...) end

---@xmake.context instance
function package:arch_set(...) end

---@xmake.context instance
function package:base(...) end

---@xmake.context instance
function package:branch(...) end

---@xmake.context instance
function package:build_addenv(...) end

---@xmake.context instance
function package:build_envs(...) end

---@xmake.context instance
function package:build_getenv(...) end

---@xmake.context instance
function package:build_setenv(...) end

---@xmake.context instance
function package:builddir(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-buildhash
---@xmake.context instance
function package:buildhash(...) end

---@xmake.context instance
function package:buildir(...) end

---@xmake.context instance
function package:check_alignof(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-check-csnippets
---@xmake.context instance
function package:check_csnippets(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-check-cxxsnippets
---@xmake.context instance
function package:check_cxxsnippets(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-check-fcsnippets
---@xmake.context instance
function package:check_fcsnippets(...) end

---@xmake.context instance
function package:check_importfiles(...) end

---@xmake.context instance
function package:check_msnippets(...) end

---@xmake.context instance
function package:check_mxxsnippets(...) end

---@xmake.context instance
function package:check_sizeof(...) end

---@xmake.context instance
function package:commit(...) end

---@xmake.context instance
function package:compiler(...) end

---@xmake.context instance
function package:component(...) end

---@xmake.context instance
function package:components(...) end

---@xmake.context instance
function package:components_default(...) end

---@xmake.context instance
function package:components_deps(...) end

---@xmake.context instance
function package:components_orderlist(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-config-set
---@xmake.context instance
function package:config_set(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-configs
---@xmake.context instance
function package:configs(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-current-scheme
---@xmake.context instance
function package:current_scheme(...) end

---@xmake.context instance
function package:current_scheme_set(...) end

---@xmake.context instance
function package:data(...) end

---@xmake.context instance
function package:data_add(...) end

---@xmake.context instance
function package:data_set(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-deps
---@xmake.context instance
function package:deps(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-description
---@xmake.context instance
function package:description(...) end

---@xmake.context instance
function package:displayname(...) end

---@xmake.context instance
function package:displayname_set(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-envs
---@xmake.context instance
function package:envs(...) end

---@xmake.context instance
function package:envs_enter(...) end

---@xmake.context instance
function package:exists(...) end

---@xmake.context instance
function package:extraconf(...) end

---@xmake.context instance
function package:extraconf_set(...) end

---@xmake.context instance
function package:extsources(...) end

---@xmake.context instance
function package:fetch(...) end

---@xmake.context instance
function package:fetch_librarydeps(...) end

---@xmake.context instance
function package:filelock(...) end

---@xmake.context instance
function package:find_package(...) end

---@xmake.context instance
function package:find_tool(...) end

---@xmake.context instance
function package:fullname(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-get
---@xmake.context instance
function package:get(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-getenv
---@xmake.context instance
function package:getenv(...) end

---@xmake.context instance
function package:gitref(...) end

---@xmake.context instance
function package:group(...) end

---@xmake.context instance
function package:has_addon(...) end

---@xmake.context instance
function package:has_cflags(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-cfuncs
---@xmake.context instance
function package:has_cfuncs(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-cincludes
---@xmake.context instance
function package:has_cincludes(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-ctypes
---@xmake.context instance
function package:has_ctypes(...) end

---@xmake.context instance
function package:has_cxxflags(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-cxxfuncs
---@xmake.context instance
function package:has_cxxfuncs(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-cxxincludes
---@xmake.context instance
function package:has_cxxincludes(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-cxxtypes
---@xmake.context instance
function package:has_cxxtypes(...) end

---@xmake.context instance
function package:has_features(...) end

---@xmake.context instance
function package:has_runtime(...) end

---@xmake.context instance
function package:has_source(...) end

---@xmake.context instance
function package:has_tool(...) end

---@xmake.context instance
function package:is_addon(...) end

---@xmake.context instance
function package:is_arch64(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-binary
---@xmake.context instance
function package:is_binary(...) end

---@xmake.context instance
function package:is_binary_embed(...) end

---@xmake.context instance
function package:is_built(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-cross
---@xmake.context instance
function package:is_cross(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-debug
---@xmake.context instance
function package:is_debug(...) end

---@xmake.context instance
function package:is_fetchonly(...) end

---@xmake.context instance
function package:is_headeronly(...) end

---@xmake.context instance
function package:is_host(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-library
---@xmake.context instance
function package:is_library(...) end

---@xmake.context instance
function package:is_local(...) end

---@xmake.context instance
function package:is_moduleonly(...) end

---@xmake.context instance
function package:is_optional(...) end

---@xmake.context instance
function package:is_parallelize(...) end

---@xmake.context instance
function package:is_plugin(...) end

---@xmake.context instance
function package:is_precompiled(...) end

---@xmake.context instance
function package:is_private(...) end

---@xmake.context instance
function package:is_source_embed(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-supported
---@xmake.context instance
function package:is_supported(...) end

---@xmake.context instance
function package:is_system(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-targetarch
---@xmake.context instance
function package:is_targetarch(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-targetos
---@xmake.context instance
function package:is_targetos(...) end

---@xmake.context instance
function package:is_template(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-thirdparty
---@xmake.context instance
function package:is_thirdparty(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-toolchain
---@xmake.context instance
function package:is_toolchain(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-toplevel
---@xmake.context instance
function package:is_toplevel(...) end

---@xmake.context instance
function package:is_verify(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-kind
---@xmake.context instance
function package:kind(...) end

---@xmake.context instance
function package:label(...) end

---@xmake.context instance
function package:librarydep(...) end

---@xmake.context instance
function package:librarydeps(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-license
---@xmake.context instance
function package:license(...) end

---@xmake.context instance
function package:linker(...) end

---@xmake.context instance
function package:lock(...) end

---@xmake.context instance
function package:manifest_file(...) end

---@xmake.context instance
function package:manifest_load(...) end

---@xmake.context instance
function package:manifest_save(...) end

---@xmake.context instance
function package:mark_as_pathenv(...) end

---@xmake.context instance
function package:memcache(...) end

---@xmake.context instance
function package:mode(...) end

---@xmake.context instance
function package:namespace(...) end

---@xmake.context instance
function package:new(...) end

---@xmake.context instance
function package:orderdeps(...) end

---@xmake.context instance
function package:originfile(...) end

---@xmake.context instance
function package:originfile_set(...) end

---@xmake.context instance
function package:parents(...) end

---@xmake.context instance
function package:parents_add(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-patches
---@xmake.context instance
function package:patches(...) end

---@xmake.context instance
function package:plaindeps(...) end

---@xmake.context instance
function package:plat_set(...) end

---@xmake.context instance
function package:policy(...) end

---@xmake.context instance
function package:prepare_install_scheme(...) end

---@xmake.context instance
function package:references(...) end

---@xmake.context instance
function package:repo(...) end

---@xmake.context instance
function package:requireinfo(...) end

---@xmake.context instance
function package:requireinfo_set(...) end

---@xmake.context instance
function package:resource(...) end

---@xmake.context instance
function package:resourcedir(...) end

---@xmake.context instance
function package:resourcefile(...) end

---@xmake.context instance
function package:resources(...) end

---@xmake.context instance
function package:revision(...) end

---@xmake.context instance
function package:rulesdir(...) end

---@xmake.context instance
function package:runtimes(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-scheme
---@xmake.context instance
function package:scheme(...) end

---@xmake.context instance
function package:schemes(...) end

---@xmake.context instance
function package:schemes_orderlist(...) end

---@xmake.context instance
function package:script(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-scriptdir
---@xmake.context instance
function package:scriptdir(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-set
---@xmake.context instance
function package:set(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-setenv
---@xmake.context instance
function package:setenv(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-sourcehash
---@xmake.context instance
function package:sourcehash(...) end

---@xmake.context instance
function package:sourceinfo(...) end

---@xmake.context instance
function package:tag(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-targetarch
---@xmake.context instance
function package:targetarch(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-targetos
---@xmake.context instance
function package:targetos(...) end

---@xmake.context instance
function package:tool(...) end

---@xmake.context instance
function package:toolchain(...) end

---@xmake.context instance
function package:toolchains(...) end

---@xmake.context instance
function package:toolconfig(...) end

---@xmake.context instance
function package:type(...) end

---@xmake.context instance
function package:unlock(...) end

---@xmake.context instance
function package:url_alias(...) end

---@xmake.context instance
function package:url_excludes(...) end

---@xmake.context instance
function package:url_http_headers(...) end

---@xmake.context instance
function package:url_includes(...) end

---@xmake.context instance
function package:url_version(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-urls
---@xmake.context instance
function package:urls(...) end

---@xmake.context instance
function package:urls_set(...) end

---@xmake.context instance
function package:use_external_includes(...) end

---@xmake.context instance
function package:use_precompiled_artifacts(...) end

---@xmake.context instance
function package:version_set(...) end

---@see https://xmake.io/api/scripts/package-instance.html#package-versions
---@xmake.context instance
function package:versions(...) end

