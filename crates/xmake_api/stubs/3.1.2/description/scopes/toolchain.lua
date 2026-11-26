---@meta
---@diagnostic disable: unused-local

---@class xmake.toolchain.values
local toolchain_values = {}

function toolchain_values.add_aicpuflags() end

---@param arflags string|string[] Static library archive option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple static library archive option strings
---@see https://xmake.io/api/description/project-target.html#add-arflags
function toolchain_values.add_arflags(arflags, ...) end

function toolchain_values.add_ascflags() end

function toolchain_values.add_ascnpuarchs() end

---@param asflags string|string[] Assembly compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple assembly compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-asflags
function toolchain_values.add_asflags(asflags, ...) end

---@param cflags string|string[] C compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple C compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cflags
function toolchain_values.add_cflags(cflags, ...) end

function toolchain_values.add_csflags() end

---@param cuflags string|string[] CUDA compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cuflags
function toolchain_values.add_cuflags(cuflags, ...) end

---@param cugencodes string|string[] CUDA device gencode setting string or array, such as "sm_30", "sm_50"
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA device gencode setting strings
---@see https://xmake.io/api/description/project-target.html#add-cugencodes
function toolchain_values.add_cugencodes(cugencodes, ...) end

---@param culdflags string|string[] CUDA device link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple CUDA device link option strings
---@see https://xmake.io/api/description/project-target.html#add-culdflags
function toolchain_values.add_culdflags(culdflags, ...) end

---@see https://xmake.io/api/description/project-target.html#add-cxflags
function toolchain_values.add_cxflags() end

---@param cxxflags string|string[] C++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple C++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-cxxflags
function toolchain_values.add_cxxflags(cxxflags, ...) end

---@param dcflags string|string[] D language compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple D language compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-dcflags
function toolchain_values.add_dcflags(dcflags, ...) end

---@param defines string|string[] Macro definition string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple macro definition strings
---@see https://xmake.io/api/description/project-target.html#add-defines
function toolchain_values.add_defines(defines, ...) end

---@param fcflags string|string[] Fortran compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Fortran compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-fcflags
function toolchain_values.add_fcflags(fcflags, ...) end

---@param frameworkdirs string|string[] Framework search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple framework search directory strings
---@see https://xmake.io/api/description/project-target.html#add-frameworkdirs
function toolchain_values.add_frameworkdirs(frameworkdirs, ...) end

---@param frameworks string|string[] Framework name string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple framework name strings
---@see https://xmake.io/api/description/project-target.html#add-frameworks
function toolchain_values.add_frameworks(frameworks, ...) end

---@param gcflags string|string[] Go compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Go compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-gcflags
function toolchain_values.add_gcflags(gcflags, ...) end

---@param includedirs string|string[] Header file search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple header file search directory strings
---@see https://xmake.io/api/description/project-target.html#add-includedirs
function toolchain_values.add_includedirs(includedirs, ...) end

function toolchain_values.add_kcflags() end

---@param ldflags string|string[] Link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple link option strings
---@see https://xmake.io/api/description/project-target.html#add-ldflags
function toolchain_values.add_ldflags(ldflags, ...) end

---@param linkdirs string|string[] Link library search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple link library search directory strings
---@see https://xmake.io/api/description/project-target.html#add-linkdirs
function toolchain_values.add_linkdirs(linkdirs, ...) end

---@param links string|string[] Link library name string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple link library name strings
---@see https://xmake.io/api/description/project-target.html#add-links
function toolchain_values.add_links(links, ...) end

---@param mflags string|string[] ObjC compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mflags
function toolchain_values.add_mflags(mflags, ...) end

function toolchain_values.add_mrcflags() end

---@param mxflags string|string[] ObjC/ObjC++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC/ObjC++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mxflags
function toolchain_values.add_mxflags(mxflags, ...) end

---@param mxxflags string|string[] ObjC++ compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple ObjC++ compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-mxxflags
function toolchain_values.add_mxxflags(mxxflags, ...) end

function toolchain_values.add_ncflags() end

function toolchain_values.add_pcflags() end

---@param rcflags string|string[] Rust compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Rust compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-rcflags
function toolchain_values.add_rcflags(rcflags, ...) end

---@param rpathdirs string|string[] Runtime library search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple runtime library search directory strings
---@see https://xmake.io/api/description/project-target.html#add-rpathdirs
function toolchain_values.add_rpathdirs(rpathdirs, ...) end

