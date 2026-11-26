---@meta
---@diagnostic disable: unused-local

--- The description APIs, called in `xmake.lua` and the files it includes.
---
--- Their parameters come from the kind of each API (values, paths, script,
--- ...); these stubs hold their documentation. `xmake.api` documents an API
--- in every scope, `xmake.<scope>` overrides it in one scope.
---@class xmake.api
local api = {}

--- Adds compilation flags for C files.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_cflags("-Wno-unused-parameter")
---     add_cflags("-fno-strict-aliasing", {force = true})
--- ```
---
--- # Notes
---
--- - xmake checks every flag against the compiler and drops the unsupported
---   ones; `{force = true}` skips the check.
--- - Prefer the portable APIs when one exists: `set_warnings`, `set_optimize`, ...
function api.add_cflags() end

--- Adds template files to configure: `${NAME}` and `${define NAME}` are replaced
--- by the variables of `set_configvar`, the options and builtin values.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_configdir("$(projectdir)/include")
---     add_configfiles("config.h.in")
---     set_configvar("HAS_SSL", 1)
--- ```
---
--- # Notes
---
--- `config.h.in` may contain `#define VERSION "${VERSION}"` and `${define HAS_SSL}`.
function api.add_configfiles() end

--- Declares a configuration of a package, set by `add_requires(..., {configs = {...}})`.
---
--- # Examples
---
--- ```lua
--- package("mylib")
---     add_configs("shared", {description = "Build a shared library.", default = false, type = "boolean"})
---     on_install(function (package)
---         local configs = {"-DBUILD_SHARED_LIBS=" .. (package:config("shared") and "ON" or "OFF")}
---         import("package.tools.cmake").install(package, configs)
---     end)
--- ```
function api.add_configs() end

--- Adds compilation flags for both C and C++ files.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_cxflags("-fPIC")
---     if is_plat("windows") then
---         add_cxflags("/utf-8")
---     end
--- ```
---
--- # Notes
---
--- Unsupported flags are dropped unless `{force = true}`.
function api.add_cxflags() end

--- Adds compilation flags for C++ files.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_cxxflags("-fno-rtti")
--- ```
---
--- # Notes
---
--- Unsupported flags are dropped unless `{force = true}`.
function api.add_cxxflags() end

--- Adds macro definitions.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_defines("DEBUG", "VERSION=\"1.0\"")
---     add_defines("MYLIB_STATIC", {public = true})
--- ```
---
--- # Notes
---
--- `{public = true}` also defines them for the targets depending on this one.
function api.add_defines() end

--- Adds targets of the project this target depends on: they are built first,
--- and libraries are linked.
---
--- # Examples
---
--- ```lua
--- target("core")
---     set_kind("static")
---     add_files("core/*.c")
---     add_includedirs("core", {public = true})
---
--- target("app")
---     set_kind("binary")
---     add_deps("core")
--- ```
---
--- # Notes
---
--- - Public settings of the dependencies (`{public = true}`) are inherited.
--- - `{inherit = false}` keeps the build order only.
--- - Targets of a namespace are named `ns::name`.
function api.add_deps() end

--- Adds source files. Globs are supported: `*` within a directory, `**`
--- recursively; `|` separates excluded patterns.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_files("src/*.c")
---     add_files("src/**.cpp|test/*.cpp|bench/*.cpp")
---     add_files("src/gen.c", {defines = "GENERATED"})
--- ```
---
--- # Notes
---
--- - The table of options configures these files only: `defines`,
---   `cflags`, `cxflags`, ... and `{force = {cxflags = "..."}}`.
--- - The tool is chosen by extension; `set_extensions` of a rule claims more.
function api.add_files() end

--- Adds header files: installed with the target and listed in generated IDE
--- projects. Parentheses keep the enclosed part of the path when installing.
---
--- # Examples
---
--- ```lua
--- target("mylib")
---     add_headerfiles("include/(mylib/*.h)")      -- installs include/mylib/*.h
---     add_headerfiles("src/*.h", {prefixdir = "mylib"})
--- ```
function api.add_headerfiles() end

