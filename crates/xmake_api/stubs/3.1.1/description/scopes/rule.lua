---@meta

---@class xmake.rule.values
local rule_values = {}

---@param deps string|string[] Dependency rule name string or array
---@param ... string|string[] Variable parameters, can pass multiple dependency names
---@param opts? {order?: any}
---@see https://xmake.io/api/description/custom-rule.html#add-deps
function rule_values.add_deps(deps, ...) end

---@param modules string|string[] Module name string or array
---@param ... string|string[] Variable parameters, can pass multiple module names
---@see https://xmake.io/api/description/custom-rule.html#add-imports
function rule_values.add_imports(modules, ...) end

---@param extensions string|string[] File extension string or array
---@param ... string|string[] Variable parameters, can pass multiple extensions
---@see https://xmake.io/api/description/custom-rule.html#set-extensions
function rule_values.set_extensions(extensions, ...) end

---@param kind string Target type string, specifies the type of compilation target
---@see https://xmake.io/api/description/project-target.html#set-kind
function rule_values.set_kind(kind) end

function rule_values.set_sourcekinds() end

---@class xmake.rule.groups
local rule_groups = {}

function rule_groups.add_orders() end

---@class xmake.rule.script
local rule_script = {}

---@param script fun(target: target) After build script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-build
function rule_script.after_build(script) end

---@param script fun(target: target, sourcefile: any, opt: any) After build file script function with target, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-build-file
function rule_script.after_build_file(script) end

---@param script fun(target: target, sourcebatch: any, opt: any) After build files script function with target, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-build-files
function rule_script.after_build_files(script) end

---@param script fun(package: package, batchcmds: any) After build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#after-buildcmd
function rule_script.after_buildcmd(script) end

---@param script fun(target: target, batchcmds: any, sourcefile: any, opt: any) After build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-buildcmd-file
function rule_script.after_buildcmd_file(script) end

---@param script fun(target: target, batchcmds: any, sourcebatch: any, opt: any) After build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-buildcmd-files
function rule_script.after_buildcmd_files(script) end

---@param script fun(target: target) After clean script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-clean
function rule_script.after_clean(script) end

---@param script fun(target: target) After config script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-config
function rule_script.after_config(script) end

---@param script fun(target: target) After install script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-install
function rule_script.after_install(script) end

---@param script fun(component: any, batchcmds: any) After install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#after-installcmd
function rule_script.after_installcmd(script) end

---@param script fun(target: target) After link script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-link
function rule_script.after_link(script) end

function rule_script.after_linkcmd() end

function rule_script.after_load() end

---@param script fun(target: target) After package script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-package
function rule_script.after_package(script) end

---@param script fun(target: target) After prepare script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-prepare
function rule_script.after_prepare(script) end

---@param script fun(target: target, sourcefile: any, opt: any) After prepare single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-prepare-file
function rule_script.after_prepare_file(script) end

---@param script fun(target: target, sourcebatch: any, opt: any) After prepare batch files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-prepare-files
function rule_script.after_prepare_files(script) end

function rule_script.after_preparecmd() end

function rule_script.after_preparecmd_file() end

function rule_script.after_preparecmd_files() end

---@param script fun(target: target) After run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-run
function rule_script.after_run(script) end

function rule_script.after_test() end

---@param script fun(target: target) After uninstall script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-uninstall
function rule_script.after_uninstall(script) end

---@param script fun(component: any, batchcmds: any) After uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#after-uninstallcmd
function rule_script.after_uninstallcmd(script) end

---@param script fun(target: target) Before build script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-build
function rule_script.before_build(script) end

---@param script fun(target: target, sourcefile: any, opt: any) Before build file script function with target, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-build-file
function rule_script.before_build_file(script) end

---@param script fun(target: target, sourcebatch: any, opt: any) Before build files script function with target, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-build-files
function rule_script.before_build_files(script) end

---@param script fun(package: package, batchcmds: any) Before build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#before-buildcmd
function rule_script.before_buildcmd(script) end

---@param script fun(target: target, batchcmds: any, sourcefile: any, opt: any) Before build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-buildcmd-file
function rule_script.before_buildcmd_file(script) end

