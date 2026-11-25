---@meta

--- JSON encoding and decoding.
---
--- # Examples
---
--- ```lua
--- import("core.base.json")
--- local data = json.loadfile("package.json")
--- json.savefile("out.json", {name = data.name})
--- ```
---@class core.base.json
---@see https://xmake.io/api/scripts/extension-modules/core/base/json.html
local json = {}

--- Decodes a JSON string.
---@param text string
---@return any value
---@see https://xmake.io/api/scripts/extension-modules/core/base/json.html#json-decode
---@xmake.context import
function json.decode(text) end

--- Encodes a value as JSON.
---
--- # Notes
---
--- Empty tables encode as objects; mark arrays with `json.mark_as_array`.
---@param value any
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/base/json.html#json-encode
---@xmake.context import
function json.encode(value) end

--- Reads and decodes a JSON file.
---@param path string
---@return any value
---@see https://xmake.io/api/scripts/extension-modules/core/base/json.html#json-loadfile
---@xmake.context import
function json.loadfile(path) end

--- Encodes a value and writes it to a file.
---@param path string
---@param value any
---@see https://xmake.io/api/scripts/extension-modules/core/base/json.html#json-savefile
---@xmake.context import
function json.savefile(path, value) end

--- Marks a table to encode as an array, even when empty.
---@param value table
---@return table
---@xmake.context import
function json.mark_as_array(value) end

---@xmake.context import
function json.is_marked_as_array(...) end

--- The command line options of xmake and of tasks.
---@class core.base.option
---@see https://xmake.io/api/scripts/extension-modules/core/base/option.html
local option = {}

--- Returns the value of a command line option.
---
--- # Examples
---
--- ```lua
--- import("core.base.option")
--- if option.get("verbose") then
---     print("verbose mode")
--- end
--- ```
---@param name string
---@return any
---@see https://xmake.io/api/scripts/extension-modules/core/base/option.html#option-get
---@xmake.context import
function option.get(name) end

---@xmake.context import
function option.boolean(...) end

---@xmake.context import
function option.default(...) end

---@xmake.context import
function option.defaults(...) end

---@xmake.context import
function option.options(...) end

---@xmake.context import
function option.parse(...) end

---@xmake.context import
function option.raw_parse(...) end

---@xmake.context import
function option.restore(...) end

---@xmake.context import
function option.save(...) end

---@xmake.context import
function option.set(...) end

---@xmake.context import
function option.show_logo(...) end

---@xmake.context import
function option.show_options(...) end

---@xmake.context import
function option.taskmenu(...) end

---@xmake.context import
function option.taskname(...) end

--- The configuration of the project (`xmake f`).
---@class core.project.config
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html
local config = {}

--- Returns a configuration value, e.g. `plat`, `arch`, `mode`.
---
--- # Examples
---
--- ```lua
--- import("core.project.config")
--- config.load()
--- print(config.get("mode"))
--- ```
---@param name string
---@return any
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-get
---@xmake.context import
function config.get(name) end

--- Sets a configuration value.
---@param name string
---@param value any
---@param opt? {readonly?: boolean, force?: boolean}
---@xmake.context import
function config.set(name, value, opt) end

--- Loads the configuration of the project.
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-load
---@xmake.context import
function config.load() end

--- Returns the build directory.
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-builddir
---@xmake.context import
function config.builddir() end

--- Returns the platform being configured.
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-plat
---@xmake.context import
function config.plat() end

--- Returns the architecture being configured.
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-arch
---@xmake.context import
function config.arch() end

--- Returns the build mode being configured.
---@return string
---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-mode
---@xmake.context import
function config.mode() end

---@xmake.context import
function config.buildir(...) end