--- Adds include directories.
---
--- # Examples
---
--- ```lua
--- target("mylib")
---     set_kind("static")
---     add_includedirs("include", {public = true})
---     add_includedirs("src")
--- ```
---
--- # Notes
---
--- `{public = true}` also adds them to the targets depending on this one; `{interface = true}` only to them.
function api.add_includedirs() end

--- Adds files to install with `xmake install`.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_installfiles("doc/*.md", {prefixdir = "share/doc/app"})
---     add_installfiles("assets/(**.png)", {prefixdir = "share/app"})
--- ```
---
--- # Notes
---
--- Parentheses keep the enclosed part of the path under the install directory.
function api.add_installfiles() end

--- Adds link flags for programs (and shared libraries with some toolchains).
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_kind("binary")
---     add_ldflags("-static", {force = true})
--- ```
---
--- # Notes
---
--- Shared libraries use `add_shflags`; static libraries `add_arflags`.
function api.add_ldflags() end

--- Adds library search directories.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_linkdirs("third_party/lib")
---     add_links("foo")
--- ```
function api.add_linkdirs() end

--- Adds libraries to link, by name without prefix and extension.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_linkdirs("lib")
---     add_links("z", "png")
--- ```
---
--- # Notes
---
--- - `add_deps` links the libraries of the project by itself.
--- - System libraries go to `add_syslinks`, linked after these.
function api.add_links() end

--- Associates options: when enabled, their settings apply to the target.
---
--- # Examples
---
--- ```lua
--- option("with_ssl")
---     set_default(false)
---     add_defines("WITH_SSL")
--- option_end()
---
--- target("app")
---     add_options("with_ssl")
--- ```
function api.add_options() end

--- Uses packages required with `add_requires`: their includes, links and defines.
---
--- # Examples
---
--- ```lua
--- add_requires("zlib", "fmt")
---
--- target("app")
---     add_packages("zlib", "fmt")
--- ```
---
--- # Notes
---
--- `{public = true}` passes the package on to the targets depending on this one.
function api.add_packages() end

--- Adds package repositories, searched before the official one.
---
--- # Examples
---
--- ```lua
--- add_repositories("my-repo https://github.com/me/xmake-repo.git")
--- add_repositories("local-repo packages") -- a directory of the project
--- ```
function api.add_repositories() end

--- Requires packages: xmake downloads, builds and installs them, then targets
--- use them with `add_packages`.
---
--- Each argument is a package name, optionally followed by a version range.
---
--- # Examples
---
--- ```lua
--- add_requires("zlib 1.2.x", "fmt")
--- add_requires("openssl", {optional = true})
--- add_requires("libpng", {configs = {shared = true}})
--- add_requires("conan::zlib/1.2.13", {alias = "conan_zlib"})
---
--- target("app")
---     add_packages("zlib", "fmt")
--- ```
---
--- # Notes
---
--- - `{system = false}` always builds the package; `{system = true}` only
---   uses the system one.
--- - A required package is not linked anywhere until a target adds it.
function api.add_requires() end

--- Applies rules: builtin ones (`mode.debug`, `c++.unity_build`, ...) or rules
--- defined with `rule(...)`.
---
--- # Examples
---
--- ```lua
--- -- At the root: for every target.
--- add_rules("mode.debug", "mode.release")
---
--- target("app")
---     add_rules("c++.unity_build", {batchsize = 4})
--- ```
---
--- # Notes
---
--- `mode.debug` and `mode.release` set the symbols and optimization of the matching `xmake f -m <mode>`.
function api.add_rules() end

--- Adds values to environment variables for `xmake run`.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_runenvs("PATH", "bin")
---     add_runenvs("LD_LIBRARY_PATH", "lib")
--- ```
---
--- # Notes
---
--- `set_runenv` replaces a variable instead.
function api.add_runenvs() end

--- Adds link flags for shared libraries.
---
--- # Examples
---
--- ```lua
--- target("plugin")
---     set_kind("shared")
---     add_shflags("-Wl,--no-undefined", {force = true})
--- ```
function api.add_shflags() end

--- Adds system libraries, linked after `add_links` and the packages.
---
--- # Examples
---
--- ```lua
--- target("app")
---     if is_plat("linux") then
---         add_syslinks("pthread", "dl", "m")
---     elseif is_plat("windows") then
---         add_syslinks("user32", "ws2_32")
---     end
--- ```
function api.add_syslinks() end

--- Undefines macros (`-U` flags).
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_undefines("NDEBUG")
--- ```
function api.add_undefines() end

