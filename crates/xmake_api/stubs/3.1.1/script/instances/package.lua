---@meta
---@diagnostic disable: unused-local

--- A package, given to the scripts of `package(...)` definitions.
---@class package
---@see https://xmake.io/api/description/package-dependencies.html#package
package = {}

---@return string
---@see https://xmake.io/api/scripts/package-instance.html#package-name
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
function package:is_plat(...) end

---@param ... string
---@return boolean
---@see https://xmake.io/api/scripts/package-instance.html#package-is-arch
function package:is_arch(...) end

---@return string
---@see https://xmake.io/api/scripts/package-instance.html#package-plat
function package:plat() end

---@return string
---@see https://xmake.io/api/scripts/package-instance.html#package-arch
function package:arch() end

---@return boolean
---@see https://xmake.io/api/scripts/package-instance.html#package-debug
function package:debug() end

---@param name string
---@return package?
---@see https://xmake.io/api/scripts/package-instance.html#package-dep
function package:dep(name) end

---@return string
---@see https://xmake.io/api/scripts/package-instance.html#package-cachedir
function package:cachedir() end

---@return string
function package:sourcedir() end

---@see https://xmake.io/api/scripts/package-instance.html#package-addenv
function package:addenv() end

---@see https://xmake.io/api/scripts/package-instance.html#package-alias
function package:alias() end

function package:arch_set() end

function package:base() end

function package:branch() end

function package:build_addenv() end

function package:build_envs() end

function package:build_getenv() end

function package:build_setenv() end

function package:builddir() end

---@see https://xmake.io/api/scripts/package-instance.html#package-buildhash
function package:buildhash() end

function package:buildir() end

function package:check_alignof() end

---@see https://xmake.io/api/scripts/package-instance.html#package-check-csnippets
function package:check_csnippets() end

---@see https://xmake.io/api/scripts/package-instance.html#package-check-cxxsnippets
function package:check_cxxsnippets() end

---@see https://xmake.io/api/scripts/package-instance.html#package-check-fcsnippets
function package:check_fcsnippets() end

function package:check_importfiles() end

function package:check_msnippets() end

function package:check_mxxsnippets() end

function package:check_sizeof() end

function package:commit() end

function package:compiler() end

function package:component() end

function package:components() end

function package:components_default() end

function package:components_deps() end

function package:components_orderlist() end

---@see https://xmake.io/api/scripts/package-instance.html#package-config-set
function package:config_set() end

---@see https://xmake.io/api/scripts/package-instance.html#package-configs
function package:configs() end

---@see https://xmake.io/api/scripts/package-instance.html#package-current-scheme
function package:current_scheme() end

function package:current_scheme_set() end

function package:data() end

function package:data_add() end

function package:data_set() end

---@see https://xmake.io/api/scripts/package-instance.html#package-deps
function package:deps() end

---@see https://xmake.io/api/scripts/package-instance.html#package-description
function package:description() end

function package:displayname() end

function package:displayname_set() end

---@see https://xmake.io/api/scripts/package-instance.html#package-envs
function package:envs() end

function package:envs_enter() end

function package:exists() end

function package:extraconf() end

function package:extraconf_set() end

function package:extsources() end

function package:fetch() end

function package:fetch_librarydeps() end

function package:filelock() end

function package:find_package() end

function package:find_tool() end

function package:fullname() end

---@see https://xmake.io/api/scripts/package-instance.html#package-get
function package:get() end

---@see https://xmake.io/api/scripts/package-instance.html#package-getenv
function package:getenv() end

function package:gitref() end

function package:group() end

function package:has_addon() end

function package:has_cflags() end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-cfuncs
function package:has_cfuncs() end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-cincludes
function package:has_cincludes() end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-ctypes
function package:has_ctypes() end

function package:has_cxxflags() end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-cxxfuncs
function package:has_cxxfuncs() end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-cxxincludes
function package:has_cxxincludes() end

---@see https://xmake.io/api/scripts/package-instance.html#package-has-cxxtypes
function package:has_cxxtypes() end

function package:has_features() end

function package:has_runtime() end

function package:has_source() end

function package:has_tool() end

function package:is_addon() end

function package:is_arch64() end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-binary
function package:is_binary() end

function package:is_binary_embed() end

function package:is_built() end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-cross
function package:is_cross() end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-debug
function package:is_debug() end

function package:is_fetchonly() end

function package:is_headeronly() end

function package:is_host() end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-library
function package:is_library() end

function package:is_local() end

function package:is_moduleonly() end

function package:is_optional() end

function package:is_parallelize() end

function package:is_plugin() end

function package:is_precompiled() end

function package:is_private() end

function package:is_source_embed() end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-supported
function package:is_supported() end

function package:is_system() end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-targetarch
function package:is_targetarch() end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-targetos
function package:is_targetos() end

function package:is_template() end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-thirdparty
function package:is_thirdparty() end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-toolchain
function package:is_toolchain() end

---@see https://xmake.io/api/scripts/package-instance.html#package-is-toplevel
function package:is_toplevel() end

function package:is_verify() end

---@see https://xmake.io/api/scripts/package-instance.html#package-kind
function package:kind() end

function package:label() end

function package:librarydep() end

function package:librarydeps() end

---@see https://xmake.io/api/scripts/package-instance.html#package-license
function package:license() end

function package:linker() end

function package:lock() end

function package:manifest_file() end

function package:manifest_load() end

function package:manifest_save() end

function package:mark_as_pathenv() end

function package:memcache() end

function package:mode() end

function package:namespace() end

function package:new() end

function package:orderdeps() end

function package:originfile() end

function package:originfile_set() end

function package:parents() end

function package:parents_add() end

---@see https://xmake.io/api/scripts/package-instance.html#package-patches
function package:patches() end

function package:plaindeps() end

function package:plat_set() end

function package:policy() end

function package:prepare_install_scheme() end

function package:references() end

function package:repo() end

function package:requireinfo() end

function package:requireinfo_set() end

function package:resource() end

function package:resourcedir() end

function package:resourcefile() end

function package:resources() end

function package:revision() end

function package:rulesdir() end

function package:runtimes() end

---@see https://xmake.io/api/scripts/package-instance.html#package-scheme
function package:scheme() end

function package:schemes() end

function package:schemes_orderlist() end

function package:script() end

---@see https://xmake.io/api/scripts/package-instance.html#package-scriptdir
function package:scriptdir() end

---@see https://xmake.io/api/scripts/package-instance.html#package-set
function package:set() end

---@see https://xmake.io/api/scripts/package-instance.html#package-setenv
function package:setenv() end

---@see https://xmake.io/api/scripts/package-instance.html#package-sourcehash
function package:sourcehash() end

function package:sourceinfo() end

function package:tag() end

---@see https://xmake.io/api/scripts/package-instance.html#package-targetarch
function package:targetarch() end

---@see https://xmake.io/api/scripts/package-instance.html#package-targetos
function package:targetos() end

function package:tool() end

function package:toolchain() end

function package:toolchains() end

function package:toolconfig() end

function package:type() end

function package:unlock() end

function package:url_alias() end

function package:url_excludes() end

function package:url_http_headers() end

function package:url_includes() end

function package:url_version() end

---@see https://xmake.io/api/scripts/package-instance.html#package-urls
function package:urls() end

function package:urls_set() end

function package:use_external_includes() end

function package:use_precompiled_artifacts() end

function package:version_set() end

---@see https://xmake.io/api/scripts/package-instance.html#package-versions
function package:versions() end
