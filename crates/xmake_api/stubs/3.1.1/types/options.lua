---@meta

--- `{public = true}` of the values of a target: who else gets them.
---@class xmake.options.visibility
---@field public? any Also for the targets depending on this one.
---@field interface? any Only for the targets depending on this one.
---@field private? any Only for this target (the default).
local options_visibility = {}

--- The options of the compilation flags.
---@class xmake.options.flags
---@field force? any Do not check whether the compiler supports the flags.
---@field tools? any Only for these tools: `"cl"`, `{"gcc", "clang"}`.
---@field public? any Also for the targets depending on this one.
---@field interface? any Only for the targets depending on this one.
local options_flags = {}

--- The options of the source files.
---@class xmake.options.files
---@field defines? any Macro definitions for these files only.
---@field undefines? any Undefined macros for these files only.
---@field includedirs? any Include directories for these files only.
---@field cflags? any C flags for these files only.
---@field cxflags? any C and C++ flags for these files only.
---@field cxxflags? any C++ flags for these files only.
---@field mflags? any Objective-C flags for these files only.
---@field asflags? any Assembler flags for these files only.
---@field languages? any Language standards for these files only.
---@field force? any Flags not checked: `{force = {cxflags = "-O3"}}`.
---@field sourcekind? any Compile as another kind of source: `"cxx"`, `"cc"`, ...
---@field rule? any Build these files with a rule.
---@field always_added? any Add the files even if no rule claims their extension.
local options_files = {}

--- The options of the installed files.
---@class xmake.options.install
---@field prefixdir? any The directory under the install directory.
---@field rootdir? any Keep the paths relative to this directory.
---@field filename? any Install under another file name.
local options_install = {}

--- The options of the configuration files.
---@class xmake.options.configfile
---@field prefixdir? any The directory under the configuration directory.
---@field rootdir? any Keep the paths relative to this directory.
---@field filename? any Write under another file name.
---@field pattern? any The pattern of the variables, default `"${(.-)}"`; e.g. `"@(.-)@"`.
---@field variables? any Extra variables: `{NAME = value}`.
---@field onlycopy? any Copy the file without replacing variables.
local options_configfile = {}

--- The options of the tests.
---@class xmake.options.test
---@field runargs? any The arguments of the program: `{"--fast"}`.
---@field runenvs? any Environment variables of the program.
---@field rundir? any The working directory of the program.
---@field pass_outputs? any Pass when the output matches (string or Lua pattern list).
---@field fail_outputs? any Fail when the output matches.
---@field trim_output? any Trim the output before matching.
---@field plain? any Match the outputs as plain text, not patterns.
---@field timeout? any Fail after this many milliseconds.
---@field realtime_output? any Show the output while the test runs.
---@field build_should_pass? any Only build: the test passes when the build does.
---@field build_should_fail? any Only build: the test passes when the build fails.
---@field kind? any Build the test as another kind of target.
---@field files? any Extra source files of the test.
---@field remove_files? any Source files the test does not build.
---@field defines? any Extra macro definitions of the test.
---@field packages? any Extra packages of the test.
---@field languages? any Language standards of the test.
---@field group? any The group of the test, for `xmake test -g`.
local options_test = {}

--- The options of the dependencies.
---@class xmake.options.dep
---@field inherit? any `false`: only the build order, nothing inherited.
---@field public? any Also for the targets depending on this one.
---@field interface? any Only for the targets depending on this one.
local options_dep = {}

--- The options of the version.
---@class xmake.options.version
---@field build? any The format of the build version (`os.date`): `"%Y%m%d%H%M"`.
---@field soname? any Version the shared library file names (`true`, or a version).
local options_version = {}