--- Adds the URLs a package is downloaded from; `$(version)` is replaced.
---
--- # Examples
---
--- ```lua
--- package("mylib")
---     add_urls("https://github.com/me/mylib/archive/refs/tags/v$(version).tar.gz",
---              "https://github.com/me/mylib.git")
---     add_versions("1.0.0", "<sha256 of the archive>")
--- ```
function api.add_urls() end

--- Adds a version of a package with the SHA-256 of its archive (or a commit for git).
---
--- # Examples
---
--- ```lua
--- package("mylib")
---     add_versions("1.0.0", "<sha256>")
---     add_versions("1.1.0", "<sha256>")
--- ```
function api.add_versions() end

--- Runs a script after the target is built.
---
--- # Examples
---
--- ```lua
--- target("app")
---     after_build(function (target)
---         os.cp(target:targetfile(), "$(projectdir)/dist/")
---         print("built %s", target:name())
---     end)
--- ```
function api.after_build() end

--- Runs a script before the target is built.
---
--- # Examples
---
--- ```lua
--- target("app")
---     before_build(function (target)
---         os.mkdir(target:autogendir())
---     end)
--- ```
function api.before_build() end

--- Runs a script before the target is linked.
---
--- # Examples
---
--- ```lua
--- target("app")
---     before_link(function (target)
---         print("linking %s", target:targetfile())
---     end)
--- ```
function api.before_link() end

--- Returns the value of the configuration `name`, or `nil`.
---
--- # Examples
---
--- ```lua
--- local renderer = get_config("renderer") or "opengl"
--- add_defines("RENDERER=\"" .. renderer .. "\"")
--- ```
function api.get_config() end

--- Whether any of the given configurations is enabled (set and not `false`).
---
--- # Examples
---
--- ```lua
--- if has_config("with_ssl") then
---     add_packages("openssl")
--- end
--- ```
function api.has_config() end

--- Whether any of the given required packages is found and enabled.
---
--- # Examples
---
--- ```lua
--- add_requires("openssl", {optional = true})
---
--- target("app")
---     add_packages("openssl")
---     if has_package("openssl") then
---         add_defines("HAVE_OPENSSL")
---     end
--- ```
---
--- # Notes
---
--- Only meaningful in scripts and in `on_load`, once packages are resolved.
function api.has_package() end

--- Loads other description files: their targets, options and rules join the
--- project. Paths are relative to the current file.
---
--- # Examples
---
--- ```lua
--- includes("src/xmake.lua")
--- includes("lib")            -- lib/xmake.lua
--- includes("../common.lua")
--- includes("**/xmake.lua")   -- every xmake.lua below
--- includes("@builtin/check") -- builtin helpers: check_cfuncs, ...
--- ```
---
--- # Notes
---
--- - A directory loads the `xmake.lua` it contains.
--- - The included files are description files like `xmake.lua`.
function api.includes() end

--- Whether the target architecture is one of the arguments (`xmake f -a <arch>`).
---
--- # Examples
---
--- ```lua
--- if is_arch("x86_64", "x64") then
---     add_defines("ARCH_64")
--- elseif is_arch("arm.*") then
---     add_defines("ARCH_ARM")
--- end
--- ```
---
--- # Notes
---
--- Arguments are Lua patterns.
function api.is_arch() end

--- Whether the configuration `name` has one of the given values.
---
--- # Examples
---
--- ```lua
--- if is_config("renderer", "vulkan", "metal") then
---     add_files("src/gpu/*.cpp")
--- end
--- ```
---
--- # Notes
---
--- Values are Lua patterns.
function api.is_config() end

--- Whether xmake runs on one of the given systems, whatever the target platform.
---
--- # Examples
---
--- ```lua
--- if is_host("windows") then
---     set_toolchains("msvc")
--- end
--- ```
function api.is_host() end

--- Whether the kind of the current target is one of the arguments.
---
--- # Examples
---
--- ```lua
--- if is_kind("shared") then
---     add_defines("MYLIB_EXPORTS")
--- end
--- ```
function api.is_kind() end