---@xmake.context import
function config.clear(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-directory
---@xmake.context import
function config.directory(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/project/config.html#config-dump
---@xmake.context import
function config.dump(...) end

---@xmake.context import
function config.filepath(...) end

---@xmake.context import
function config.host(...) end

---@xmake.context import
function config.read(...) end

---@xmake.context import
function config.readonly(...) end

---@xmake.context import
function config.save(...) end

--- The project: its targets, options and requirements.
---@class core.project.project
---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html
local project = {}

--- Returns the target `name`.
---
--- # Examples
---
--- ```lua
--- import("core.project.project")
--- local app = project.target("app")
--- print(app:targetfile())
--- ```
---@param name string
---@return target?
---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-target
---@xmake.context import
function project.target(name) end

--- Returns the targets of the project, by name.
---@return table<string, target>
---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-targets
---@xmake.context import
function project.targets() end

--- Returns the root directory of the project.
---@return string
---@xmake.context import
function project.directory() end

---@xmake.context import
function project.allfiles(...) end

---@xmake.context import
function project.allowed_archs(...) end

---@xmake.context import
function project.allowed_modes(...) end

---@xmake.context import
function project.allowed_plats(...) end

---@xmake.context import
function project.apis(...) end

---@xmake.context import
function project.chdir(...) end

---@xmake.context import
function project.check_options(...) end

---@xmake.context import
function project.default_arch(...) end

---@xmake.context import
function project.extraconf(...) end

---@xmake.context import
function project.filelock(...) end

---@xmake.context import
function project.get(...) end

---@xmake.context import
function project.is_loaded(...) end

---@xmake.context import
function project.load_targets(...) end

---@xmake.context import
function project.lock(...) end

---@xmake.context import
function project.modes(...) end

---@xmake.context import
function project.mtimes(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-name
---@xmake.context import
function project.name(...) end

---@xmake.context import
function project.namespaces(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-option
---@xmake.context import
function project.option(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-options
---@xmake.context import
function project.options(...) end

---@xmake.context import
function project.ordertargets(...) end

---@xmake.context import
function project.policy(...) end

---@xmake.context import
function project.policy_set(...) end

---@xmake.context import
function project.rcfiles(...) end

---@xmake.context import
function project.requireconfs_str(...) end

---@xmake.context import
function project.required_package(...) end

---@xmake.context import
function project.required_packages(...) end

---@xmake.context import
function project.requires_str(...) end

---@xmake.context import
function project.requireslock(...) end

---@xmake.context import
function project.requireslock_version(...) end

---@xmake.context import
function project.rootfile(...) end

---@xmake.context import
function project.rule(...) end

---@xmake.context import
function project.rules(...) end

---@xmake.context import
function project.scope(...) end

---@xmake.context import
function project.target_add(...) end

---@xmake.context import
function project.tmpdir(...) end

---@xmake.context import
function project.tmpfile(...) end

---@xmake.context import
function project.toolchain(...) end

---@xmake.context import
function project.toolchains(...) end

---@xmake.context import
function project.trylock(...) end

---@xmake.context import
function project.unlock(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/project/project.html#project-version
---@xmake.context import
function project.version(...) end

--- Finds a program and returns its path and version.
---
--- # Examples
---
--- ```lua
--- import("lib.detect.find_tool")
--- local git = find_tool("git", {version = true})
--- if git then
---     print(git.program, git.version)
--- end
--- ```
---@class lib.detect.find_tool
---@overload fun(name: string, opt?: {version?: boolean, paths?: string[], check?: string}): {program: string, version?: string}?
local find_tool = {}

--- Finds a program and returns its path.
---@class lib.detect.find_program
---@overload fun(name: string, opt?: {paths?: string[], check?: string}): string?
local find_program = {}

---@class async
local async = {}

---@see https://xmake.io/api/scripts/extension-modules/async/runjobs.html
---@xmake.context import
function async.runjobs(...) end

---@class async.jobgraph
local jobgraph = {}

---@see https://xmake.io/api/scripts/extension-modules/async/jobgraph.html#jobgraph-new
---@xmake.context import
function jobgraph.new(...) end

---@class cli
local cli = {}

---@see https://xmake.io/api/scripts/extension-modules/cli/amalgamate.html
---@xmake.context import
function cli.amalgamate(...) end

---@xmake.context import
function cli.archive(...) end

---@xmake.context import
function cli.bisect(...) end

---@xmake.context import
function cli.extract(...) end

---@see https://xmake.io/api/scripts/extension-modules/cli/iconv.html
---@xmake.context import
function cli.iconv(...) end

---@class cli.binutils
local binutils = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/binutils.html#binutils-bin2c
---@xmake.context import
function binutils.bin2c(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/binutils.html#binutils-bin2obj
---@xmake.context import
function binutils.bin2obj(...) end

---@class core.base
local base = {}

---@xmake.context import
function base.bytes(...) end

---@xmake.context import
function base.dlist(...) end

---@xmake.context import
function base.graph(...) end

---@xmake.context import
function base.list(...) end

---@xmake.context import
function base.object(...) end

---@xmake.context import
function base.queue(...) end

---@class core.base.base64
local base64 = {}

---@xmake.context import
function base64.decode(...) end

---@xmake.context import
function base64.encode(...) end

---@class core.base.binutils
local binutils2 = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/binutils.html#binutils-bin2c
---@xmake.context import
function binutils2.bin2c(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/binutils.html#binutils-bin2obj
---@xmake.context import
function binutils2.bin2obj(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/binutils.html#binutils-deplibs
---@xmake.context import
function binutils2.deplibs(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/binutils.html#binutils-extractlib
---@xmake.context import
function binutils2.extractlib(...) end

---@xmake.context import
function binutils2.format(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/binutils.html#binutils-readsyms
---@xmake.context import
function binutils2.readsyms(...) end

---@xmake.context import
function binutils2.rpath_clean(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/binutils.html#binutils-rpath-list
---@xmake.context import
function binutils2.rpath_list(...) end

---@class core.base.bit
local bit = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/bit.html#bit-band
---@xmake.context import
function bit.band(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/bit.html#bit-bnot
---@xmake.context import
function bit.bnot(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/bit.html#bit-bor
---@xmake.context import
function bit.bor(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/bit.html#bit-bxor
---@xmake.context import
function bit.bxor(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/bit.html#bit-lshift
---@xmake.context import
function bit.lshift(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/bit.html#bit-rshift
---@xmake.context import
function bit.rshift(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/bit.html#bit-tobit
---@xmake.context import
function bit.tobit(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/bit.html#bit-tohex
---@xmake.context import
function bit.tohex(...) end

---@class core.base.bloom_filter
local bloom_filter = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/bloom_filter.html#bloom-filter-new
---@xmake.context import
function bloom_filter.new(...) end

---@class core.base.cli
local cli2 = {}

---@xmake.context import
function cli2.parse(...) end

---@xmake.context import
function cli2.parsev(...) end

---@class core.base.colors
local colors = {}

---@xmake.context import
function colors.ignore(...) end

---@xmake.context import
function colors.rainbow24(...) end

---@xmake.context import
function colors.rainbow256(...) end

---@xmake.context import
function colors.theme(...) end

---@xmake.context import
function colors.theme_set(...) end

---@xmake.context import
function colors.translate(...) end

---@class core.base.cpu
local cpu = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/cpu.html#cpu-family
---@xmake.context import
function cpu.family(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/cpu.html#cpu-features
---@xmake.context import
function cpu.features(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/cpu.html#cpu-has-feature
---@xmake.context import
function cpu.has_feature(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/cpu.html#cpu-info
---@xmake.context import
function cpu.info(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/cpu.html#cpu-march
---@xmake.context import
function cpu.march(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/cpu.html#cpu-model
---@xmake.context import
function cpu.model(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/cpu.html#cpu-model-name
---@xmake.context import
function cpu.model_name(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/cpu.html#cpu-number
---@xmake.context import
function cpu.number(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/cpu.html#cpu-usagerate
---@xmake.context import
function cpu.usagerate(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/cpu.html#cpu-vendor
---@xmake.context import
function cpu.vendor(...) end

---@class core.base.filter
local filter = {}

---@xmake.context import
function filter.new(...) end

---@class core.base.fwatcher
local fwatcher = {}

---@xmake.context import
function fwatcher.add(...) end

---@xmake.context import
function fwatcher.on_created(...) end

---@xmake.context import
function fwatcher.on_deleted(...) end

---@xmake.context import
function fwatcher.on_modified(...) end

---@xmake.context import
function fwatcher.remove(...) end

---@xmake.context import
function fwatcher.wait(...) end

---@xmake.context import
function fwatcher.watchdirs(...) end

---@class core.base.heap
local heap = {}

---@xmake.context import
function heap.cdataheap(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/heap.html#heap-valueheap
---@xmake.context import
function heap.valueheap(...) end

---@class core.base.interpreter
local interpreter = {}

---@xmake.context import
function interpreter.builtin_modules(...) end

---@xmake.context import
function interpreter.instance(...) end

---@class core.base.libc
local libc = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/libc.html#libc-byteof
---@xmake.context import
function libc.byteof(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/libc.html#libc-dataptr
---@xmake.context import
function libc.dataptr(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/libc.html#libc-free
---@xmake.context import
function libc.free(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/libc.html#libc-malloc
---@xmake.context import
function libc.malloc(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/libc.html#libc-memcpy
---@xmake.context import
function libc.memcpy(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/libc.html#libc-memmov
---@xmake.context import
function libc.memmov(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/libc.html#libc-memset
---@xmake.context import
function libc.memset(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/libc.html#libc-ptraddr
---@xmake.context import
function libc.ptraddr(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/libc.html#libc-setbyte
---@xmake.context import
function libc.setbyte(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/libc.html#libc-strndup
---@xmake.context import
function libc.strndup(...) end

---@class core.base.license
local license = {}

---@xmake.context import
function license.compatible(...) end

---@xmake.context import
function license.list(...) end

---@xmake.context import
function license.normalize(...) end

---@class core.base.memory
local memory = {}

---@xmake.context import
function memory.info(...) end

---@class core.base.pipe
local pipe = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/pipe.html#pipe-open
---@xmake.context import
function pipe.open(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/pipe.html#pipe-openpair
---@xmake.context import
function pipe.openpair(...) end

---@class core.base.privilege
local privilege = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/privilege.html#privilege-get
---@xmake.context import
function privilege.get(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/privilege.html#privilege-has
---@xmake.context import
function privilege.has(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/privilege.html#privilege-store
---@xmake.context import
function privilege.store(...) end

---@class core.base.process
local process = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/process.html#process-open
---@xmake.context import
function process.open(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/process.html#process-openv
---@xmake.context import
function process.openv(...) end

---@class core.base.profiler
local profiler = {}

---@xmake.context import
function profiler.enter(...) end

---@xmake.context import
function profiler.leave(...) end

---@class core.base.scheduler
local scheduler = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-count
---@xmake.context import
function scheduler.co_count(...) end

---@xmake.context import
function scheduler.co_group(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-group-begin
---@xmake.context import
function scheduler.co_group_begin(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-group-wait
---@xmake.context import
function scheduler.co_group_wait(...) end

---@xmake.context import
function scheduler.co_group_waitobjs(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-lock
---@xmake.context import
function scheduler.co_lock(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-resume
---@xmake.context import
function scheduler.co_resume(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-running
---@xmake.context import
function scheduler.co_running(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-semaphore
---@xmake.context import
function scheduler.co_semaphore(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-sleep
---@xmake.context import
function scheduler.co_sleep(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-start
---@xmake.context import
function scheduler.co_start(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-start-named
---@xmake.context import
function scheduler.co_start_named(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-start-withopt
---@xmake.context import
function scheduler.co_start_withopt(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-suspend
---@xmake.context import
function scheduler.co_suspend(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-unlock
---@xmake.context import
function scheduler.co_unlock(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/scheduler.html#scheduler-co-yield
---@xmake.context import
function scheduler.co_yield(...) end

---@class core.base.signal
local signal = {}

---@see https://xmake.io/api/scripts/builtin-modules/signal.html#signal-ignore
---@xmake.context import
function signal.ignore(...) end

---@see https://xmake.io/api/scripts/builtin-modules/signal.html#signal-register
---@xmake.context import
function signal.register(...) end

---@see https://xmake.io/api/scripts/builtin-modules/signal.html#signal-reset
---@xmake.context import
function signal.reset(...) end

---@class core.base.singleton
local singleton = {}

---@xmake.context import
function singleton.get(...) end

---@xmake.context import
function singleton.instances(...) end

---@class core.base.socket
local socket = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-bind
---@xmake.context import
function socket.bind(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-bind-unix
---@xmake.context import
function socket.bind_unix(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-connect
---@xmake.context import
function socket.connect(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-connect-unix
---@xmake.context import
function socket.connect_unix(...) end

---@xmake.context import
function socket.open(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-tcp
---@xmake.context import
function socket.tcp(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-udp
---@xmake.context import
function socket.udp(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/socket.html#socket-unix
---@xmake.context import
function socket.unix(...) end

---@class core.base.text
local text = {}

---@xmake.context import
function text.table(...) end

---@xmake.context import
function text.wordwrap(...) end

---@class core.base.thread
local thread = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-event
---@xmake.context import
function thread.event(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-mutex
---@xmake.context import
function thread.mutex(...) end

---@xmake.context import
function thread.new(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-queue
---@xmake.context import
function thread.queue(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-running
---@xmake.context import
function thread.running(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-semaphore
---@xmake.context import
function thread.semaphore(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-sharedata
---@xmake.context import
function thread.sharedata(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-start
---@xmake.context import
function thread.start(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/thread.html#thread-start-named
---@xmake.context import
function thread.start_named(...) end

---@xmake.context import
function thread.start_withopt(...) end

---@class core.base.tty
local tty = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-cr
---@xmake.context import
function tty.cr(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-cursor-and-attrs-restore
---@xmake.context import
function tty.cursor_and_attrs_restore(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-cursor-and-attrs-save
---@xmake.context import
function tty.cursor_and_attrs_save(...) end

---@xmake.context import
function tty.cursor_hide(...) end

---@xmake.context import
function tty.cursor_move(...) end

---@xmake.context import
function tty.cursor_move_down(...) end

---@xmake.context import
function tty.cursor_move_left(...) end

---@xmake.context import
function tty.cursor_move_right(...) end

---@xmake.context import
function tty.cursor_move_to_col(...) end

---@xmake.context import
function tty.cursor_move_up(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-cursor-restore
---@xmake.context import
function tty.cursor_restore(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-cursor-save
---@xmake.context import
function tty.cursor_save(...) end

---@xmake.context import
function tty.cursor_show(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-down
---@xmake.context import
function tty.erase_down(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-line
---@xmake.context import
function tty.erase_line(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-line-to-end
---@xmake.context import
function tty.erase_line_to_end(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-line-to-start
---@xmake.context import
function tty.erase_line_to_start(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-screen
---@xmake.context import
function tty.erase_screen(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-erase-up
---@xmake.context import
function tty.erase_up(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-flush
---@xmake.context import
function tty.flush(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-has-color24
---@xmake.context import
function tty.has_color24(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-has-color256
---@xmake.context import
function tty.has_color256(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-has-color8
---@xmake.context import
function tty.has_color8(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-has-emoji
---@xmake.context import
function tty.has_emoji(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-has-vtansi
---@xmake.context import
function tty.has_vtansi(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-session-id
---@xmake.context import
function tty.session_id(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-shell
---@xmake.context import
function tty.shell(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-term
---@xmake.context import
function tty.term(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/tty.html#tty-term-mode
---@xmake.context import
function tty.term_mode(...) end

---@class core.base.xml
local xml = {}

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-cdata
---@xmake.context import
function xml.cdata(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-comment
---@xmake.context import
function xml.comment(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-decode
---@xmake.context import
function xml.decode(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-doctype
---@xmake.context import
function xml.doctype(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-empty
---@xmake.context import
function xml.empty(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-encode
---@xmake.context import
function xml.encode(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-find
---@xmake.context import
function xml.find(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-loadfile
---@xmake.context import
function xml.loadfile(...) end

---@xmake.context import
function xml.new(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-savefile
---@xmake.context import
function xml.savefile(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-scan
---@xmake.context import
function xml.scan(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-text
---@xmake.context import
function xml.text(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/xml.html#xml-text-of
---@xmake.context import
function xml.text_of(...) end

---@class core.cache.detectcache
local detectcache = {}

---@see https://xmake.io/api/scripts/extension-modules/core/cache/detectcache.html#detectcache-clear
---@xmake.context import
function detectcache.clear(...) end

---@xmake.context import
function detectcache.data(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/detectcache.html#detectcache-get
---@xmake.context import
function detectcache.get(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/detectcache.html#detectcache-get2
---@xmake.context import
function detectcache.get2(...) end

---@xmake.context import
function detectcache.get3(...) end

---@xmake.context import
function detectcache.load(...) end

---@xmake.context import
function detectcache.name(...) end

---@xmake.context import
function detectcache.new(...) end

---@xmake.context import
function detectcache.save(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/detectcache.html#detectcache-set
---@xmake.context import
function detectcache.set(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/detectcache.html#detectcache-set2
---@xmake.context import
function detectcache.set2(...) end

---@xmake.context import
function detectcache.set3(...) end

---@class core.cache.global_detectcache
local global_detectcache = {}

---@see https://xmake.io/api/scripts/extension-modules/core/cache/global_detectcache.html#global-detectcache-clear
---@xmake.context import
function global_detectcache.clear(...) end

---@xmake.context import
function global_detectcache.data(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/global_detectcache.html#global-detectcache-get
---@xmake.context import
function global_detectcache.get(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/global_detectcache.html#global-detectcache-get2
---@xmake.context import
function global_detectcache.get2(...) end

---@xmake.context import
function global_detectcache.get3(...) end

---@xmake.context import
function global_detectcache.load(...) end

---@xmake.context import
function global_detectcache.name(...) end

---@xmake.context import
function global_detectcache.new(...) end

---@xmake.context import
function global_detectcache.save(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/global_detectcache.html#global-detectcache-set
---@xmake.context import
function global_detectcache.set(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/global_detectcache.html#global-detectcache-set2
---@xmake.context import
function global_detectcache.set2(...) end

---@xmake.context import
function global_detectcache.set3(...) end

---@class core.cache.globalcache
local globalcache = {}

---@xmake.context import
function globalcache.cache(...) end

---@xmake.context import
function globalcache.caches(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/globalcache.html#globalcache-clear
---@xmake.context import
function globalcache.clear(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/globalcache.html#globalcache-get
---@xmake.context import
function globalcache.get(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/globalcache.html#globalcache-get2
---@xmake.context import
function globalcache.get2(...) end

---@xmake.context import
function globalcache.get3(...) end

---@xmake.context import
function globalcache.save(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/globalcache.html#globalcache-set
---@xmake.context import
function globalcache.set(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/globalcache.html#globalcache-set2
---@xmake.context import
function globalcache.set2(...) end

---@xmake.context import
function globalcache.set3(...) end

---@class core.cache.localcache
local localcache = {}

---@xmake.context import
function localcache.cache(...) end

---@xmake.context import
function localcache.caches(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/localcache.html#localcache-clear
---@xmake.context import
function localcache.clear(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/localcache.html#localcache-get
---@xmake.context import
function localcache.get(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/localcache.html#localcache-get2
---@xmake.context import
function localcache.get2(...) end

---@xmake.context import
function localcache.get3(...) end

---@xmake.context import
function localcache.save(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/localcache.html#localcache-set
---@xmake.context import
function localcache.set(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/localcache.html#localcache-set2
---@xmake.context import
function localcache.set2(...) end

---@xmake.context import
function localcache.set3(...) end

---@class core.cache.memcache
local memcache = {}

---@xmake.context import
function memcache.cache(...) end

---@xmake.context import
function memcache.caches(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/memcache.html#memcache-clear
---@xmake.context import
function memcache.clear(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/memcache.html#memcache-get
---@xmake.context import
function memcache.get(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/memcache.html#memcache-get2
---@xmake.context import
function memcache.get2(...) end

---@xmake.context import
function memcache.get3(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/memcache.html#memcache-set
---@xmake.context import
function memcache.set(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/cache/memcache.html#memcache-set2
---@xmake.context import
function memcache.set2(...) end

---@xmake.context import
function memcache.set3(...) end

---@class core.compress.lz4
local lz4 = {}

---@see https://xmake.io/api/scripts/extension-modules/core/compress/lz4.html#lz4-block-compress
---@xmake.context import
function lz4.block_compress(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/compress/lz4.html#lz4-block-decompress
---@xmake.context import
function lz4.block_decompress(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/compress/lz4.html#lz4-compress
---@xmake.context import
function lz4.compress(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/compress/lz4.html#lz4-compress-file
---@xmake.context import
function lz4.compress_file(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/compress/lz4.html#lz4-compress-stream
---@xmake.context import
function lz4.compress_stream(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/compress/lz4.html#lz4-decompress
---@xmake.context import
function lz4.decompress(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/compress/lz4.html#lz4-decompress-file
---@xmake.context import
function lz4.decompress_file(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/compress/lz4.html#lz4-decompress-stream
---@xmake.context import
function lz4.decompress_stream(...) end

---@class core.language.language
local language = {}

---@xmake.context import
function language.apis(...) end

---@xmake.context import
function language.extension_of(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/language/language.html#language-extensions
---@xmake.context import
function language.extensions(...) end

---@xmake.context import
function language.langkinds(...) end

---@xmake.context import
function language.linkerinfos_of(...) end

---@xmake.context import
function language.linkerkinds(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/language/language.html#language-load
---@xmake.context import
function language.load(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/language/language.html#language-load-ex
---@xmake.context import
function language.load_ex(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/language/language.html#language-load-sk
---@xmake.context import
function language.load_sk(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/language/language.html#language-sourceflags
---@xmake.context import
function language.sourceflags(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/language/language.html#language-sourcekind-of
---@xmake.context import
function language.sourcekind_of(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/language/language.html#language-sourcekinds
---@xmake.context import
function language.sourcekinds(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/language/language.html#language-targetkinds
---@xmake.context import
function language.targetkinds(...) end

---@class core.language.menu
local menu = {}

---@xmake.context import
function menu.options(...) end

---@class core.package.addon
local addon = {}

---@xmake.context import
function addon.addons(...) end

---@xmake.context import
function addon.dirname(...) end

---@xmake.context import
function addon.installdir(...) end

---@xmake.context import
function addon.manifest(...) end

---@xmake.context import
function addon.owner(...) end

---@xmake.context import
function addon.payloadinfos(...) end

---@xmake.context import
function addon.payloadroot(...) end

---@xmake.context import
function addon.payloads_of(...) end

---@xmake.context import
function addon.register(...) end

---@xmake.context import
function addon.remove(...) end

---@xmake.context import
function addon.resolve_reference(...) end

---@xmake.context import
function addon.unregister(...) end

---@xmake.context import
function addon.versions(...) end

---@xmake.context import
function addon.workdir(...) end

---@class core.package.package
local package = {}

---@xmake.context import
function package.apis(...) end

---@xmake.context import
function package.cachedir(...) end

---@xmake.context import
function package.installdir(...) end

---@xmake.context import
function package.load_from_project(...) end

---@xmake.context import
function package.load_from_repository(...) end

---@xmake.context import
function package.load_from_system(...) end

---@xmake.context import
function package.new(...) end

---@xmake.context import
function package.searchdirs(...) end

---@xmake.context import
function package.targetarch(...) end

---@xmake.context import
function package.targetplat(...) end

---@class core.package.repository
local repository = {}

---@xmake.context import
function repository.add(...) end

---@xmake.context import
function repository.clear(...) end

---@xmake.context import
function repository.directory(...) end

---@xmake.context import
function repository.get(...) end

---@xmake.context import
function repository.load(...) end

---@xmake.context import
function repository.remove(...) end

---@xmake.context import
function repository.repositories(...) end

---@class core.platform.menu
local menu2 = {}

---@xmake.context import
function menu2.options(...) end

---@class core.platform.platform
local platform = {}

---@xmake.context import
function platform.archs(...) end

---@xmake.context import
function platform.get(...) end

---@xmake.context import
function platform.load(...) end

---@xmake.context import
function platform.os(...) end

---@xmake.context import
function platform.plats(...) end

---@xmake.context import
function platform.tool(...) end

---@xmake.context import
function platform.toolchains(...) end

---@xmake.context import
function platform.toolconfig(...) end

---@class core.project.addons
local addons = {}

---@xmake.context import
function addons.locked(...) end

---@xmake.context import
function addons.locked_valid(...) end

---@xmake.context import
function addons.lockfile(...) end

---@xmake.context import
function addons.lockfile_version(...) end

---@xmake.context import
function addons.requirename(...) end

---@class core.project.depend
local depend = {}

---@xmake.context import
function depend.is_changed(...) end

---@xmake.context import
function depend.load(...) end

---@xmake.context import
function depend.on_changed(...) end

---@xmake.context import
function depend.save(...) end

---@class core.project.menu
local menu3 = {}

---@xmake.context import
function menu3.options(...) end

---@class core.project.option
local option2 = {}

---@xmake.context import
function option2.apis(...) end

---@xmake.context import
function option2.interpreter(...) end

---@xmake.context import
function option2.new(...) end

---@class core.project.policy
local policy = {}

---@xmake.context import
function policy.build_warnings(...) end

---@xmake.context import
function policy.policies(...) end

---@class core.project.rule
local rule = {}

---@xmake.context import
function rule.apis(...) end

---@xmake.context import
function rule.new(...) end

---@xmake.context import
function rule.rule(...) end

---@xmake.context import
function rule.rules(...) end

---@class core.project.target
local target = {}

---@xmake.context import
function target.apis(...) end

---@xmake.context import
function target.filename(...) end

---@xmake.context import
function target.linkname(...) end

---@xmake.context import
function target.new(...) end

---@class core.project.task
local task = {}

---@xmake.context import
function task.names(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/base/task.html#task-run
---@xmake.context import
function task.run(...) end

---@class core.sandbox.module
local module = {}

---@xmake.context import
function module.add_directories(...) end

---@xmake.context import
function module.coredir(...) end

---@xmake.context import
function module.directories(...) end

---@xmake.context import
function module.find(...) end

---@xmake.context import
function module.get(...) end

---@xmake.context import
function module.import(...) end

---@xmake.context import
function module.inherit(...) end

---@xmake.context import
function module.name(...) end

---@class core.sandbox.sandbox
local sandbox = {}

---@xmake.context import
function sandbox.builtin_modules(...) end

---@xmake.context import
function sandbox.filter(...) end

---@xmake.context import
function sandbox.fork(...) end

---@xmake.context import
function sandbox.interactive(...) end

---@class core.theme.theme
local theme = {}

---@xmake.context import
function theme.get(...) end

---@xmake.context import
function theme.instance(...) end

---@xmake.context import
function theme.load(...) end

---@xmake.context import
function theme.names(...) end

---@class core.tool.compiler
local compiler = {}

---@xmake.context import
function compiler.build(...) end

---@xmake.context import
function compiler.buildargv(...) end

---@xmake.context import
function compiler.buildcmd(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/compiler.html#compiler-compargv
---@xmake.context import
function compiler.compargv(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/compiler.html#compiler-compcmd
---@xmake.context import
function compiler.compcmd(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/compiler.html#compiler-compflags
---@xmake.context import
function compiler.compflags(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/compiler.html#compiler-compile
---@xmake.context import
function compiler.compile(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/compiler.html#compiler-features
---@xmake.context import
function compiler.features(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/compiler.html#compiler-has-features
---@xmake.context import
function compiler.has_features(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/compiler.html#compiler-has-flags
---@xmake.context import
function compiler.has_flags(...) end

---@xmake.context import
function compiler.load(...) end

---@xmake.context import
function compiler.map_flags(...) end

---@xmake.context import
function compiler.name(...) end

---@class core.tool.linker
local linker = {}

---@see https://xmake.io/api/scripts/extension-modules/core/tool/linker.html#linker-has-flags
---@xmake.context import
function linker.has_flags(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/linker.html#linker-link
---@xmake.context import
function linker.link(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/linker.html#linker-linkargv
---@xmake.context import
function linker.linkargv(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/linker.html#linker-linkcmd
---@xmake.context import
function linker.linkcmd(...) end

---@see https://xmake.io/api/scripts/extension-modules/core/tool/linker.html#linker-linkflags
---@xmake.context import
function linker.linkflags(...) end

---@xmake.context import
function linker.load(...) end

---@xmake.context import
function linker.map_flags(...) end

---@class core.tool.toolchain
local toolchain = {}

---@xmake.context import
function toolchain.apis(...) end

---@xmake.context import
function toolchain.directories(...) end

---@xmake.context import
function toolchain.list(...) end

---@xmake.context import
function toolchain.load(...) end

---@xmake.context import
function toolchain.save(...) end

---@class core.ui
local ui = {}

---@xmake.context import
function ui.action(...) end

---@xmake.context import
function ui.application(...) end

---@xmake.context import
function ui.border(...) end

---@xmake.context import
function ui.boxdialog(...) end

---@xmake.context import
function ui.button(...) end

---@xmake.context import
function ui.canvas(...) end

---@xmake.context import
function ui.choicebox(...) end

---@xmake.context import
function ui.choicedialog(...) end

---@xmake.context import
function ui.desktop(...) end

---@xmake.context import
function ui.dialog(...) end

---@xmake.context import
function ui.event(...) end

---@xmake.context import
function ui.inputdialog(...) end

---@xmake.context import
function ui.label(...) end

---@xmake.context import
function ui.mconfdialog(...) end

---@xmake.context import
function ui.menubar(...) end

---@xmake.context import
function ui.menuconf(...) end

---@xmake.context import
function ui.object(...) end

---@xmake.context import
function ui.panel(...) end

---@xmake.context import
function ui.point(...) end

---@xmake.context import
function ui.program(...) end

---@xmake.context import
function ui.rect(...) end

---@xmake.context import
function ui.statusbar(...) end

---@xmake.context import
function ui.textarea(...) end

---@xmake.context import
function ui.textdialog(...) end

---@xmake.context import
function ui.textedit(...) end

---@xmake.context import
function ui.view(...) end

---@xmake.context import
function ui.window(...) end

---@class core.ui.curses
local curses = {}

---@xmake.context import
function curses.acs(...) end

---@xmake.context import
function curses.attr(...) end

---@xmake.context import
function curses.calc_attr(...) end

---@xmake.context import
function curses.cbreak(...) end

---@xmake.context import
function curses.color(...) end

---@xmake.context import
function curses.color_pair(...) end

---@xmake.context import
function curses.columns(...) end

---@xmake.context import
function curses.cursor_set(...) end

---@xmake.context import
function curses.done(...) end

---@xmake.context import
function curses.doupdate(...) end

---@xmake.context import
function curses.echo(...) end

---@xmake.context import
function curses.getmouse(...) end

---@xmake.context import
function curses.has_colors(...) end

---@xmake.context import
function curses.has_mouse(...) end

---@xmake.context import
function curses.init(...) end

---@xmake.context import
function curses.init_pair(...) end

---@xmake.context import
function curses.iscolor(...) end

---@xmake.context import
function curses.isdone(...) end

---@xmake.context import
function curses.lines(...) end

---@xmake.context import
function curses.main_window(...) end

---@xmake.context import
function curses.mousemask(...) end

---@xmake.context import
function curses.napms(...) end

---@xmake.context import
function curses.new_pad(...) end

---@xmake.context import
function curses.nl(...) end

---@xmake.context import
function curses.start_color(...) end

---@class core.ui.log
local log = {}

---@xmake.context import
function log.clear(...) end

---@xmake.context import
function log.close(...) end

---@xmake.context import
function log.enable(...) end

---@xmake.context import
function log.file(...) end

---@xmake.context import
function log.flush(...) end

---@xmake.context import
function log.outputfile(...) end

---@xmake.context import
function log.print(...) end

---@xmake.context import
function log.printf(...) end

---@xmake.context import
function log.printv(...) end

---@xmake.context import
function log.write(...) end

---@class detect.packages
local packages = {}

---@xmake.context import
function packages.find_libxml2(...) end

---@xmake.context import
function packages.find_mbedtls(...) end

---@xmake.context import
function packages.find_mkl(...) end

---@xmake.context import
function packages.find_mysql(...) end

---@xmake.context import
function packages.find_nvtx(...) end

---@xmake.context import
function packages.find_openssl(...) end

---@xmake.context import
function packages.find_pcre(...) end

---@xmake.context import
function packages.find_pcre2(...) end

---@xmake.context import
function packages.find_tbb(...) end

---@xmake.context import
function packages.find_vulkansdk(...) end

---@xmake.context import
function packages.find_zlib(...) end

---@class detect.sdks
local sdks = {}

---@xmake.context import
function sdks.find_android_sdk(...) end

---@xmake.context import
function sdks.find_ascend(...) end

---@xmake.context import
function sdks.find_c51(...) end

---@xmake.context import
function sdks.find_cross_toolchain(...) end

---@xmake.context import
function sdks.find_cuda(...) end

---@xmake.context import
function sdks.find_dia_sdk(...) end

---@xmake.context import
function sdks.find_dotnet(...) end

---@xmake.context import
function sdks.find_emsdk(...) end

---@xmake.context import
function sdks.find_hdk(...) end

---@xmake.context import
function sdks.find_iarsdk(...) end

---@xmake.context import
function sdks.find_iccenv(...) end

---@xmake.context import
function sdks.find_icxenv(...) end

---@xmake.context import
function sdks.find_ifortenv(...) end

---@xmake.context import
function sdks.find_ifxenv(...) end

---@xmake.context import
function sdks.find_masm32(...) end

---@xmake.context import
function sdks.find_matlab(...) end

---@xmake.context import
function sdks.find_matlab_runtime(...) end

---@xmake.context import
function sdks.find_mdk(...) end

---@xmake.context import
function sdks.find_mingw(...) end

---@xmake.context import
function sdks.find_ndk(...) end

---@xmake.context import
function sdks.find_qt(...) end

---@xmake.context import
function sdks.find_sdasstm8(...) end

---@xmake.context import
function sdks.find_vcpkgdir(...) end

---@xmake.context import
function sdks.find_vstudio(...) end

---@xmake.context import
function sdks.find_vulkansdk(...) end

---@xmake.context import
function sdks.find_wasisdk(...) end

---@xmake.context import
function sdks.find_wdk(...) end

---@xmake.context import
function sdks.find_xcode(...) end

---@class detect.sdks.matlab
local matlab = {}

---@xmake.context import
function matlab.versions(...) end

---@xmake.context import
function matlab.versions_names(...) end

---@class devel.debugger
local debugger = {}

---@xmake.context import
function debugger.run(...) end

---@class devel.git.remote
local remote = {}

---@xmake.context import
function remote.get_url(...) end

---@xmake.context import
function remote.set_url(...) end

---@class devel.git.submodule
local submodule = {}

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#submodule-clean
---@xmake.context import
function submodule.clean(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#submodule-reset
---@xmake.context import
function submodule.reset(...) end

---@see https://xmake.io/api/scripts/extension-modules/devel/git.html#submodule-update
---@xmake.context import
function submodule.update(...) end

---@class devel.git.support
local support = {}

---@xmake.context import
function support.can_clone_tag(...) end

---@xmake.context import
function support.can_shallow_submodules(...) end

---@xmake.context import
function support.can_sparse_checkout(...) end

---@xmake.context import
function support.can_treeless(...) end

---@class lib.detect
local detect = {}

---@xmake.context import
function detect.check_alignof(...) end

---@xmake.context import
function detect.check_bigendian(...) end

---@xmake.context import
function detect.check_csnippets(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-check-cxsnippets
---@xmake.context import
function detect.check_cxsnippets(...) end

---@xmake.context import
function detect.check_cxxsnippets(...) end

---@xmake.context import
function detect.check_fcsnippets(...) end

---@xmake.context import
function detect.check_importfiles(...) end

---@xmake.context import
function detect.check_msnippets(...) end

---@xmake.context import
function detect.check_mxxsnippets(...) end

---@xmake.context import
function detect.check_sizeof(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-features
---@xmake.context import
function detect.features(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-find-cudadevices
---@xmake.context import
function detect.find_cudadevices(...) end

---@xmake.context import
function detect.find_directory(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-find-file
---@xmake.context import
function detect.find_file(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-find-library
---@xmake.context import
function detect.find_library(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-find-package
---@xmake.context import
function detect.find_package(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-find-path
---@xmake.context import
function detect.find_path(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-find-program
---@xmake.context import
function detect.find_program(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-find-programver
---@xmake.context import
function detect.find_programver(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-find-tool
---@xmake.context import
function detect.find_tool(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-find-toolname
---@xmake.context import
function detect.find_toolname(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-has-cfuncs
---@xmake.context import
function detect.has_cfuncs(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-has-cincludes
---@xmake.context import
function detect.has_cincludes(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-has-ctypes
---@xmake.context import
function detect.has_ctypes(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-has-cxxfuncs
---@xmake.context import
function detect.has_cxxfuncs(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-has-cxxincludes
---@xmake.context import
function detect.has_cxxincludes(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-has-cxxtypes
---@xmake.context import
function detect.has_cxxtypes(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-has-features
---@xmake.context import
function detect.has_features(...) end

---@see https://xmake.io/api/scripts/extension-modules/lib/detect.html#detect-has-flags
---@xmake.context import
function detect.has_flags(...) end

---@class lib.detect.pkgconfig
local pkgconfig = {}

---@xmake.context import
function pkgconfig.libinfo(...) end

---@xmake.context import
function pkgconfig.pcfile(...) end

---@xmake.context import
function pkgconfig.variables(...) end

---@xmake.context import
function pkgconfig.version(...) end

---@class lib.lua.package
local package2 = {}

---@see https://xmake.io/api/scripts/extension-modules/lib/lua/package.html#package-loadlib
---@xmake.context import
function package2.loadlib(...) end

---@class net
local net = {}

---@see https://xmake.io/api/scripts/extension-modules/net/ping.html
---@xmake.context import
function net.ping(...) end

---@class net.fasturl
local fasturl = {}

---@xmake.context import
function fasturl.add(...) end

---@xmake.context import
function fasturl.sort(...) end

---@class net.proxy
local proxy = {}

---@xmake.context import
function proxy.config(...) end

---@xmake.context import
function proxy.mirror(...) end

---@class os.winver
local winver = {}

---@xmake.context import
function winver.ntddi_version(...) end

---@xmake.context import
function winver.subsystem(...) end

---@xmake.context import
function winver.target_version(...) end

---@xmake.context import
function winver.version(...) end

---@xmake.context import
function winver.winnt_version(...) end

---@class package.manager
local manager = {}

---@xmake.context import
function manager.find_package(...) end

---@xmake.context import
function manager.install_package(...) end

---@class package.manager.apt
local apt = {}

---@xmake.context import
function apt.find_package(...) end

---@xmake.context import
function apt.install_package(...) end

---@xmake.context import
function apt.search_package(...) end

---@class package.manager.brew
local brew = {}

---@xmake.context import
function brew.find_package(...) end

---@xmake.context import
function brew.install_package(...) end

---@class package.manager.cargo
local cargo = {}

---@xmake.context import
function cargo.configurations(...) end

---@xmake.context import
function cargo.find_package(...) end

---@xmake.context import
function cargo.install_package(...) end

---@class package.manager.clib
local clib = {}

---@xmake.context import
function clib.configurations(...) end

---@xmake.context import
function clib.find_package(...) end

---@xmake.context import
function clib.install_package(...) end

---@class package.manager.cmake
local cmake = {}

---@xmake.context import
function cmake.configurations(...) end

---@xmake.context import
function cmake.find_package(...) end

---@class package.manager.conan
local conan = {}

---@xmake.context import
function conan.configurations(...) end

---@xmake.context import
function conan.find_package(...) end

---@xmake.context import
function conan.install_package(...) end

---@xmake.context import
function conan.search_package(...) end

---@class package.manager.conan.v1
local v1 = {}

---@xmake.context import
function v1.install_package(...) end

---@class package.manager.conan.v2
local v2 = {}

---@xmake.context import
function v2.install_package(...) end

---@class package.manager.conda
local conda = {}

---@xmake.context import
function conda.find_package(...) end

---@xmake.context import
function conda.install_package(...) end

---@xmake.context import
function conda.search_package(...) end

---@class package.manager.dub
local dub = {}

---@xmake.context import
function dub.find_package(...) end

---@xmake.context import
function dub.install_package(...) end

---@class package.manager.kotlin-native
local kotlin_native = {}

---@xmake.context import
function kotlin_native.configurations(...) end

---@xmake.context import
function kotlin_native.find_package(...) end

---@xmake.context import
function kotlin_native.install_package(...) end

---@class package.manager.nimble
local nimble = {}

---@xmake.context import
function nimble.find_package(...) end

---@xmake.context import
function nimble.install_package(...) end

---@class package.manager.nix
local nix = {}

---@xmake.context import
function nix.configurations(...) end

---@xmake.context import
function nix.find_package(...) end

---@xmake.context import
function nix.install_package(...) end

---@xmake.context import
function nix.search_package(...) end

---@class package.manager.nuget
local nuget = {}

---@xmake.context import
function nuget.find_package(...) end

---@xmake.context import
function nuget.install_package(...) end

---@class package.manager.pacman
local pacman = {}

---@xmake.context import
function pacman.configurations(...) end

---@xmake.context import
function pacman.find_package(...) end

---@xmake.context import
function pacman.get_package_name(...) end

---@xmake.context import
function pacman.install_package(...) end

---@class package.manager.pkg
local pkg = {}

---@xmake.context import
function pkg.install_package(...) end

---@class package.manager.pkgconfig
local pkgconfig2 = {}

---@xmake.context import
function pkgconfig2.find_package(...) end

---@class package.manager.portage
local portage = {}

---@xmake.context import
function portage.find_package(...) end

---@xmake.context import
function portage.install_package(...) end

---@class package.manager.system
local system = {}

---@xmake.context import
function system.find_package(...) end

---@class package.manager.vcpkg
local vcpkg = {}

---@xmake.context import
function vcpkg.configurations(...) end

---@xmake.context import
function vcpkg.find_package(...) end

---@xmake.context import
function vcpkg.install_package(...) end

---@xmake.context import
function vcpkg.search_package(...) end

---@class package.manager.vcpkg.utils
local utils = {}

---@xmake.context import
function utils.classic_curdir(...) end

---@xmake.context import
function utils.has_installed_features(...) end

---@xmake.context import
function utils.is_installed(...) end

---@xmake.context import
function utils.need_manifest(...) end

---@class package.manager.xmake
local xmake = {}

---@xmake.context import
function xmake.find_package(...) end

---@xmake.context import
function xmake.search_package(...) end

---@class package.manager.yum
local yum = {}

---@xmake.context import
function yum.install_package(...) end

---@class package.manager.zypper
local zypper = {}

---@xmake.context import
function zypper.find_package(...) end

---@xmake.context import
function zypper.install_package(...) end

---@xmake.context import
function zypper.search_package(...) end

---@class package.tools.autoconf
local autoconf = {}

---@xmake.context import
function autoconf.autogen_envs(...) end

---@xmake.context import
function autoconf.build(...) end

---@xmake.context import
function autoconf.buildenvs(...) end

---@xmake.context import
function autoconf.configure(...) end

---@xmake.context import
function autoconf.install(...) end

---@xmake.context import
function autoconf.make(...) end

---@class package.tools.bazel
local bazel = {}

---@xmake.context import
function bazel.build(...) end

---@xmake.context import
function bazel.buildenvs(...) end

---@class package.tools.cmake
local cmake2 = {}

---@xmake.context import
function cmake2.build(...) end

---@xmake.context import
function cmake2.buildenvs(...) end

---@xmake.context import
function cmake2.configure(...) end

---@xmake.context import
function cmake2.install(...) end

---@class package.tools.gn
local gn = {}

---@xmake.context import
function gn.build(...) end

---@xmake.context import
function gn.buildenvs(...) end

---@xmake.context import
function gn.generate(...) end

---@xmake.context import
function gn.install(...) end

---@class package.tools.jom
local jom = {}

---@xmake.context import
function jom.build(...) end

---@xmake.context import
function jom.buildenvs(...) end

---@xmake.context import
function jom.install(...) end

---@xmake.context import
function jom.make(...) end

---@class package.tools.make
local make = {}

---@xmake.context import
function make.build(...) end

---@xmake.context import
function make.buildenvs(...) end

---@xmake.context import
function make.install(...) end

---@xmake.context import
function make.make(...) end

---@class package.tools.meson
local meson = {}

---@xmake.context import
function meson.build(...) end

---@xmake.context import
function meson.buildenvs(...) end

---@xmake.context import
function meson.generate(...) end

---@xmake.context import
function meson.install(...) end

---@class package.tools.msbuild
local msbuild = {}

---@xmake.context import
function msbuild.build(...) end

---@xmake.context import
function msbuild.buildenvs(...) end

---@class package.tools.ninja
local ninja = {}

---@xmake.context import
function ninja.build(...) end

---@xmake.context import
function ninja.install(...) end

---@class package.tools.nmake
local nmake = {}

---@xmake.context import
function nmake.build(...) end

---@xmake.context import
function nmake.buildenvs(...) end

---@xmake.context import
function nmake.install(...) end

---@xmake.context import
function nmake.make(...) end

---@class package.tools.scons
local scons = {}

---@xmake.context import
function scons.build(...) end

---@xmake.context import
function scons.buildenvs(...) end

---@class privilege.sudo
local sudo = {}

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-exec
---@xmake.context import
function sudo.exec(...) end

---@xmake.context import
function sudo.execl(...) end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-execv
---@xmake.context import
function sudo.execv(...) end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-has
---@xmake.context import
function sudo.has(...) end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-iorun
---@xmake.context import
function sudo.iorun(...) end

---@xmake.context import
function sudo.iorunl(...) end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-iorunv
---@xmake.context import
function sudo.iorunv(...) end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-run
---@xmake.context import
function sudo.run(...) end

---@xmake.context import
function sudo.runl(...) end

---@see https://xmake.io/api/scripts/extension-modules/privilege/sudo.html#sudo-runv
---@xmake.context import
function sudo.runv(...) end

---@xmake.context import
function sudo.vrun(...) end

---@xmake.context import
function sudo.vrunl(...) end

---@xmake.context import
function sudo.vrunv(...) end

---@class target.action
local action = {}

---@xmake.context import
function action.clean(...) end

---@xmake.context import
function action.install(...) end

---@xmake.context import
function action.uninstall(...) end

---@class target.action.install
local install = {}

---@xmake.context import
function install.cmake_importfiles(...) end

---@xmake.context import
function install.pkgconfig_importfiles(...) end

---@class ui.app
local app = {}

---@xmake.context import
function app.showfile(...) end

---@xmake.context import
function app.version(...) end

---@class utils.binary
local binary = {}

---@xmake.context import
function binary.bin2c(...) end

---@xmake.context import
function binary.bin2obj(...) end

---@xmake.context import
function binary.deplibs(...) end

---@xmake.context import
function binary.extractlib(...) end

---@xmake.context import
function binary.readsyms(...) end

---@class utils.binary.rpath
local rpath = {}

---@xmake.context import
function rpath.change(...) end

---@xmake.context import
function rpath.clean(...) end

---@xmake.context import
function rpath.insert(...) end

---@xmake.context import
function rpath.list(...) end

---@xmake.context import
function rpath.remove(...) end

---@class utils.checker
local checker = {}

---@xmake.context import
function checker.is_running(...) end

---@class utils.ci
local ci = {}

---@xmake.context import
function ci.is_running(...) end

---@xmake.context import
function ci.packageskey(...) end

---@class utils.ipa
local ipa = {}

---@xmake.context import
function ipa.install(...) end

---@xmake.context import
function ipa.package(...) end

---@xmake.context import
function ipa.resign(...) end

---@class utils.platform
local platform2 = {}

---@xmake.context import
function platform2.gnu2mslib(...) end

---@class utils.progress
local progress = {}

---@xmake.context import
function progress.is_multirow(...) end

---@xmake.context import
function progress.refresh(...) end

---@xmake.context import
function progress.restore_style(...) end

---@xmake.context import
function progress.set_style(...) end

---@xmake.context import
function progress.set_target(...) end

---@xmake.context import
function progress.show(...) end

---@xmake.context import
function progress.show_abort(...) end

---@xmake.context import
function progress.show_output(...) end

---@xmake.context import
function progress.text(...) end

---@class utils.waiting_indicator
local waiting_indicator = {}

---@xmake.context import
function waiting_indicator.new(...) end

---@class utils.wdk
local wdk = {}

---@xmake.context import
function wdk.testcert(...) end