---@param scflags string|string[] Swift compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Swift compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-scflags
function toolchain_values.add_scflags(scflags, ...) end

---@param shflags string|string[] Dynamic library link option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple dynamic library link option strings
---@see https://xmake.io/api/description/project-target.html#add-shflags
function toolchain_values.add_shflags(shflags, ...) end

---@param includedirs string|string[] System header file search directory string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple system header file search directory strings
---@see https://xmake.io/api/description/project-target.html#add-sysincludedirs
function toolchain_values.add_sysincludedirs(includedirs, ...) end

---@param syslinks string|string[] System link library name string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple system link library name strings
---@see https://xmake.io/api/description/project-target.html#add-syslinks
function toolchain_values.add_syslinks(syslinks, ...) end

---@param undefines string|string[] Macro definition name string or array, such as "DEBUG"
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple macro definition name strings
---@see https://xmake.io/api/description/project-target.html#add-undefines
function toolchain_values.add_undefines(undefines, ...) end

---@param zcflags string|string[] Zig compilation option string or array, supports wildcard matching patterns
---@param ... string|string[]|{private?: any} Variable parameters, can pass multiple Zig compilation option strings
---@see https://xmake.io/api/description/project-target.html#add-zcflags
function toolchain_values.add_zcflags(zcflags, ...) end

function toolchain_values.set_archs() end

---@param bindir string Binary directory path string
---@see https://xmake.io/api/description/custom-toolchain.html#set-bindir
function toolchain_values.set_bindir(bindir) end

function toolchain_values.set_cross() end

---@param description string|string[] Description string or array
---@param ... string|string[] Variable parameters, can pass multiple description lines
---@see https://xmake.io/api/description/configuration-option.html#set-description
function toolchain_values.set_description(description, ...) end

---@param url string Package homepage URL string
---@see https://xmake.io/api/description/package-dependencies.html#set-homepage
function toolchain_values.set_homepage(url) end

---@param kind "standalone" Toolchain type: "standalone"
---@see https://xmake.io/api/description/custom-toolchain.html#set-kind
function toolchain_values.set_kind(kind) end

---@param runtimes xmake.value.runtime|xmake.value.runtime[] Runtime library string or array, such as "MT", "MD", "MTd", "MDd"
---@param ... xmake.value.runtime|xmake.value.runtime[] Variable parameters, can pass multiple runtime library strings
---@see https://xmake.io/api/description/project-target.html#set-runtimes
function toolchain_values.set_runtimes(runtimes, ...) end

---@param sdkdir string SDK directory path string
---@see https://xmake.io/api/description/custom-toolchain.html#set-sdkdir
function toolchain_values.set_sdkdir(sdkdir) end

---@class xmake.toolchain.keyvalues
local toolchain_keyvalues = {}

---@param name string Environment variable name string, such as "PATH", "LD_LIBRARY_PATH"
---@param values string|string[] Environment variable value string or array, supports multiple values
---@param ... string|string[] Variable parameters, can pass multiple environment variable values
---@see https://xmake.io/api/description/project-target.html#add-runenvs
function toolchain_keyvalues.add_runenvs(name, values, ...) end

function toolchain_keyvalues.add_toolset() end

---@param formats string|string[] Package format name string or array
---@param ... string|string[] Variable parameters, can pass multiple format names
---@see https://xmake.io/api/description/xpack-interfaces.html#set-formats
function toolchain_keyvalues.set_formats(formats, ...) end

---@param tool xmake.value.toolkind Tool name string (cc, cxx, ld, sh, ar, ex, strip, mm, mxx, as)
---@param tools string|string[] Tool program name string or array
---@param ... string|string[] Variable parameters, can pass multiple tool names
---@see https://xmake.io/api/description/custom-toolchain.html#set-toolset
function toolchain_keyvalues.set_toolset(tool, tools, ...) end

---@class xmake.toolchain.script
local toolchain_script = {}

---@param script fun(toolchain: toolchain)|string Check script function with toolchain parameter
---@see https://xmake.io/api/description/custom-toolchain.html#on-check
function toolchain_script.on_check(script) end

---@param script fun(toolchain: toolchain)|string Load script function with toolchain parameter, or a string referring to a sibling `.lua` file (v3.0.9 and above)
---@see https://xmake.io/api/description/custom-toolchain.html#on-load
function toolchain_script.on_load(script) end