--- Whether the build mode is one of the arguments (`xmake f -m <mode>`).
---
--- # Examples
---
--- ```lua
--- if is_mode("debug") then
---     add_defines("DEBUG")
---     set_symbols("debug")
--- end
--- ```
---
--- # Notes
---
--- Prefer `add_rules("mode.debug", "mode.release")` for the usual settings.
function api.is_mode() end

--- Whether the operating system of the target is one of the arguments.
---
--- # Examples
---
--- ```lua
--- if is_os("windows") then
---     add_files("src/win/*.c")
--- end
--- ```
---
--- # Notes
---
--- Unlike `is_plat`, `mingw` and `windows` platforms both run on `windows`.
function api.is_os() end

--- Whether the target platform is one of the arguments, as configured with
--- `xmake f -p <plat>`.
---
--- # Examples
---
--- ```lua
--- if is_plat("windows", "mingw") then
---     add_syslinks("ws2_32")
--- elseif is_plat("linux") then
---     add_syslinks("pthread")
--- end
--- ```
---
--- # Notes
---
--- Arguments are Lua patterns: `is_plat("iphone.*")`.
function api.is_plat() end

--- Opens a namespace: the targets, options and rules defined inside are named
--- `ns::name`, so that included projects do not clash.
---
--- # Examples
---
--- ```lua
--- namespace("net", function ()
---     target("core")
---         set_kind("static")
---         add_files("net/*.c")
--- end)
---
--- target("app")
---     add_deps("net::core")
--- ```
---
--- # Notes
---
--- Available since xmake 3.0. `namespace("ns")` ... `namespace_end()` works too.
function api.namespace() end

--- Replaces the build of the target by a script.
---
--- # Examples
---
--- ```lua
--- target("docs")
---     set_kind("phony")
---     on_build(function (target)
---         os.vrunv("mkdocs", {"build"})
---     end)
--- ```
---
--- # Notes
---
--- - The builtin build does not run anymore: compile and link yourself, or use
---   `before_build`/`after_build` to add steps around it.
function api.on_build() end

--- Builds one source file of the target, in a rule: replaces its compilation.
---
--- # Examples
---
--- ```lua
--- rule("lua2c")
---     set_extensions(".lua")
---     on_build_file(function (target, sourcefile, opt)
---         local outfile = path.join(target:autogendir(), path.basename(sourcefile) .. ".c")
---         os.vrunv("luac", {"-o", outfile, sourcefile})
---     end)
--- ```
---
--- # Notes
---
--- `opt.progress` holds the build progress to print with `cprint`.
function api.on_build_file() end

--- Checks whether an option or a toolchain is available: enable it from the script.
---
--- # Examples
---
--- ```lua
--- option("has_avx")
---     on_check(function (option)
---         import("lib.detect.check_cxsnippets")
---         if check_cxsnippets("__m256 x;", {includes = "immintrin.h"}) then
---             option:enable(true)
---         end
---     end)
--- ```
function api.on_check() end

