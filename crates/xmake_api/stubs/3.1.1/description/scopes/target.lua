---@meta
---@diagnostic disable: unused-local

---@class xmake.target.values
local target_values = {}

function target_values.add_aicpuflags() end

---@param arflags string|string[] Static library archive option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple static library archive option strings
---@see https://xmake.io/api/description/project-target.html#add-arflags
function target_values.add_arflags(arflags, ...) end

function target_values.add_ascflags() end

function target_values.add_ascnpuarchs() end

---@param asflags string|string[] Assembly compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple assembly compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-asflags
function target_values.add_asflags(asflags, ...) end

---@param cflags string|string[] C compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple C compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cflags
function target_values.add_cflags(cflags, ...) end

function target_values.add_csflags() end

---@param cuflags string|string[] CUDA compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cuflags
function target_values.add_cuflags(cuflags, ...) end

---@param cugencodes string|string[] CUDA device gencode setting string or array, such as "sm_30", "sm_50"
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA device gencode setting strings
---@see https://xmake.io/api/description/project-target.html#add-cugencodes
function target_values.add_cugencodes(cugencodes, ...) end

---@param culdflags string|string[] CUDA device link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA device link option strings
---@see https://xmake.io/api/description/project-target.html#add-culdflags
function target_values.add_culdflags(culdflags, ...) end

---@see https://xmake.io/api/description/project-target.html#add-cxflags
function target_values.add_cxflags() end

---@param cxxflags string|string[] C++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple C++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cxxflags
function target_values.add_cxxflags(cxxflags, ...) end

---@param dcflags string|string[] D language compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple D language compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-dcflags
function target_values.add_dcflags(dcflags, ...) end

---@param defines string|string[] Macro definition string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple macro definition strings
---@see https://xmake.io/api/description/project-target.html#add-defines
function target_values.add_defines(defines, ...) end

---@param deps xmake.value.target|xmake.value.target[] Dependency target name string or array, supports wildcard matching patterns
---@param ... xmake.value.target|xmake.value.target[]|{inherit?: any} Variable parameters, can pass multiple dependency target name strings
---@see https://xmake.io/api/description/project-target.html#add-deps
function target_values.add_deps(deps, ...) end

---@param fcflags string|string[] Fortran compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Fortran compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-fcflags
function target_values.add_fcflags(fcflags, ...) end

---@param name string Group name string, such as "source", "header", "test"
---@param files string|string[] Source file path string or array, supports wildcard matching patterns
---@param ... string|string[] Variable parameters, can pass multiple source file path strings
---@see https://xmake.io/api/description/project-target.html#add-filegroups
function target_values.add_filegroups(name, files, ...) end

---@param includes string|string[] Header file path string or array, such as "config.h"
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple header file path strings
---@see https://xmake.io/api/description/project-target.html#add-forceincludes
function target_values.add_forceincludes(includes, ...) end

---@param frameworks string|string[] Framework name string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple framework name strings
---@see https://xmake.io/api/description/project-target.html#add-frameworks
function target_values.add_frameworks(frameworks, ...) end

---@param gcflags string|string[] Go compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Go compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-gcflags
function target_values.add_gcflags(gcflags, ...) end

---@param modules string|string[] Module name string or array
---@param ... string|string[] Variable parameters, can pass multiple module names
---@see https://xmake.io/api/description/project-target.html#add-imports
function target_values.add_imports(modules, ...) end

function target_values.add_kcflags() end

---@param languages string|string[] Language standard string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple language standard strings
---@see https://xmake.io/api/description/project-target.html#add-languages
function target_values.add_languages(languages, ...) end

---@param ldflags string|string[] Link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple link option strings
---@see https://xmake.io/api/description/project-target.html#add-ldflags
function target_values.add_ldflags(ldflags, ...) end

---@param links string|string[] Link library name string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple link library name strings
---@see https://xmake.io/api/description/project-target.html#add-links
function target_values.add_links(links, ...) end

---@param mflags string|string[] ObjC compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mflags
function target_values.add_mflags(mflags, ...) end

