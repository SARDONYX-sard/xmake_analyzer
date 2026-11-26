---@meta
---@diagnostic disable: unused-local

---@class xmake.package.values
local package_values = {}

function package_values.add_aicpuflags() end

---@param arflags string|string[] Static library archive option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple static library archive option strings
---@see https://xmake.io/api/description/project-target.html#add-arflags
function package_values.add_arflags(arflags, ...) end

function package_values.add_ascflags() end

---@param asflags string|string[] Assembly compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple assembly compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-asflags
function package_values.add_asflags(asflags, ...) end

---@param dirs string|string[] Executable directory path string or array
---@param ... string|string[] Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/package-dependencies.html#add-bindirs
function package_values.add_bindirs(dirs, ...) end

---@param cflags string|string[] C compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple C compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cflags
function package_values.add_cflags(cflags, ...) end

---@param components string|string[] Component name string or array
---@param ... string|string[]|{deps?: any} Variable parameters, can pass multiple component names
---@see https://xmake.io/api/description/package-dependencies.html#add-components
function package_values.add_components(components, ...) end

---@param name string Configuration parameter name
---@param opts? {description?: any, default?: any, values?: any, type?: any, readonly?: any}
---@see https://xmake.io/api/description/package-dependencies.html#add-configs
function package_values.add_configs(name, opts) end

---@param cuflags string|string[] CUDA compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cuflags
function package_values.add_cuflags(cuflags, ...) end

---@param culdflags string|string[] CUDA device link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA device link option strings
---@see https://xmake.io/api/description/project-target.html#add-culdflags
function package_values.add_culdflags(culdflags, ...) end

---@see https://xmake.io/api/description/project-target.html#add-cxflags
function package_values.add_cxflags() end

---@param cxxflags string|string[] C++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple C++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cxxflags
function package_values.add_cxxflags(cxxflags, ...) end

---@param dcflags string|string[] D language compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple D language compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-dcflags
function package_values.add_dcflags(dcflags, ...) end

---@param defines string|string[] Macro definition string or array
---@param ... string|string[] Variable parameters, can pass multiple definitions
---@see https://xmake.io/api/description/package-dependencies.html#add-defines
function package_values.add_defines(defines, ...) end

---@param deps string|string[] Dependency package name string or array
---@param ... string|string[] Variable parameters, can pass multiple dependency names
---@see https://xmake.io/api/description/package-dependencies.html#add-deps
function package_values.add_deps(deps, ...) end

---@param sources string|string[] External source string or array, format: "pkgconfig::name" or "brew::name"
---@param ... string|string[] Variable parameters, can pass multiple external sources
---@see https://xmake.io/api/description/package-dependencies.html#add-extsources
function package_values.add_extsources(sources, ...) end

---@param fcflags string|string[] Fortran compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Fortran compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-fcflags
function package_values.add_fcflags(fcflags, ...) end

---@param frameworkdirs string|string[] Framework search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple framework search directory strings
---@see https://xmake.io/api/description/project-target.html#add-frameworkdirs
function package_values.add_frameworkdirs(frameworkdirs, ...) end

---@param frameworks string|string[] Framework name string or array
---@param ... string|string[] Variable parameters, can pass multiple framework names
---@see https://xmake.io/api/description/package-dependencies.html#add-frameworks
function package_values.add_frameworks(frameworks, ...) end

---@param modules string|string[] Module name string or array
---@param ... string|string[] Variable parameters, can pass multiple module names
---@see https://xmake.io/api/description/project-target.html#add-imports
function package_values.add_imports(modules, ...) end

---@param dirs string|string[] Include directory path string or array
---@param ... string|string[] Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/package-dependencies.html#add-includedirs
function package_values.add_includedirs(dirs, ...) end

---@param ldflags string|string[] Link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple link option strings
---@see https://xmake.io/api/description/project-target.html#add-ldflags
function package_values.add_ldflags(ldflags, ...) end

---@param dirs string|string[] Link directory path string or array
---@param ... string|string[] Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/package-dependencies.html#add-linkdirs
function package_values.add_linkdirs(dirs, ...) end