--- Runs after every target is loaded and before the build: the configuration of the dependencies is known.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_deps("core")
---     on_config(function (target)
---         local core = target:dep("core")
---         print("core is a %s", core:kind())
---     end)
--- ```
function api.on_config() end

--- Replaces `xmake install` for the target, or installs a package.
---
--- # Examples
---
--- ```lua
--- target("app")
---     on_install(function (target)
---         os.cp(target:targetfile(), path.join(target:installdir(), "bin"))
---     end)
---
--- package("mylib")
---     on_install(function (package)
---         import("package.tools.cmake").install(package)
---     end)
--- ```
function api.on_install() end

--- Runs when the target is loaded, before the configuration is complete: the
--- place to configure it dynamically with `target:add`, `target:set`.
---
--- # Examples
---
--- ```lua
--- target("app")
---     on_load(function (target)
---         if os.isfile("extra.c") then
---             target:add("files", "extra.c")
---         end
---         target:add("defines", "BUILD_HOST=\"" .. os.host() .. "\"")
---     end)
--- ```
---
--- # Notes
---
--- Rules have their own `on_load`, run for each target applying them.
function api.on_load() end

--- Replaces how `xmake run` runs the target, or the body of a task.
---
--- # Examples
---
--- ```lua
--- target("app")
---     on_run(function (target)
---         os.execv(target:targetfile(), {"--verbose"})
---     end)
--- ```
function api.on_run() end

--- Defines an option: a configuration switch or value, set with
--- `xmake f --name=value` or detected automatically.
---
--- Targets use it with `add_options("name")`; scripts read it with
--- `has_config("name")` and `get_config("name")`.
---
--- # Examples
---
--- ```lua
--- option("with_ssl")
---     set_default(false)
---     set_showmenu(true)
---     set_description("Enable SSL support")
---     add_defines("WITH_SSL")
--- option_end()
---
--- target("app")
---     add_options("with_ssl")
--- ```
---
--- # Notes
---
--- - The settings of an option (`add_defines`, `add_links`, ...) apply to the
---   targets that add it, only when it is enabled.
--- - With `add_cfuncs`, `add_cincludes`, ... the option is enabled when the
---   check succeeds.
function api.option() end

--- Defines a package in the project: how to download, build and install a
--- dependency that is not in the official repository.
---
--- Require it with `add_requires("name")` like any other package.
---
--- # Examples
---
--- ```lua
--- package("mylib")
---     add_urls("https://example.com/mylib-$(version).tar.gz")
---     add_versions("1.0.0", "<sha256 of the archive>")
---     on_install(function (package)
---         import("package.tools.cmake").install(package)
---     end)
--- package_end()
---
--- add_requires("mylib 1.0.0")
--- ```
function api.package() end

--- Removes files added by an earlier `add_files`, e.g. from a glob.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_files("src/*.c")
---     remove_files("src/legacy.c")
--- ```
function api.remove_files() end

--- Defines a rule: build steps shared by targets, e.g. for a file type.
---
--- Targets apply it with `add_rules("name")`.
---
--- # Examples
---
--- ```lua
--- rule("markdown")
---     set_extensions(".md")
---     on_build_file(function (target, sourcefile, opt)
---         os.cp(sourcefile, target:targetdir())
---     end)
--- rule_end()
---
--- target("docs")
---     set_kind("phony")
---     add_rules("markdown")
---     add_files("docs/*.md")
--- ```
---
--- # Notes
---
--- The script callbacks of a rule receive the target the rule is applied to.
function api.rule() end

--- Sets the name of the output file, without prefix and extension.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_basename("hello") -- hello.exe, libhello.a, ...
--- ```
function api.set_basename() end

--- Sets the category of a task: `plugin`, `action` or `task` (default).
---
--- # Examples
---
--- ```lua
--- task("hello")
---     set_category("plugin")
--- ```
---
--- # Notes
---
--- Plugins and actions are listed by `xmake --help`.
function api.set_category() end

--- Sets the default value of a configuration, as `xmake f --name=value` would.
---
--- # Examples
---
--- ```lua
--- set_config("cc", "clang")
--- set_config("plat", "mingw")
--- ```
function api.set_config() end

--- Sets a variable for the files of `add_configfiles`.
---
--- # Examples
---
--- ```lua
--- target("app")
---     add_configfiles("config.h.in")
---     set_configvar("APP_NAME", "hello")
---     set_configvar("HAS_SSL", 1)
--- ```
function api.set_configvar() end

--- Sets whether `xmake` without a target name builds this target.
---
--- # Examples
---
--- ```lua
--- target("tests")
---     set_kind("binary")
---     set_default(false) -- only with `xmake build tests`
--- ```
function api.set_default() end

--- Sets the description shown by `xmake f --help` (options) or `xmake show` (targets).
---
--- # Examples
---
--- ```lua
--- option("with_ssl")
---     set_showmenu(true)
---     set_description("Enable SSL support")
--- ```
function api.set_description() end

--- Sets the encodings of the sources and of the execution character set.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_encodings("utf-8") -- e.g. /utf-8 with MSVC
--- ```
---
--- # Notes
---
--- `source.utf-8` and `target.utf-8` set only one of the two.
function api.set_encodings() end