function target_values.add_mrcflags() end

---@param mxflags string|string[] ObjC/ObjC++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC/ObjC++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mxflags
function target_values.add_mxflags(mxflags, ...) end

---@param mxxflags string|string[] ObjC++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mxxflags
function target_values.add_mxxflags(mxxflags, ...) end

function target_values.add_ncflags() end

---@see https://xmake.io/api/description/project-target.html#add-options
function target_values.add_options() end

---@param packages xmake.value.package|xmake.value.package[] Package name string or array, supports wildcard matching patterns
---@param ... xmake.value.package|xmake.value.package[]|{private?: any} Variable parameters, can pass multiple package name strings
---@see https://xmake.io/api/description/project-target.html#add-packages
function target_values.add_packages(packages, ...) end

function target_values.add_pcflags() end

---@param rcflags string|string[] Rust compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Rust compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-rcflags
function target_values.add_rcflags(rcflags, ...) end

---@param rpathdirs string|string[] Runtime library search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple runtime library search directory strings
---@see https://xmake.io/api/description/project-target.html#add-rpathdirs
function target_values.add_rpathdirs(rpathdirs, ...) end

---@param rules xmake.value.rule|xmake.value.rule[] Rule name string or array, such as "markdown"
---@param ... xmake.value.rule|xmake.value.rule[]|{private?: any} Variable parameters, can pass multiple rule name strings
---@see https://xmake.io/api/description/project-target.html#add-rules
function target_values.add_rules(rules, ...) end

---@param scflags string|string[] Swift compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Swift compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-scflags
function target_values.add_scflags(scflags, ...) end

---@param shflags string|string[] Dynamic library link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple dynamic library link option strings
---@see https://xmake.io/api/description/project-target.html#add-shflags
function target_values.add_shflags(shflags, ...) end

---@param syslinks string|string[] System link library name string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple system link library name strings
---@see https://xmake.io/api/description/project-target.html#add-syslinks
function target_values.add_syslinks(syslinks, ...) end

---@param tests string|string[] Test case name string or array, such as "test1", "test2"
---@param ... string|string[]|{runargs?: any, runenvs?: any, timeout?: any} Variable parameters, can pass multiple test case name strings
---@see https://xmake.io/api/description/project-target.html#add-tests
function target_values.add_tests(tests, ...) end

function target_values.add_toolchains() end

---@param undefines string|string[] Macro definition name string or array, such as "DEBUG"
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple macro definition name strings
---@see https://xmake.io/api/description/project-target.html#add-undefines
function target_values.add_undefines(undefines, ...) end

---@param vectorexts string|string[] Vector extension instruction string or array, such as "mmx", "neon", "avx"
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple vector extension instruction strings
---@see https://xmake.io/api/description/project-target.html#add-vectorexts
function target_values.add_vectorexts(vectorexts, ...) end

---@param zcflags string|string[] Zig compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Zig compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-zcflags
function target_values.add_zcflags(zcflags, ...) end

---@param arch xmake.value.arch Architecture name string, such as "x86", "x64", "arm64", "armv7"
---@see https://xmake.io/api/description/project-target.html#set-arch
function target_values.set_arch(arch) end

---@param basename string Target file base name string
---@see https://xmake.io/api/description/project-target.html#set-basename
function target_values.set_basename(basename) end

---@param default boolean Whether to mark as default build target, true means default build, false means not default build
---@see https://xmake.io/api/description/project-target.html#set-default
function target_values.set_default(default) end

---@param enabled boolean Whether to enable the target, true means enabled, false means disabled
---@see https://xmake.io/api/description/project-target.html#set-enabled
function target_values.set_enabled(enabled) end

---@param encodings xmake.value.encoding|xmake.value.encoding[] Encoding string or array, such as "utf-8", "gb2312"
---@param ... xmake.value.encoding|xmake.value.encoding[] Variable parameters, can pass multiple encoding strings
---@see https://xmake.io/api/description/project-target.html#set-encodings
function target_values.set_encodings(encodings, ...) end