---@param script fun(target: target, batchcmds: any, sourcebatch: any, opt: any) Before build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-buildcmd-files
function rule_script.before_buildcmd_files(script) end

---@param script fun(target: target) Before clean script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-clean
function rule_script.before_clean(script) end

---@param script fun(target: target) Before config script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-config
function rule_script.before_config(script) end

---@param script fun(target: target) Before install script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-install
function rule_script.before_install(script) end

---@param script fun(component: any, batchcmds: any) Before install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#before-installcmd
function rule_script.before_installcmd(script) end

---@param script fun(target: target) Before link script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-link
function rule_script.before_link(script) end

function rule_script.before_linkcmd() end

function rule_script.before_load() end

---@param script fun(target: target) Before package script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-package
function rule_script.before_package(script) end

---@param script fun(target: target) Before prepare script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-prepare
function rule_script.before_prepare(script) end

---@param script fun(target: target, sourcefile: any, opt: any) Before prepare single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-prepare-file
function rule_script.before_prepare_file(script) end

---@param script fun(target: target, sourcebatch: any, opt: any) Before prepare batch files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-prepare-files
function rule_script.before_prepare_files(script) end

function rule_script.before_preparecmd() end

function rule_script.before_preparecmd_file() end

function rule_script.before_preparecmd_files() end

---@param script fun(target: target) Before run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-run
function rule_script.before_run(script) end

function rule_script.before_test() end

---@param script fun(target: target) Before uninstall script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-uninstall
function rule_script.before_uninstall(script) end

---@param script fun(component: any, batchcmds: any) Before uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#before-uninstallcmd
function rule_script.before_uninstallcmd(script) end

---@param script fun(target: target) Build script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-build
function rule_script.on_build(script) end

---@param script fun(target: target, sourcefile: any, opt: any) Build file script function with target, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-build-file
function rule_script.on_build_file(script) end

---@param script fun(target: target, sourcebatch: any, opt: any) Build files script function with target, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-build-files
function rule_script.on_build_files(script) end

---@param script fun(package: package, batchcmds: any) Build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#on-buildcmd
function rule_script.on_buildcmd(script) end

---@param script fun(target: target, batchcmds: any, sourcefile: any, opt: any) Build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-buildcmd-file
function rule_script.on_buildcmd_file(script) end

---@param script fun(target: target, batchcmds: any, sourcebatch: any, opt: any) Build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-buildcmd-files
function rule_script.on_buildcmd_files(script) end

---@param script fun(target: target) Clean script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-clean
function rule_script.on_clean(script) end

---@param script fun(target: target) Configuration script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-config
function rule_script.on_config(script) end

---@param script fun(target: target) Install script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-install
function rule_script.on_install(script) end

---@param script fun(component: any, batchcmds: any) Install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#on-installcmd
function rule_script.on_installcmd(script) end

---@param script fun(target: target) Link script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-link
function rule_script.on_link(script) end

function rule_script.on_linkcmd() end

---@param script fun(target: target) Load script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-load
function rule_script.on_load(script) end

---@param script fun(target: target) Package script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-package
function rule_script.on_package(script) end

---@param script fun(target: target, opt: any) Prepare phase script function, receives target, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-prepare
function rule_script.on_prepare(script) end

---@param script fun(target: target, sourcefile: any, opt: any) Single file processing script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-prepare-file
function rule_script.on_prepare_file(script) end

---@param script fun(target: target, jobgraph: any, sourcebatch: any, opt: any) Batch file processing script function, receives target, jobgraph, sourcebatch, opt parameters
---@param opts? {jobgraph?: any}
---@see https://xmake.io/api/description/project-target.html#on-prepare-files
function rule_script.on_prepare_files(script, opts) end

function rule_script.on_preparecmd() end

function rule_script.on_preparecmd_file() end

function rule_script.on_preparecmd_files() end

---@param script fun(target: target) Run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-run
function rule_script.on_run(script) end

---@param script fun(package: package) Test script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-test
function rule_script.on_test(script) end

---@param script fun(target: target) Uninstall script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-uninstall
function rule_script.on_uninstall(script) end

---@param script fun(component: any, batchcmds: any) Uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#on-uninstallcmd
function rule_script.on_uninstallcmd(script) end