--- Enables or disables exceptions, portably across compilers.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_exceptions("cxx")
---
--- target("kernel")
---     set_exceptions("no-cxx")
--- ```
function api.set_exceptions() end

--- Sets the file extensions a rule builds.
---
--- # Examples
---
--- ```lua
--- rule("markdown")
---     set_extensions(".md", ".markdown")
---     on_build_file(function (target, sourcefile, opt)
---         print("build %s", sourcefile)
---     end)
--- ```
function api.set_extensions() end

--- Sets the whole file name of the output.
---
--- # Examples
---
--- ```lua
--- target("plugin")
---     set_kind("shared")
---     set_filename("plugin.so")
--- ```
function api.set_filename() end

--- Sets the folder of the target in generated IDE projects; also selects targets with `xmake -g`.
---
--- # Examples
---
--- ```lua
--- target("test_math")
---     set_group("tests")
---
--- -- xmake build -g tests
--- ```
function api.set_group() end

--- Sets the homepage of a package, shown by `xrepo info`.
---
--- # Examples
---
--- ```lua
--- package("mylib")
---     set_homepage("https://github.com/me/mylib")
---     set_description("My library")
--- ```
function api.set_homepage() end

--- Sets the kind of the target.
---
--- - `binary`: an executable program;
--- - `static`: a static library;
--- - `shared`: a shared (dynamic) library;
--- - `object`: object files only, without linking;
--- - `headeronly`: headers only, nothing to build;
--- - `moduleonly`: C++20 modules only;
--- - `phony`: nothing to build, groups dependencies.
---
--- # Examples
---
--- ```lua
--- target("core")
---     set_kind("static")
---     add_files("src/core/*.c")
---
--- target("app")
---     set_kind("binary")
---     add_deps("core")
--- ```
---
--- # Notes
---
--- Without `set_kind`, the kind comes from `xmake f -k <kind>` (`static` by default).
function api.set_kind() end

--- Sets the language standards of the target.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_languages("c11", "cxx20")
--- ```
---
--- # Notes
---
--- - `cxxlatest` and `clatest` select the newest standard of the compiler.
--- - `gnu*` variants enable compiler extensions: `gnu99`, `gnuxx17`.
function api.set_languages() end

--- Sets the command line of a task: its usage, description and options.
---
--- # Examples
---
--- ```lua
--- task("hello")
---     set_menu {
---         usage = "xmake hello [options]",
---         description = "Print a greeting",
---         options = {
---             {"n", "name", "kv", "xmake", "The name to greet."}
---         }
---     }
---     on_run(function ()
---         import("core.base.option")
---         print("hello %s", option.get("name"))
---     end)
--- ```
function api.set_menu() end

--- Sets the optimization level, portably across compilers.
---
--- # Examples
---
--- ```lua
--- target("app")
---     if is_mode("release") then
---         set_optimize("fastest")
---     end
--- ```
---
--- # Notes
---
--- Values: `none`, `fast`, `faster`, `fastest`, `smallest`, `aggressive`.
function api.set_optimize() end

--- Sets the precompiled header of the C++ files.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_pcxxheader("src/pch.h")
---     add_files("src/*.cpp")
--- ```
---
--- # Notes
---
--- `set_pcheader` does the same for C files.
function api.set_pcxxheader() end

--- Changes a build policy, for the project (at the root) or a target.
---
--- # Examples
---
--- ```lua
--- set_policy("build.ccache", false)
---
--- target("app")
---     set_policy("check.auto_ignore_flags", false)
--- ```
---
--- # Notes
---
--- `xmake l core.project.policy.policies` lists the policies, or see the completion of this argument.
function api.set_policy() end

--- Sets the name of the project, used by generated IDE projects and packages.
---
--- # Examples
---
--- ```lua
--- set_project("hello")
--- set_version("1.0.0")
--- ```
function api.set_project() end

--- Sets the working directory of `xmake run`.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_rundir("$(projectdir)")
--- ```
function api.set_rundir() end

--- Sets the C/C++ runtime library.
---
--- # Examples
---
--- ```lua
--- target("app")
---     if is_plat("windows") then
---         set_runtimes(is_mode("debug") and "MDd" or "MD")
---     end
--- ```
---
--- # Notes
---
--- MSVC: `MT`, `MTd`, `MD`, `MDd`; LLVM and GNU: `c++_static`, `c++_shared`, `stdc++_static`, `stdc++_shared`.
function api.set_runtimes() end