---@param exceptions xmake.value.exception|xmake.value.exception[] Exception mode string or array, such as "cxx", "objc", "no-cxx"
---@param ... xmake.value.exception|xmake.value.exception[] Variable parameters, can pass multiple exception mode strings
---@see https://xmake.io/api/description/project-target.html#set-exceptions
function target_values.set_exceptions(exceptions, ...) end

---@param extension string Target file extension string, such as ".dll" or ".so"
---@see https://xmake.io/api/description/project-target.html#set-extension
function target_values.set_extension(extension) end

---@param filename string Target file full name string, including prefix and suffix
---@see https://xmake.io/api/description/project-target.html#set-filename
function target_values.set_filename(filename) end

---@param fpmodels xmake.value.fpmodel|xmake.value.fpmodel[] Float-point mode string or array, optional values: fast, strict, except, precise
---@param ... xmake.value.fpmodel|xmake.value.fpmodel[]|{private?: any} Variable parameters, can pass multiple float-point mode strings
---@see https://xmake.io/api/description/project-target.html#set-fpmodels
function target_values.set_fpmodels(fpmodels, ...) end

---@param group string Group name string, such as "test", "libs", "tools"
---@see https://xmake.io/api/description/project-target.html#set-group
function target_values.set_group(group) end

---@param kind xmake.value.kind Target type string, specifies the type of compilation target
---@see https://xmake.io/api/description/project-target.html#set-kind
function target_values.set_kind(kind) end

---@param languages xmake.value.language|xmake.value.language[] Language standard string or array, supports wildcard matching patterns
---@param ... xmake.value.language|xmake.value.language[]|{private?: any} Variable parameters, can pass multiple language standard strings
---@see https://xmake.io/api/description/project-target.html#set-languages
function target_values.set_languages(languages, ...) end

---@param license string License name string
---@see https://xmake.io/api/description/xpack-interfaces.html#set-license
function target_values.set_license(license) end

---@param optimize xmake.value.optimize Optimization level string, optional values: none, fast, faster, fastest, smallest, aggressive
---@see https://xmake.io/api/description/project-target.html#set-optimize
function target_values.set_optimize(optimize) end

---@param options xmake.value.option|xmake.value.option[] Option name string or array, supports wildcard matching patterns
---@param ... xmake.value.option|xmake.value.option[] Variable parameters, can pass multiple option name strings
---@see https://xmake.io/api/description/project-target.html#set-options
function target_values.set_options(options, ...) end

---@param plat xmake.value.platform Platform name string, such as "linux", "macosx", "windows", "android"
---@see https://xmake.io/api/description/project-target.html#set-plat
function target_values.set_plat(plat) end

---@param prefixdir string Installation prefix subdirectory path string
---@see https://xmake.io/api/description/project-target.html#set-prefixdir
function target_values.set_prefixdir(prefixdir) end

---@param prefixname string Target file prefix name string, such as "lib" or ""
---@see https://xmake.io/api/description/project-target.html#set-prefixname
function target_values.set_prefixname(prefixname) end

function target_values.set_rules() end

---@param runargs string|string[] Run argument string or array, such as "-x", "--arg1=val"
---@param ... string|string[] Variable parameters, can pass multiple run argument strings
---@see https://xmake.io/api/description/project-target.html#set-runargs
function target_values.set_runargs(runargs, ...) end

---@param runtimes xmake.value.runtime|xmake.value.runtime[] Runtime library string or array, such as "MT", "MD", "MTd", "MDd"
---@param ... xmake.value.runtime|xmake.value.runtime[] Variable parameters, can pass multiple runtime library strings
---@see https://xmake.io/api/description/project-target.html#set-runtimes
function target_values.set_runtimes(runtimes, ...) end

---@param strip xmake.value.strip Strip mode string, optional values: debug, all
---@see https://xmake.io/api/description/project-target.html#set-strip
function target_values.set_strip(strip) end

---@param suffixname string Target file suffix name string, such as "-d" or ""
---@see https://xmake.io/api/description/project-target.html#set-suffixname
function target_values.set_suffixname(suffixname) end