---@param links string|string[] Library link name string or array
---@param ... string|string[] Variable parameters, can pass multiple link names
---@see https://xmake.io/api/description/package-dependencies.html#add-links
function package_values.add_links(links, ...) end

---@param mflags string|string[] ObjC compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mflags
function package_values.add_mflags(mflags, ...) end

---@param mxflags string|string[] ObjC/ObjC++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC/ObjC++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mxflags
function package_values.add_mxflags(mxflags, ...) end

---@param mxxflags string|string[] ObjC++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mxxflags
function package_values.add_mxxflags(mxxflags, ...) end

function package_values.add_pcflags() end

---@param rpathdirs string|string[] Runtime library search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple runtime library search directory strings
---@see https://xmake.io/api/description/project-target.html#add-rpathdirs
function package_values.add_rpathdirs(rpathdirs, ...) end

---@param schemes string|string[] Scheme name string or array
---@param ... string|string[] Variable parameters, can pass multiple scheme names
---@see https://xmake.io/api/description/package-dependencies.html#add-schemes
function package_values.add_schemes(schemes, ...) end

---@param shflags string|string[] Dynamic library link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple dynamic library link option strings
---@see https://xmake.io/api/description/project-target.html#add-shflags
function package_values.add_shflags(shflags, ...) end

---@param includedirs string|string[] System header file search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple system header file search directory strings
---@see https://xmake.io/api/description/project-target.html#add-sysincludedirs
function package_values.add_sysincludedirs(includedirs, ...) end

---@param syslinks string|string[] System library name string or array
---@param ... string|string[] Variable parameters, can pass multiple system library names
---@see https://xmake.io/api/description/package-dependencies.html#add-syslinks
function package_values.add_syslinks(syslinks, ...) end

---@param undefines string|string[] Macro definition name string or array, such as "DEBUG"
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple macro definition name strings
---@see https://xmake.io/api/description/project-target.html#add-undefines
function package_values.add_undefines(undefines, ...) end

---@param urls string|string[] Package source URL string or array
---@param ... string|string[]|{alias?: any, excludes?: any, version?: any, http_headers?: any} Variable parameters, can pass multiple URLs
---@see https://xmake.io/api/description/package-dependencies.html#add-urls
function package_values.add_urls(urls, ...) end

---@param vectorexts xmake.value.vectorext|xmake.value.vectorext[] Vector extension instruction string or array, such as "mmx", "neon", "avx"
---@param ... xmake.value.vectorext|xmake.value.vectorext[]|{private?: any} Variable parameters, can pass multiple vector extension instruction strings
---@see https://xmake.io/api/description/project-target.html#add-vectorexts
function package_values.add_vectorexts(vectorexts, ...) end

---@param zcflags string|string[] Zig compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Zig compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-zcflags
function package_values.add_zcflags(zcflags, ...) end

---@param arch xmake.value.arch Architecture name string
---@param ... xmake.value.arch Variable parameters, can pass multiple architecture names
---@see https://xmake.io/api/description/conditions.html#is-arch
function package_values.is_arch(arch, ...) end

---@param host xmake.value.platform Host system name string
---@param ... xmake.value.platform Variable parameters, can pass multiple host names
---@see https://xmake.io/api/description/conditions.html#is-host
function package_values.is_host(host, ...) end

---@param mode xmake.value.mode Compilation mode name string
---@param ... xmake.value.mode Variable parameters, can pass multiple mode names
---@see https://xmake.io/api/description/conditions.html#is-mode
function package_values.is_mode(mode, ...) end

---@param plat xmake.value.platform Platform name string
---@param ... xmake.value.platform Variable parameters, can pass multiple platform names
---@see https://xmake.io/api/description/conditions.html#is-plat
function package_values.is_plat(plat, ...) end

---@param arch xmake.value.arch Architecture name string, such as "x86", "x64", "arm64", "armv7"
---@see https://xmake.io/api/description/project-target.html#set-arch
function package_values.set_arch(arch) end

---@param package string Base package name to inherit from
---@see https://xmake.io/api/description/package-dependencies.html#set-base
function package_values.set_base(package) end

function package_values.set_cachedir() end

---@param description string Package description string
---@see https://xmake.io/api/description/package-dependencies.html#set-description
function package_values.set_description(description) end