--- Shows the option in `xmake f --help`, so users can set it.
---
--- # Examples
---
--- ```lua
--- option("with_ssl")
---     set_default(false)
---     set_showmenu(true)
---     set_description("Enable SSL support")
--- ```
function api.set_showmenu() end

--- Strips symbols from the output when linking.
---
--- # Examples
---
--- ```lua
--- target("app")
---     if is_mode("release") then
---         set_strip("all")
---     end
--- ```
---
--- # Notes
---
--- Values: `none`, `debug`, `all`.
function api.set_strip() end

--- Sets the debug information and the default visibility of symbols.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_symbols("debug")
---
--- target("mylib")
---     set_kind("shared")
---     set_symbols("hidden")
--- ```
---
--- # Notes
---
--- Values: `debug`, `hidden`, `none`, `edit` and `embed` (MSVC).
function api.set_symbols() end

--- Sets the directory of the output file.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_targetdir("bin")
--- ```
function api.set_targetdir() end

--- Sets the toolchains of the target, instead of the configured one.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_toolchains("clang")
---
--- target("gpu")
---     set_toolchains("cuda")
--- ```
function api.set_toolchains() end

--- Sets the program of a tool of a toolchain: `cc`, `cxx`, `ld`, `sh`, `ar`, `as`, ...
---
--- # Examples
---
--- ```lua
--- toolchain("mygcc")
---     set_kind("standalone")
---     set_toolset("cc", "gcc")
---     set_toolset("cxx", "g++")
---     set_toolset("ld", "g++")
---     set_toolset("ar", "ar")
--- ```
function api.set_toolset() end

--- Sets the version of the project (at the root) or of a target. It is
--- available as `$(version)` and in configuration files.
---
--- # Examples
---
--- ```lua
--- set_version("1.2.3", {build = "%Y%m%d%H%M"})
--- ```
---
--- # Notes
---
--- The `build` option is a `os.date` format, available as `$(buildversion)`.
function api.set_version() end

--- Sets the warning level, portably across compilers.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_warnings("all", "error")
--- ```
---
--- # Notes
---
--- Values: `none`, `less`, `more`, `all`, `allextra`, `extra`, `everything`, `pedantic`, `error`.
function api.set_warnings() end

--- Sets the minimal xmake version the project needs: older versions stop with an error.
---
--- # Examples
---
--- ```lua
--- set_xmakever("2.8.0")
--- ```
---
--- # Notes
---
--- xmake Analyzer also reports it when the installed xmake is older.
function api.set_xmakever() end

--- Defines a target: a program, a library or a phony group of dependencies.
---
--- The calls after it configure the target, up to `target_end()` or the next
--- scope function (`target`, `option`, `rule`, ...).
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_kind("binary")
---     add_files("src/*.c")
--- target_end()
--- ```
---
--- # Notes
---
--- - APIs called at the root, before any scope, apply to every target.
--- - `target("name", {kind = "binary", files = "src/*.c"})` defines a target in
---   one call, without opening a scope.
function api.target() end

--- Closes the current `target(...)` scope: later calls apply to the root again.
---
--- # Examples
---
--- ```lua
--- target("app")
---     set_kind("binary")
--- target_end()
---
--- -- Root again: applies to every target.
--- add_defines("NDEBUG")
--- ```
---
--- # Notes
---
--- Optional before another scope function, which closes the current scope anyway.
function api.target_end() end

--- Defines a task or a plugin, run with `xmake <name>`.
---
--- # Examples
---
--- ```lua
--- task("hello")
---     set_category("plugin")
---     on_run(function ()
---         print("hello xmake")
---     end)
---     set_menu {
---         usage = "xmake hello",
---         description = "Print a greeting",
---         options = {}
---     }
--- task_end()
--- ```
function api.task() end

--- Defines a toolchain: the compilers, linkers and flags used to build.
---
--- Targets select it with `set_toolchains("name")`.
---
--- # Examples
---
--- ```lua
--- toolchain("mygcc")
---     set_kind("standalone")
---     set_toolset("cc", "gcc")
---     set_toolset("cxx", "g++")
---     set_toolset("ld", "g++")
--- toolchain_end()
---
--- target("app")
---     set_toolchains("mygcc")
--- ```
function api.toolchain() end