---@param symbols xmake.value.symbol Symbol mode string, optional values: debug, hidden, none
---@param opts? {private?: any}
---@see https://xmake.io/api/description/project-target.html#set-symbols
function target_values.set_symbols(symbols, opts) end

---@param toolchains xmake.value.toolchain|xmake.value.toolchain[] Toolchain name string or array, such as "gcc", "clang", "msvc", "ndk[gcc]", "mingw[clang]@llvm-mingw", "msvc[vs=2025]"
---@param ... xmake.value.toolchain|xmake.value.toolchain[]|{vs?: any, plat?: any, arch?: any, clang?: any, gcc?: any, vs_sdkver?: any, vs_toolset?: any} Variable parameters, can pass multiple toolchain name strings
---@see https://xmake.io/api/description/project-target.html#set-toolchains
function target_values.set_toolchains(toolchains, ...) end

---@param version string Project version string, e.g. "1.5.1"
---@param opts? {build?: any, soname?: any}
---@see https://xmake.io/api/description/global-interfaces.html#set-version
function target_values.set_version(version, opts) end

---@param warnings xmake.value.warning Warning level string, optional values: none, less, more, all, error
---@param opts? {private?: any}
---@see https://xmake.io/api/description/project-target.html#set-warnings
function target_values.set_warnings(warnings, opts) end

---@class xmake.target.paths
local target_paths = {}

function target_paths.add_cleanfiles() end

---@param configfiles string|string[] Configuration file path string or array, supports wildcard matching patterns
---@param ... string|string[]|{prefixdir?: any, rootdir?: any, filename?: any} Variable parameters, can pass multiple configuration file path strings
---@see https://xmake.io/api/description/project-target.html#add-configfiles
function target_paths.add_configfiles(configfiles, ...) end

---@param embeddirs string|string[] #embed search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple #embed search directory strings
---@see https://xmake.io/api/description/project-target.html#add-embeddirs
function target_paths.add_embeddirs(embeddirs, ...) end

---@param extrafiles string|string[] Extra file path string or array, such as "assets/other.txt"
---@param ... string|string[] Variable parameters, can pass multiple extra file path strings
---@see https://xmake.io/api/description/project-target.html#add-extrafiles
function target_paths.add_extrafiles(extrafiles, ...) end

---@param files string|string[] File path string or file path array, supports wildcard matching patterns
---@param ... string|string[]|{defines?: any, languages?: any, includedirs?: any, rules?: any, force?: any, sourcekind?: any, flags?: any} Variable parameters, can pass multiple file path strings
---@see https://xmake.io/api/description/project-target.html#add-files
function target_paths.add_files(files, ...) end

---@param frameworkdirs string|string[] Framework search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple framework search directory strings
---@see https://xmake.io/api/description/project-target.html#add-frameworkdirs
function target_paths.add_frameworkdirs(frameworkdirs, ...) end

---@param headerfiles string|string[] Header file path string or array, supports wildcard matching patterns
---@param ... string|string[]|{prefixdir?: any, rootdir?: any, filename?: any} Variable parameters, can pass multiple header file path strings
---@see https://xmake.io/api/description/project-target.html#add-headerfiles
function target_paths.add_headerfiles(headerfiles, ...) end

---@param includedirs string|string[] Header file search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple header file search directory strings
---@see https://xmake.io/api/description/project-target.html#add-includedirs
function target_paths.add_includedirs(includedirs, ...) end

---@param installfiles string|string[] Installation file path string or array, supports wildcard matching patterns
---@param ... string|string[]|{prefixdir?: any, rootdir?: any, filename?: any} Variable parameters, can pass multiple installation file path strings
---@see https://xmake.io/api/description/project-target.html#add-installfiles
function target_paths.add_installfiles(installfiles, ...) end

---@param linkdirs string|string[] Link library search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple link library search directory strings
---@see https://xmake.io/api/description/project-target.html#add-linkdirs
function target_paths.add_linkdirs(linkdirs, ...) end