---@param url string Package homepage URL string
---@see https://xmake.io/api/description/package-dependencies.html#set-homepage
function package_values.set_homepage(url) end

---@param installdir string Installation directory path string
---@see https://xmake.io/api/description/project-target.html#set-installdir
function package_values.set_installdir(installdir) end

function package_values.set_installtips() end

---@param kind "library"|"binary"|"toolchain" Package type: "library", "binary", "toolchain"
---@param opts? {headeronly?: any}
---@see https://xmake.io/api/description/package-dependencies.html#set-kind
function package_values.set_kind(kind, opts) end

---@param license string License name string
---@see https://xmake.io/api/description/xpack-interfaces.html#set-license
function package_values.set_license(license) end

function package_values.set_parallelize() end

---@param plat xmake.value.platform Platform name string, such as "linux", "macosx", "windows", "android"
---@see https://xmake.io/api/description/project-target.html#set-plat
function package_values.set_plat(plat) end

---@param sourcedir string Package source directory path
---@see https://xmake.io/api/description/package-dependencies.html#set-sourcedir
function package_values.set_sourcedir(sourcedir) end

---@param urls string|string[] Package source URL string or array
---@param ... string|string[]|{excludes?: any, version?: any, http_headers?: any} Variable parameters, can pass multiple URLs
---@see https://xmake.io/api/description/package-dependencies.html#set-urls
function package_values.set_urls(urls, ...) end

---@class xmake.package.groups
local package_groups = {}

---@param groups string|string[] Link group name string or array
---@param ... string|string[]|{name?: any, group?: any} Variable parameters, can pass multiple group names
---@see https://xmake.io/api/description/package-dependencies.html#add-linkgroups
function package_groups.add_linkgroups(groups, ...) end

---@param orders string|string[] Link order string or array
---@param ... string|string[] Variable parameters, can pass multiple order specifications
---@see https://xmake.io/api/description/package-dependencies.html#add-linkorders
function package_groups.add_linkorders(orders, ...) end

---@class xmake.package.keyvalues
local package_keyvalues = {}

---@param version string Package version for which patch applies
---@param url string Patch file URL
---@param hash string SHA256 hash value for patch verification
---@see https://xmake.io/api/description/package-dependencies.html#add-patches
function package_keyvalues.add_patches(version, url, hash) end

function package_keyvalues.add_resources() end

---@param policy xmake.value.policy Policy name string, such as "check.auto_ignore_flags", "build.warning"
---@param value boolean Policy value, true means enable, false means disable
---@see https://xmake.io/api/description/project-target.html#set-policy
function package_keyvalues.set_policy(policy, value) end

---@class xmake.package.paths
local package_paths = {}

---@param file string Version file path containing version and hash pairs
---@see https://xmake.io/api/description/package-dependencies.html#add-versionfiles
function package_paths.add_versionfiles(file) end

---@class xmake.package.dictionary
local package_dictionary = {}

---@param version string Package version string
---@param hash string SHA256 hash value for verification
---@see https://xmake.io/api/description/package-dependencies.html#add-versions
function package_dictionary.add_versions(version, hash) end

---@class xmake.package.script
local package_script = {}

---@param script fun(package: package)|string Check script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-check
function package_script.on_check(script) end

---@param component string Component name string, optional (if not provided, applies to all components)
---@param script fun(package: package, component: any)|string Component configuration script function with package and component parameters
---@see https://xmake.io/api/description/package-dependencies.html#on-component
function package_script.on_component(component, script) end

---@param script fun(package: package, opt: any)|string Download script function with package and opt parameters
---@see https://xmake.io/api/description/package-dependencies.html#on-download
function package_script.on_download(script) end

---@param script fun(package: package, opt: any)|string Fetch script function with package and opt parameters
---@see https://xmake.io/api/description/package-dependencies.html#on-fetch
function package_script.on_fetch(script) end

---@param script fun(package: package)|string Install script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-install
function package_script.on_install(script) end

---@param script fun(package: package)|string Package load script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-load
function package_script.on_load(script) end

function package_script.on_source() end

---@param script fun(package: package)|string Test script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-test
function package_script.on_test(script) end
