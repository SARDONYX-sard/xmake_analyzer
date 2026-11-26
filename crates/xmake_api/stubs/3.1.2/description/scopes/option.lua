---@meta
---@diagnostic disable: unused-local

--- The APIs of the `option` scope documented differently there.
---@class xmake.option.values
local option_values = {}

--- Sets the default value of the option: `true`/`false` for a switch, or a
--- string for a value option.
---
--- # Examples
---
--- ```lua
--- option("with_ssl")
---     set_default(false)
---     set_showmenu(true)
---
--- option("renderer")
---     set_default("opengl")
---     set_values("opengl", "vulkan")
--- ```
---
--- # Notes
---
--- Without a default, the option is detected by its checks (`add_cfuncs`, `add_links`, ...).
---@param value string|boolean|number Default value (string, boolean, or number)
---@see https://xmake.io/api/description/configuration-option.html#set-default
function option_values.set_default(value) end

function option_values.add_aicpuflags() end

---@param arflags string|string[] Static library archive option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple static library archive option strings
---@see https://xmake.io/api/description/project-target.html#add-arflags
function option_values.add_arflags(arflags, ...) end

function option_values.add_ascflags() end

function option_values.add_ascnpuarchs() end

---@param asflags string|string[] Assembly compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple assembly compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-asflags
function option_values.add_asflags(asflags, ...) end

---@param cflags string|string[] C compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple C compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cflags
function option_values.add_cflags(cflags, ...) end

---@param funcs string|string[] C function name string or array
---@param ... string|string[] Variable parameters, can pass multiple function names
---@see https://xmake.io/api/description/configuration-option.html#add-cfuncs
function option_values.add_cfuncs(funcs, ...) end

---@param includes string|string[] Header file name string or array
---@param ... string|string[] Variable parameters, can pass multiple header file names
---@see https://xmake.io/api/description/configuration-option.html#add-cincludes
function option_values.add_cincludes(includes, ...) end

function option_values.add_csflags() end

---@param types string|string[] C type name string or array
---@param ... string|string[] Variable parameters, can pass multiple type names
---@see https://xmake.io/api/description/configuration-option.html#add-ctypes
function option_values.add_ctypes(types, ...) end

---@param cuflags string|string[] CUDA compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cuflags
function option_values.add_cuflags(cuflags, ...) end

---@param cugencodes string|string[] CUDA device gencode setting string or array, such as "sm_30", "sm_50"
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA device gencode setting strings
---@see https://xmake.io/api/description/project-target.html#add-cugencodes
function option_values.add_cugencodes(cugencodes, ...) end

---@param culdflags string|string[] CUDA device link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA device link option strings
---@see https://xmake.io/api/description/project-target.html#add-culdflags
function option_values.add_culdflags(culdflags, ...) end

---@see https://xmake.io/api/description/project-target.html#add-cxflags
function option_values.add_cxflags() end

---@param cxxflags string|string[] C++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple C++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cxxflags
function option_values.add_cxxflags(cxxflags, ...) end

---@param funcs string|string[] C++ function name string or array
---@param ... string|string[] Variable parameters, can pass multiple function names
---@see https://xmake.io/api/description/configuration-option.html#add-cxxfuncs
function option_values.add_cxxfuncs(funcs, ...) end

---@param includes string|string[] C++ header file name string or array
---@param ... string|string[] Variable parameters, can pass multiple header file names
---@see https://xmake.io/api/description/configuration-option.html#add-cxxincludes
function option_values.add_cxxincludes(includes, ...) end

---@param types string|string[] C++ type name string or array
---@param ... string|string[] Variable parameters, can pass multiple type names
---@see https://xmake.io/api/description/configuration-option.html#add-cxxtypes
function option_values.add_cxxtypes(types, ...) end

---@param dcflags string|string[] D language compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple D language compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-dcflags
function option_values.add_dcflags(dcflags, ...) end

---@param defines string|string[] Macro definition string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple macro definition strings
---@see https://xmake.io/api/description/project-target.html#add-defines
function option_values.add_defines(defines, ...) end

---@param deps string|string[] Dependency option name string or array
---@param ... string|string[] Variable parameters, can pass multiple dependency names
---@see https://xmake.io/api/description/configuration-option.html#add-deps
function option_values.add_deps(deps, ...) end

---@param fcflags string|string[] Fortran compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Fortran compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-fcflags
function option_values.add_fcflags(fcflags, ...) end

function option_values.add_features() end

---@param frameworks string|string[] Framework name string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple framework name strings
---@see https://xmake.io/api/description/project-target.html#add-frameworks
function option_values.add_frameworks(frameworks, ...) end

---@param gcflags string|string[] Go compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Go compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-gcflags
function option_values.add_gcflags(gcflags, ...) end

---@param modules string|string[] Module name string or array
---@param ... string|string[] Variable parameters, can pass multiple module names
---@see https://xmake.io/api/description/project-target.html#add-imports
function option_values.add_imports(modules, ...) end

---@param includedirs string|string[] Header file search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple header file search directory strings
---@see https://xmake.io/api/description/project-target.html#add-includedirs
function option_values.add_includedirs(includedirs, ...) end

function option_values.add_kcflags() end

---@param ldflags string|string[] Link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple link option strings
---@see https://xmake.io/api/description/project-target.html#add-ldflags
function option_values.add_ldflags(ldflags, ...) end

---@param links string|string[] Library name string or array
---@param ... string|string[] Variable parameters, can pass multiple library names
---@see https://xmake.io/api/description/configuration-option.html#add-links
function option_values.add_links(links, ...) end

---@param mflags string|string[] ObjC compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mflags
function option_values.add_mflags(mflags, ...) end

function option_values.add_mrcflags() end