---@param includedirs string|string[] System header file search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple system header file search directory strings
---@see https://xmake.io/api/description/project-target.html#add-sysincludedirs
function target_paths.add_sysincludedirs(includedirs, ...) end

function target_paths.del_files() end

function target_paths.remove_configfiles() end

function target_paths.remove_extrafiles() end

---@param files string|string[] File path string or file path array, supports wildcard matching patterns
---@param ... string|string[] Variable parameters, can pass multiple file path strings
---@see https://xmake.io/api/description/project-target.html#remove-files
function target_paths.remove_files(files, ...) end

---@param headerfiles string|string[] Header file path string or array, supports wildcard matching patterns
---@param ... string|string[] Variable parameters, can pass multiple header file path strings
---@see https://xmake.io/api/description/project-target.html#remove-headerfiles
function target_paths.remove_headerfiles(headerfiles, ...) end

function target_paths.remove_installfiles() end

function target_paths.set_autogendir() end

---@param configdir string Template configuration file output directory path string
---@see https://xmake.io/api/description/project-target.html#set-configdir
function target_paths.set_configdir(configdir) end

---@see https://xmake.io/api/description/project-target.html#set-dependir
function target_paths.set_dependir() end

---@param installdir string Installation directory path string
---@see https://xmake.io/api/description/project-target.html#set-installdir
function target_paths.set_installdir(installdir) end

---@see https://xmake.io/api/description/project-target.html#set-objectdir
function target_paths.set_objectdir() end

---@param header string C precompiled header file path string
---@see https://xmake.io/api/description/project-target.html#set-pcheader
function target_paths.set_pcheader(header) end

---@param header string C++ precompiled header file path string
---@see https://xmake.io/api/description/project-target.html#set-pcxxheader
function target_paths.set_pcxxheader(header) end

---@param header string ObjC precompiled header file path string
---@see https://xmake.io/api/description/project-target.html#set-pmheader
function target_paths.set_pmheader(header) end

---@param header string ObjC++ precompiled header file path string
---@see https://xmake.io/api/description/project-target.html#set-pmxxheader
function target_paths.set_pmxxheader(header) end

---@param rundir string Running directory path string
---@see https://xmake.io/api/description/project-target.html#set-rundir
function target_paths.set_rundir(rundir) end

---@see https://xmake.io/api/description/project-target.html#set-targetdir
function target_paths.set_targetdir() end

---@class xmake.target.groups
local target_groups = {}

---@param linkgroups string|string[] Link group name string or array, such as "group1", "group2"
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple link group name strings
---@see https://xmake.io/api/description/project-target.html#add-linkgroups
function target_groups.add_linkgroups(linkgroups, ...) end

---@param linkorders string|string[] Link order string or array, such as "dep1", "dep2"
---@param ... string|string[] Variable parameters, can pass multiple link order strings
---@see https://xmake.io/api/description/project-target.html#add-linkorders
function target_groups.add_linkorders(linkorders, ...) end

---@class xmake.target.keyvalues
local target_keyvalues = {}

---@param name string Environment variable name string, such as "PATH", "LD_LIBRARY_PATH"
---@param values string|string[] Environment variable value string or array, supports multiple values
---@param ... string|string[] Variable parameters, can pass multiple environment variable values
---@see https://xmake.io/api/description/project-target.html#add-runenvs
function target_keyvalues.add_runenvs(name, values, ...) end

---@param name string Configuration name string, such as "markdown_flags"
---@param values any Configuration value, can be any type
---@param ... any Variable parameters, can pass multiple configuration values
---@see https://xmake.io/api/description/project-target.html#add-values
function target_keyvalues.add_values(name, values, ...) end

---@param name string Configuration variable name string, such as "HAS_FOO"
---@param value any Configuration variable value, can be any type
---@see https://xmake.io/api/description/project-target.html#set-configvar
function target_keyvalues.set_configvar(name, value) end

---@param policy xmake.value.policy Policy name string, such as "check.auto_ignore_flags", "build.warning"
---@param value boolean Policy value, true means enable, false means disable
---@see https://xmake.io/api/description/project-target.html#set-policy
function target_keyvalues.set_policy(policy, value) end

