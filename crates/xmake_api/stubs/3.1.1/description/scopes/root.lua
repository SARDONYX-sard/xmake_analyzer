---@meta

---@class xmake.root.values
local root_values = {}

function root_values.add_addons() end

---@param repos string|string[] Repository name and URL string or array, format: "name url"
---@param ... string|string[] Variable parameters, can pass multiple repositories
---@param opts? {rootdir?: any}
---@see https://xmake.io/api/description/global-interfaces.html#add-repositories
function root_values.add_repositories(repos, ...) end

---@param packages string|string[] Package name string or array, supports pattern matching like "*" or "libpng.zlib"
---@param ... string|string[] Variable parameters, can pass multiple package names
---@param opts? {configs?: any, override?: any, version?: any, debug?: any}
---@see https://xmake.io/api/description/global-interfaces.html#add-requireconfs
function root_values.add_requireconfs(packages, ...) end

---@param packages string|string[] Package name string or array, supports semantic version like "tbox 1.6.*"
---@param ... string|string[] Variable parameters, can pass multiple package names
---@param opts? {optional?: any, system?: any, verify?: any, debug?: any, private?: any, configs?: any, alias?: any}
---@see https://xmake.io/api/description/global-interfaces.html#add-requires
function root_values.add_requires(packages, ...) end

---@param archs string|string[] Allowed compilation architecture string or array, supports platform-specific format like "windows
---@param ... string|string[] Variable parameters, can pass multiple architecture specifications
---@see https://xmake.io/api/description/global-interfaces.html#set-allowedarchs
function root_values.set_allowedarchs(archs, ...) end

---@param modes string|string[] Allowed compilation mode name string or array
---@param ... string|string[] Variable parameters, can pass multiple mode names
---@see https://xmake.io/api/description/global-interfaces.html#set-allowedmodes
function root_values.set_allowedmodes(modes, ...) end

---@param platforms string|string[] Allowed compilation platform name string or array
---@param ... string|string[] Variable parameters, can pass multiple platform names
---@see https://xmake.io/api/description/global-interfaces.html#set-allowedplats
function root_values.set_allowedplats(platforms, ...) end

---@param archs string|string[] Default compilation architecture string or array, supports platform-specific format like "iphoneos
---@param ... string|string[] Variable parameters, can pass multiple architecture specifications
---@see https://xmake.io/api/description/global-interfaces.html#set-defaultarchs
function root_values.set_defaultarchs(archs, ...) end

---@param mode string Default compilation mode name, e.g. "release", "debug", "releasedbg"
---@see https://xmake.io/api/description/global-interfaces.html#set-defaultmode
function root_values.set_defaultmode(mode) end

---@param platform string Default compilation platform name, e.g. "iphoneos", "windows"
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

---@param name string Configuration option name string
---@see https://xmake.io/api/description/global-interfaces.html#get-config
function root_custom.get_config(name) end

---@param configs string Configuration name string
---@param ... string Variable parameters, can pass multiple configuration names
---@see https://xmake.io/api/description/conditions.html#has-config
function root_custom.has_config(configs, ...) end

---@param packages string Package name string
---@param ... string Variable parameters, can pass multiple package names
---@see https://xmake.io/api/description/conditions.html#has-package
function root_custom.has_package(packages, ...) end

---@param arch string Architecture name string
---@param ... string Variable parameters, can pass multiple architecture names
---@see https://xmake.io/api/description/conditions.html#is-arch
function root_custom.is_arch(arch, ...) end

---@param name string Configuration option name string
---@param values string Configuration value string
---@param ... string Variable parameters, can pass multiple values
---@see https://xmake.io/api/description/conditions.html#is-config
function root_custom.is_config(name, values, ...) end

---@see https://xmake.io/api/description/conditions.html#is-cross
function root_custom.is_cross() end

---@param host string Host system name string
---@param ... string Variable parameters, can pass multiple host names
---@see https://xmake.io/api/description/conditions.html#is-host
function root_custom.is_host(host, ...) end

---@param kind string Target kind name string
---@param ... string Variable parameters, can pass multiple kind names
---@see https://xmake.io/api/description/conditions.html#is-kind
function root_custom.is_kind(kind, ...) end

---@param mode string Compilation mode name string
---@param ... string Variable parameters, can pass multiple mode names
---@see https://xmake.io/api/description/conditions.html#is-mode
function root_custom.is_mode(mode, ...) end

---@param os string Operating system name string
---@param ... string Variable parameters, can pass multiple OS names
---@see https://xmake.io/api/description/conditions.html#is-os
function root_custom.is_os(os, ...) end

---@param plat string Platform name string
---@param ... string Variable parameters, can pass multiple platform names
---@see https://xmake.io/api/description/conditions.html#is-plat
function root_custom.is_plat(plat, ...) end

---@class xmake.root.paths
local root_paths = {}

function root_paths.add_packagedirs() end

---@class xmake.root.keyvalues
local root_keyvalues = {}

---@param name string Configuration option name string
---@param value string|boolean|number Configuration value, can be string, boolean, or number
---@see https://xmake.io/api/description/global-interfaces.html#set-config
function root_keyvalues.set_config(name, value) end
