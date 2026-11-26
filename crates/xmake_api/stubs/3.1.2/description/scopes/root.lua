---@meta
---@diagnostic disable: unused-local

---@class xmake.root.values
local root_values = {}

function root_values.add_addons() end

---@param repos string|string[] Repository name and URL string or array, format: "name url"
---@param ... string|string[]|{rootdir?: any} Variable parameters, can pass multiple repositories
---@see https://xmake.io/api/description/global-interfaces.html#add-repositories
function root_values.add_repositories(repos, ...) end

---@param packages string|string[] Package name string or array, supports pattern matching like "*" or "libpng.zlib"
---@param ... string|string[]|{configs?: any, override?: any, version?: any, debug?: any} Variable parameters, can pass multiple package names
---@see https://xmake.io/api/description/global-interfaces.html#add-requireconfs
function root_values.add_requireconfs(packages, ...) end

---@param packages string|string[] Package name string or array, supports semantic version like "tbox 1.6.*"
---@param ... string|string[]|{optional?: any, system?: any, verify?: any, debug?: any, private?: any, configs?: any, alias?: any} Variable parameters, can pass multiple package names
---@see https://xmake.io/api/description/global-interfaces.html#add-requires
function root_values.add_requires(packages, ...) end

---@param archs xmake.value.arch|xmake.value.arch[] Allowed compilation architecture string or array, supports platform-specific format like "windows
---@param ... xmake.value.arch|xmake.value.arch[] Variable parameters, can pass multiple architecture specifications
---@see https://xmake.io/api/description/global-interfaces.html#set-allowedarchs
function root_values.set_allowedarchs(archs, ...) end

---@param modes xmake.value.mode|xmake.value.mode[] Allowed compilation mode name string or array
---@param ... xmake.value.mode|xmake.value.mode[] Variable parameters, can pass multiple mode names
---@see https://xmake.io/api/description/global-interfaces.html#set-allowedmodes
function root_values.set_allowedmodes(modes, ...) end

---@param platforms xmake.value.platform|xmake.value.platform[] Allowed compilation platform name string or array
---@param ... xmake.value.platform|xmake.value.platform[] Variable parameters, can pass multiple platform names
---@see https://xmake.io/api/description/global-interfaces.html#set-allowedplats
function root_values.set_allowedplats(platforms, ...) end

---@param archs xmake.value.arch|xmake.value.arch[] Default compilation architecture string or array, supports platform-specific format like "iphoneos
---@param ... xmake.value.arch|xmake.value.arch[] Variable parameters, can pass multiple architecture specifications
---@see https://xmake.io/api/description/global-interfaces.html#set-defaultarchs
function root_values.set_defaultarchs(archs, ...) end

---@param mode xmake.value.mode Default compilation mode name, e.g. "release", "debug", "releasedbg"
---@see https://xmake.io/api/description/global-interfaces.html#set-defaultmode
function root_values.set_defaultmode(mode) end

---@param platform xmake.value.platform Default compilation platform name, e.g. "iphoneos", "windows"
---@see https://xmake.io/api/description/global-interfaces.html#set-defaultplat
function root_values.set_defaultplat(platform) end

---@param description string|string[] Description string or array
---@param ... string|string[] Variable parameters, can pass multiple description lines
---@see https://xmake.io/api/description/configuration-option.html#set-description
function root_values.set_description(description, ...) end

---@param name string Project name string
---@see https://xmake.io/api/description/global-interfaces.html#set-project
function root_values.set_project(name) end

---@class xmake.root.custom
local root_custom = {}

---@param dirs string|string[] Module directory path string or array
---@param ... string|string[] Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/global-interfaces.html#add-moduledirs
function root_custom.add_moduledirs(dirs, ...) end

function root_custom.add_platformdirs() end

---@param dirs string|string[] Plugin directory path string or array
---@param ... string|string[] Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/global-interfaces.html#add-plugindirs
function root_custom.add_plugindirs(dirs, ...) end

function root_custom.add_toolchaindirs() end

---@param name xmake.value.config Configuration option name string
---@see https://xmake.io/api/description/global-interfaces.html#get-config
function root_custom.get_config(name) end

---@param configs xmake.value.config Configuration name string
---@param ... xmake.value.config Variable parameters, can pass multiple configuration names
---@see https://xmake.io/api/description/conditions.html#has-config
function root_custom.has_config(configs, ...) end

---@param packages xmake.value.package Package name string
---@param ... xmake.value.package Variable parameters, can pass multiple package names
---@see https://xmake.io/api/description/conditions.html#has-package
function root_custom.has_package(packages, ...) end

---@param arch xmake.value.arch Architecture name string
---@param ... xmake.value.arch Variable parameters, can pass multiple architecture names
---@see https://xmake.io/api/description/conditions.html#is-arch
function root_custom.is_arch(arch, ...) end

---@param name xmake.value.config Configuration option name string
---@param values string Configuration value string
---@param ... string Variable parameters, can pass multiple values
---@see https://xmake.io/api/description/conditions.html#is-config
function root_custom.is_config(name, values, ...) end

---@see https://xmake.io/api/description/conditions.html#is-cross
function root_custom.is_cross() end

---@param host xmake.value.platform Host system name string
---@param ... xmake.value.platform Variable parameters, can pass multiple host names
---@see https://xmake.io/api/description/conditions.html#is-host
function root_custom.is_host(host, ...) end

---@param kind xmake.value.kind Target kind name string
---@param ... xmake.value.kind Variable parameters, can pass multiple kind names
---@see https://xmake.io/api/description/conditions.html#is-kind
function root_custom.is_kind(kind, ...) end

---@param mode xmake.value.mode Compilation mode name string
---@param ... xmake.value.mode Variable parameters, can pass multiple mode names
---@see https://xmake.io/api/description/conditions.html#is-mode
function root_custom.is_mode(mode, ...) end

---@param os xmake.value.platform Operating system name string
---@param ... xmake.value.platform Variable parameters, can pass multiple OS names
---@see https://xmake.io/api/description/conditions.html#is-os
function root_custom.is_os(os, ...) end

---@param plat xmake.value.platform Platform name string
---@param ... xmake.value.platform Variable parameters, can pass multiple platform names
---@see https://xmake.io/api/description/conditions.html#is-plat
function root_custom.is_plat(plat, ...) end

---@class xmake.root.paths
local root_paths = {}

function root_paths.add_packagedirs() end

---@class xmake.root.keyvalues
local root_keyvalues = {}

---@param name xmake.value.config Configuration option name string
---@param value string|boolean|number Configuration value, can be string, boolean, or number
---@see https://xmake.io/api/description/global-interfaces.html#set-config
function root_keyvalues.set_config(name, value) end