---@param name string Environment variable name string, such as "PATH", "LD_LIBRARY_PATH"
---@param value string Environment variable value string
---@see https://xmake.io/api/description/project-target.html#set-runenv
function target_keyvalues.set_runenv(name, value) end

---@param toolname string Tool name string, such as "cc", "cxx", "ld", "ar"
---@param tool string Tool path string, such as "/usr/bin/gcc"
---@see https://xmake.io/api/description/project-target.html#set-toolset
function target_keyvalues.set_toolset(toolname, tool) end

---@param name string Configuration name string, such as "markdown_flags"
---@param values any Configuration value, can be any type
---@param ... any Variable parameters, can pass multiple configuration values
---@see https://xmake.io/api/description/project-target.html#set-values
function target_keyvalues.set_values(name, values, ...) end

---@class xmake.target.script
local target_script = {}

---@param script fun(target: target)|string After build script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-build
function target_script.after_build(script) end

---@param script fun(target: target, sourcefile: any, opt: any)|string After build single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-build-file
function target_script.after_build_file(script) end

---@param script fun(target: target, sourcebatch: any, opt: any)|string After build files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-build-files
function target_script.after_build_files(script) end

---@param script fun(package: package, batchcmds: any)|string After build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#after-buildcmd
function target_script.after_buildcmd(script) end

---@param script fun(target: target, batchcmds: any, sourcefile: any, opt: any)|string After build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-buildcmd-file
function target_script.after_buildcmd_file(script) end

---@param script fun(target: target, batchcmds: any, sourcebatch: any, opt: any)|string After build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-buildcmd-files
function target_script.after_buildcmd_files(script) end

---@param script fun(target: target)|string After clean script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-clean
function target_script.after_clean(script) end

---@param script fun(target: target)|string After config script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-config
function target_script.after_config(script) end

---@param script fun(target: target)|string After install script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-install
function target_script.after_install(script) end

---@param script fun(component: any, batchcmds: any)|string After install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#after-installcmd
function target_script.after_installcmd(script) end

---@param script fun(target: target)|string After link script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-link
function target_script.after_link(script) end

function target_script.after_linkcmd() end

function target_script.after_load() end

---@param script fun(target: target)|string After package script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-package
function target_script.after_package(script) end

---@param script fun(target: target)|string After prepare script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-prepare
function target_script.after_prepare(script) end

---@param script fun(target: target, sourcefile: any, opt: any)|string After prepare single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-prepare-file
function target_script.after_prepare_file(script) end

---@param script fun(target: target, sourcebatch: any, opt: any)|string After prepare batch files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-prepare-files
function target_script.after_prepare_files(script) end

function target_script.after_preparecmd() end

function target_script.after_preparecmd_file() end

function target_script.after_preparecmd_files() end

---@param script fun(target: target)|string After run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-run
function target_script.after_run(script) end

function target_script.after_test() end

---@param script fun(target: target)|string After uninstall script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-uninstall
function target_script.after_uninstall(script) end

---@param script fun(component: any, batchcmds: any)|string After uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#after-uninstallcmd
function target_script.after_uninstallcmd(script) end

---@param script fun(target: target)|string Before build script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-build
function target_script.before_build(script) end

---@param script fun(target: target, sourcefile: any, opt: any)|string Before build single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-build-file
function target_script.before_build_file(script) end

---@param script fun(target: target, sourcebatch: any, opt: any)|string Before build files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-build-files
function target_script.before_build_files(script) end

---@param script fun(package: package, batchcmds: any)|string Before build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#before-buildcmd
function target_script.before_buildcmd(script) end

---@param script fun(target: target, batchcmds: any, sourcefile: any, opt: any)|string Before build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-buildcmd-file
function target_script.before_buildcmd_file(script) end

---@param script fun(target: target, batchcmds: any, sourcebatch: any, opt: any)|string Before build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-buildcmd-files
function target_script.before_buildcmd_files(script) end