---@param mxflags string|string[] ObjC/ObjC++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC/ObjC++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mxflags
function option_values.add_mxflags(mxflags, ...) end

---@param mxxflags string|string[] ObjC++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mxxflags
function option_values.add_mxxflags(mxxflags, ...) end

function option_values.add_ncflags() end

function option_values.add_pcflags() end

---@param rcflags string|string[] Rust compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Rust compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-rcflags
function option_values.add_rcflags(rcflags, ...) end

---@param rpathdirs string|string[] RPATH directory path string or array
---@param ... string|string[] Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/configuration-option.html#add-rpathdirs
function option_values.add_rpathdirs(rpathdirs, ...) end

---@param scflags string|string[] Swift compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Swift compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-scflags
function option_values.add_scflags(scflags, ...) end

---@param shflags string|string[] Dynamic library link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple dynamic library link option strings
---@see https://xmake.io/api/description/project-target.html#add-shflags
function option_values.add_shflags(shflags, ...) end

---@param syslinks string|string[] System link library name string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple system link library name strings
---@see https://xmake.io/api/description/project-target.html#add-syslinks
function option_values.add_syslinks(syslinks, ...) end

---@param undefines string|string[] Macro definition name string or array, such as "DEBUG"
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple macro definition name strings
---@see https://xmake.io/api/description/project-target.html#add-undefines
function option_values.add_undefines(undefines, ...) end

---@param vectorexts xmake.value.vectorext|xmake.value.vectorext[] Vector extension instruction string or array, such as "mmx", "neon", "avx"
---@param ... xmake.value.vectorext|xmake.value.vectorext[]|{private?: any} Variable parameters, can pass multiple vector extension instruction strings
---@see https://xmake.io/api/description/project-target.html#add-vectorexts
function option_values.add_vectorexts(vectorexts, ...) end

---@param zcflags string|string[] Zig compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Zig compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-zcflags
function option_values.add_zcflags(zcflags, ...) end

---@param category xmake.value.category Category name string
---@see https://xmake.io/api/description/configuration-option.html#set-category
function option_values.set_category(category) end

---@param description string|string[] Description string or array
---@param ... string|string[] Variable parameters, can pass multiple description lines
---@see https://xmake.io/api/description/configuration-option.html#set-description
function option_values.set_description(description, ...) end

---@param languages xmake.value.language|xmake.value.language[] Language standard string or array, supports wildcard matching patterns
---@param ... xmake.value.language|xmake.value.language[]|{private?: any} Variable parameters, can pass multiple language standard strings
---@see https://xmake.io/api/description/project-target.html#set-languages
function option_values.set_languages(languages, ...) end

---@param optimize xmake.value.optimize Optimization level string, optional values: none, fast, faster, fastest, smallest, aggressive
---@see https://xmake.io/api/description/project-target.html#set-optimize
function option_values.set_optimize(optimize) end

---@param showmenu boolean Whether to show in menu (boolean)
---@see https://xmake.io/api/description/configuration-option.html#set-showmenu
function option_values.set_showmenu(showmenu) end

---@param values string|string[] Option value string or array
---@param ... string|string[] Variable parameters, can pass multiple values
---@see https://xmake.io/api/description/configuration-option.html#set-values
function option_values.set_values(values, ...) end

---@param warnings xmake.value.warning Warning level string, optional values: none, less, more, all, error
---@param opts? {private?: any}
---@see https://xmake.io/api/description/project-target.html#set-warnings
function option_values.set_warnings(warnings, opts) end

--- The APIs of the `option` scope documented differently there.
---@class xmake.option.dictionary
local option_dictionary = {}

---@param name string Code snippet name string
---@param code string C code fragment string
---@param opts? {tryrun?: any, output?: any, number?: any}
---@see https://xmake.io/api/description/configuration-option.html#add-csnippets
function option_dictionary.add_csnippets(name, code, opts) end

---@param name string Code snippet name string
---@param code string C++ code fragment string
---@param opts? {tryrun?: any, output?: any, number?: any}
---@see https://xmake.io/api/description/configuration-option.html#add-cxxsnippets
function option_dictionary.add_cxxsnippets(name, code, opts) end

--- The APIs of the `option` scope documented differently there.
---@class xmake.option.paths
local option_paths = {}

---@param frameworkdirs string|string[] Framework search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple framework search directory strings
---@see https://xmake.io/api/description/project-target.html#add-frameworkdirs
function option_paths.add_frameworkdirs(frameworkdirs, ...) end

---@param linkdirs string|string[] Link directory path string or array
---@param ... string|string[] Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/configuration-option.html#add-linkdirs
function option_paths.add_linkdirs(linkdirs, ...) end

---@param includedirs string|string[] System header file search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple system header file search directory strings
---@see https://xmake.io/api/description/project-target.html#add-sysincludedirs
function option_paths.add_sysincludedirs(includedirs, ...) end

--- The APIs of the `option` scope documented differently there.
---@class xmake.option.script
local option_script = {}

---@param script fun(option: option)|string After check script function with option parameter
---@see https://xmake.io/api/description/configuration-option.html#after-check
function option_script.after_check(script) end

---@param script fun(option: option)|string Before check script function with option parameter
---@see https://xmake.io/api/description/configuration-option.html#before-check
function option_script.before_check(script) end

---@param script fun(option: option)|string Check script function with option parameter
---@see https://xmake.io/api/description/configuration-option.html#on-check
function option_script.on_check(script) end

--- The APIs of the `option` scope documented differently there.
---@class xmake.option.keyvalues
local option_keyvalues = {}

---@param name string Configuration variable name string, such as "HAS_FOO"
---@param value any Configuration variable value, can be any type
---@see https://xmake.io/api/description/project-target.html#set-configvar
function option_keyvalues.set_configvar(name, value) end