---@param script fun(target: target)|string Before clean script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-clean
function target_script.before_clean(script) end

---@param script fun(target: target)|string Before config script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-config
function target_script.before_config(script) end

---@param script fun(target: target)|string Before install script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-install
function target_script.before_install(script) end

---@param script fun(component: any, batchcmds: any)|string Before install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#before-installcmd
function target_script.before_installcmd(script) end

---@param script fun(target: target)|string Before link script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-link
function target_script.before_link(script) end

function target_script.before_linkcmd() end

---@param script fun(target: target)|string Before package script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-package
function target_script.before_package(script) end

---@param script fun(target: target)|string Before prepare script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-prepare
function target_script.before_prepare(script) end

---@param script fun(target: target, sourcefile: any, opt: any)|string Before prepare single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-prepare-file
function target_script.before_prepare_file(script) end

---@param script fun(target: target, sourcebatch: any, opt: any)|string Before prepare batch files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-prepare-files
function target_script.before_prepare_files(script) end

function target_script.before_preparecmd() end

function target_script.before_preparecmd_file() end

function target_script.before_preparecmd_files() end

---@param script fun(target: target)|string Before run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-run
function target_script.before_run(script) end

function target_script.before_test() end

---@param script fun(target: target)|string Before uninstall script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-uninstall
function target_script.before_uninstall(script) end

---@param script fun(component: any, batchcmds: any)|string Before uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#before-uninstallcmd
function target_script.before_uninstallcmd(script) end

---@param script fun(target: target)|string Build script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-build
function target_script.on_build(script) end

---@param script fun(target: target, sourcefile: any, opt: any)|string Single file build script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-build-file
function target_script.on_build_file(script) end

---@param script fun(target: target, sourcebatch: any, opt: any)|string Multi-file build script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-build-files
function target_script.on_build_files(script) end

---@param script fun(package: package, batchcmds: any)|string Build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#on-buildcmd
function target_script.on_buildcmd(script) end

---@param script fun(target: target, batchcmds: any, sourcefile: any, opt: any)|string Build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-buildcmd-file
function target_script.on_buildcmd_file(script) end

---@param script fun(target: target, batchcmds: any, sourcebatch: any, opt: any)|string Build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-buildcmd-files
function target_script.on_buildcmd_files(script) end

---@param script fun(target: target)|string Clean script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-clean
function target_script.on_clean(script) end

---@param script fun(target: target)|string Configuration script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-config
function target_script.on_config(script) end

---@param script fun(target: target)|string Install script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-install
function target_script.on_install(script) end

---@param script fun(component: any, batchcmds: any)|string Install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#on-installcmd
function target_script.on_installcmd(script) end

---@param script fun(target: target)|string Link script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-link
function target_script.on_link(script) end

function target_script.on_linkcmd() end

---@param script fun(target: target)|string Load script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-load
function target_script.on_load(script) end

---@param script fun(target: target)|string Package script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-package
function target_script.on_package(script) end

---@param script fun(target: target, opt: any)|string Prepare phase script function, receives target, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-prepare
function target_script.on_prepare(script) end

---@param script fun(target: target, sourcefile: any, opt: any)|string Single file processing script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-prepare-file
function target_script.on_prepare_file(script) end

---@param script fun(target: target, jobgraph: any, sourcebatch: any, opt: any)|string Batch file processing script function, receives target, jobgraph, sourcebatch, opt parameters
---@param opts? {jobgraph?: any}
---@see https://xmake.io/api/description/project-target.html#on-prepare-files
function target_script.on_prepare_files(script, opts) end

function target_script.on_preparecmd() end

function target_script.on_preparecmd_file() end

function target_script.on_preparecmd_files() end

---@param script fun(target: target)|string Run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-run
function target_script.on_run(script) end

---@param script fun(package: package)|string Test script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-test
function target_script.on_test(script) end

---@param script fun(target: target)|string Uninstall script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-uninstall
function target_script.on_uninstall(script) end

---@param script fun(component: any, batchcmds: any)|string Uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#on-uninstallcmd
function target_script.on_uninstallcmd(script) end
