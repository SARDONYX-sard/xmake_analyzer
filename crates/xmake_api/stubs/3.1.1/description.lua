---@meta

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

--- The APIs of the `option` scope documented differently there.
---@class xmake.option
local option_scope = {}

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
---@xmake.prototype set_default(value: string|boolean|number)
---@xmake.param value Default value (string, boolean, or number)
---@see https://xmake.io/api/description/configuration-option.html#set-default
---@xmake.kind values
function option_scope.set_default() end

---@xmake.kind values
function option_scope.add_aicpuflags(...) end

---@xmake.prototype add_arflags(arflags: string|array, ..., {private})
---@xmake.param arflags Static library archive option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple static library archive option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-arflags
---@xmake.kind values
function option_scope.add_arflags(...) end

---@xmake.kind values
function option_scope.add_ascflags(...) end

---@xmake.kind values
function option_scope.add_ascnpuarchs(...) end

---@xmake.prototype add_asflags(asflags: string|array, ..., {private})
---@xmake.param asflags Assembly compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple assembly compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-asflags
---@xmake.kind values
function option_scope.add_asflags(...) end

---@xmake.prototype add_cflags(cflags: string|array, ..., {private})
---@xmake.param cflags C compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple C compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cflags
---@xmake.kind values
function option_scope.add_cflags(...) end

---@xmake.prototype add_cfuncs(funcs: string|array, ...)
---@xmake.param funcs C function name string or array
---@xmake.param ... Variable parameters, can pass multiple function names
---@see https://xmake.io/api/description/configuration-option.html#add-cfuncs
---@xmake.kind values
function option_scope.add_cfuncs(...) end

---@xmake.prototype add_cincludes(includes: string|array, ...)
---@xmake.param includes Header file name string or array
---@xmake.param ... Variable parameters, can pass multiple header file names
---@see https://xmake.io/api/description/configuration-option.html#add-cincludes
---@xmake.kind values
function option_scope.add_cincludes(...) end

---@xmake.kind values
function option_scope.add_csflags(...) end

---@xmake.prototype add_csnippets(name: string, code: string, {tryrun, output, number})
---@xmake.param name Code snippet name string
---@xmake.param code C code fragment string
---@xmake.param tryrun Whether to try running the code
---@xmake.param output Whether to capture output
---@xmake.param number Whether to parse output as number
---@see https://xmake.io/api/description/configuration-option.html#add-csnippets
---@xmake.kind dictionary
function option_scope.add_csnippets(...) end

---@xmake.prototype add_ctypes(types: string|array, ...)
---@xmake.param types C type name string or array
---@xmake.param ... Variable parameters, can pass multiple type names
---@see https://xmake.io/api/description/configuration-option.html#add-ctypes
---@xmake.kind values
function option_scope.add_ctypes(...) end

---@xmake.prototype add_cuflags(cuflags: string|array, ..., {private})
---@xmake.param cuflags CUDA compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple CUDA compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cuflags
---@xmake.kind values
function option_scope.add_cuflags(...) end

---@xmake.prototype add_cugencodes(cugencodes: string|array, ..., {private})
---@xmake.param cugencodes CUDA device gencode setting string or array, such as "sm_30", "sm_50"
---@xmake.param ... Variable parameters, can pass multiple CUDA device gencode setting strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cugencodes
---@xmake.kind values
function option_scope.add_cugencodes(...) end

---@xmake.prototype add_culdflags(culdflags: string|array, ..., {private})
---@xmake.param culdflags CUDA device link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple CUDA device link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-culdflags
---@xmake.kind values
function option_scope.add_culdflags(...) end

---@see https://xmake.io/api/description/project-target.html#add-cxflags
---@xmake.kind values
function option_scope.add_cxflags(...) end

---@xmake.prototype add_cxxflags(cxxflags: string|array, ..., {private})
---@xmake.param cxxflags C++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple C++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cxxflags
---@xmake.kind values
function option_scope.add_cxxflags(...) end

---@xmake.prototype add_cxxfuncs(funcs: string|array, ...)
---@xmake.param funcs C++ function name string or array
---@xmake.param ... Variable parameters, can pass multiple function names
---@see https://xmake.io/api/description/configuration-option.html#add-cxxfuncs
---@xmake.kind values
function option_scope.add_cxxfuncs(...) end

---@xmake.prototype add_cxxincludes(includes: string|array, ...)
---@xmake.param includes C++ header file name string or array
---@xmake.param ... Variable parameters, can pass multiple header file names
---@see https://xmake.io/api/description/configuration-option.html#add-cxxincludes
---@xmake.kind values
function option_scope.add_cxxincludes(...) end

---@xmake.prototype add_cxxsnippets(name: string, code: string, {tryrun, output, number})
---@xmake.param name Code snippet name string
---@xmake.param code C++ code fragment string
---@xmake.param tryrun Whether to try running the code
---@xmake.param output Whether to capture output
---@xmake.param number Whether to parse output as number
---@see https://xmake.io/api/description/configuration-option.html#add-cxxsnippets
---@xmake.kind dictionary
function option_scope.add_cxxsnippets(...) end

---@xmake.prototype add_cxxtypes(types: string|array, ...)
---@xmake.param types C++ type name string or array
---@xmake.param ... Variable parameters, can pass multiple type names
---@see https://xmake.io/api/description/configuration-option.html#add-cxxtypes
---@xmake.kind values
function option_scope.add_cxxtypes(...) end

---@xmake.prototype add_dcflags(dcflags: string|array, ..., {private})
---@xmake.param dcflags D language compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple D language compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-dcflags
---@xmake.kind values
function option_scope.add_dcflags(...) end

---@xmake.prototype add_defines(defines: string|array, ..., {private})
---@xmake.param defines Macro definition string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple macro definition strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-defines
---@xmake.kind values
function option_scope.add_defines(...) end

---@xmake.prototype add_deps(deps: string|array, ...)
---@xmake.param deps Dependency option name string or array
---@xmake.param ... Variable parameters, can pass multiple dependency names
---@see https://xmake.io/api/description/configuration-option.html#add-deps
---@xmake.kind values
function option_scope.add_deps(...) end

---@xmake.prototype add_fcflags(fcflags: string|array, ..., {private})
---@xmake.param fcflags Fortran compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Fortran compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-fcflags
---@xmake.kind values
function option_scope.add_fcflags(...) end

---@xmake.kind values
function option_scope.add_features(...) end

---@xmake.prototype add_frameworkdirs(frameworkdirs: string|array, ..., {private})
---@xmake.param frameworkdirs Framework search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple framework search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-frameworkdirs
---@xmake.kind paths
function option_scope.add_frameworkdirs(...) end

---@xmake.prototype add_frameworks(frameworks: string|array, ..., {private})
---@xmake.param frameworks Framework name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple framework name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-frameworks
---@xmake.kind values
function option_scope.add_frameworks(...) end

---@xmake.prototype add_gcflags(gcflags: string|array, ..., {private})
---@xmake.param gcflags Go compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Go compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-gcflags
---@xmake.kind values
function option_scope.add_gcflags(...) end

---@xmake.prototype add_imports(modules: string|array, ...)
---@xmake.param modules Module name string or array
---@xmake.param ... Variable parameters, can pass multiple module names
---@see https://xmake.io/api/description/project-target.html#add-imports
---@xmake.kind values
function option_scope.add_imports(...) end

---@xmake.prototype add_includedirs(includedirs: string|array, ..., {private})
---@xmake.param includedirs Header file search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple header file search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-includedirs
---@xmake.kind values
function option_scope.add_includedirs(...) end

---@xmake.kind values
function option_scope.add_kcflags(...) end

---@xmake.prototype add_ldflags(ldflags: string|array, ..., {private})
---@xmake.param ldflags Link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-ldflags
---@xmake.kind values
function option_scope.add_ldflags(...) end

---@xmake.prototype add_linkdirs(linkdirs: string|array, ...)
---@xmake.param linkdirs Link directory path string or array
---@xmake.param ... Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/configuration-option.html#add-linkdirs
---@xmake.kind paths
function option_scope.add_linkdirs(...) end

---@xmake.prototype add_links(links: string|array, ...)
---@xmake.param links Library name string or array
---@xmake.param ... Variable parameters, can pass multiple library names
---@see https://xmake.io/api/description/configuration-option.html#add-links
---@xmake.kind values
function option_scope.add_links(...) end

---@xmake.prototype add_mflags(mflags: string|array, ..., {private})
---@xmake.param mflags ObjC compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mflags
---@xmake.kind values
function option_scope.add_mflags(...) end

---@xmake.kind values
function option_scope.add_mrcflags(...) end

---@xmake.prototype add_mxflags(mxflags: string|array, ..., {private})
---@xmake.param mxflags ObjC/ObjC++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC/ObjC++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mxflags
---@xmake.kind values
function option_scope.add_mxflags(...) end

---@xmake.prototype add_mxxflags(mxxflags: string|array, ..., {private})
---@xmake.param mxxflags ObjC++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mxxflags
---@xmake.kind values
function option_scope.add_mxxflags(...) end

---@xmake.kind values
function option_scope.add_ncflags(...) end

---@xmake.kind values
function option_scope.add_pcflags(...) end

---@xmake.prototype add_rcflags(rcflags: string|array, ..., {private})
---@xmake.param rcflags Rust compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Rust compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-rcflags
---@xmake.kind values
function option_scope.add_rcflags(...) end

---@xmake.prototype add_rpathdirs(rpathdirs: string|array, ...)
---@xmake.param rpathdirs RPATH directory path string or array
---@xmake.param ... Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/configuration-option.html#add-rpathdirs
---@xmake.kind values
function option_scope.add_rpathdirs(...) end

---@xmake.prototype add_scflags(scflags: string|array, ..., {private})
---@xmake.param scflags Swift compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Swift compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-scflags
---@xmake.kind values
function option_scope.add_scflags(...) end

---@xmake.prototype add_shflags(shflags: string|array, ..., {private})
---@xmake.param shflags Dynamic library link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple dynamic library link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-shflags
---@xmake.kind values
function option_scope.add_shflags(...) end

---@xmake.prototype add_sysincludedirs(includedirs: string|array, ..., {private})
---@xmake.param includedirs System header file search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple system header file search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-sysincludedirs
---@xmake.kind paths
function option_scope.add_sysincludedirs(...) end

---@xmake.prototype add_syslinks(syslinks: string|array, ..., {private})
---@xmake.param syslinks System link library name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple system link library name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-syslinks
---@xmake.kind values
function option_scope.add_syslinks(...) end

---@xmake.prototype add_undefines(undefines: string|array, ..., {private})
---@xmake.param undefines Macro definition name string or array, such as "DEBUG"
---@xmake.param ... Variable parameters, can pass multiple macro definition name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-undefines
---@xmake.kind values
function option_scope.add_undefines(...) end

---@xmake.prototype add_vectorexts(vectorexts: string|array, ..., {private})
---@xmake.param vectorexts Vector extension instruction string or array, such as "mmx", "neon", "avx"
---@xmake.param ... Variable parameters, can pass multiple vector extension instruction strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-vectorexts
---@xmake.kind values
function option_scope.add_vectorexts(...) end

---@xmake.prototype add_zcflags(zcflags: string|array, ..., {private})
---@xmake.param zcflags Zig compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Zig compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-zcflags
---@xmake.kind values
function option_scope.add_zcflags(...) end

---@xmake.prototype after_check(script: function (option))
---@xmake.param script After check script function with option parameter
---@see https://xmake.io/api/description/configuration-option.html#after-check
---@xmake.kind script
function option_scope.after_check(...) end

---@xmake.prototype before_check(script: function (option))
---@xmake.param script Before check script function with option parameter
---@see https://xmake.io/api/description/configuration-option.html#before-check
---@xmake.kind script
function option_scope.before_check(...) end

---@xmake.prototype on_check(script: function (option))
---@xmake.param script Check script function with option parameter
---@see https://xmake.io/api/description/configuration-option.html#on-check
---@xmake.kind script
function option_scope.on_check(...) end

---@xmake.prototype set_category(category: string)
---@xmake.param category Category name string
---@see https://xmake.io/api/description/configuration-option.html#set-category
---@xmake.kind values
function option_scope.set_category(...) end

---@xmake.prototype set_configvar(name: string, value: any)
---@xmake.param name Configuration variable name string, such as "HAS_FOO"
---@xmake.param value Configuration variable value, can be any type
---@see https://xmake.io/api/description/project-target.html#set-configvar
---@xmake.kind keyvalues
function option_scope.set_configvar(...) end

---@xmake.prototype set_description(description: string|array, ...)
---@xmake.param description Description string or array
---@xmake.param ... Variable parameters, can pass multiple description lines
---@see https://xmake.io/api/description/configuration-option.html#set-description
---@xmake.kind values
function option_scope.set_description(...) end

---@xmake.prototype set_languages(languages: string|array, ..., {private})
---@xmake.param languages Language standard string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple language standard strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#set-languages
---@xmake.kind values
function option_scope.set_languages(...) end

---@xmake.prototype set_optimize(optimize: string)
---@xmake.param optimize Optimization level string, optional values: none, fast, faster, fastest, smallest, aggressive
---@see https://xmake.io/api/description/project-target.html#set-optimize
---@xmake.kind values
function option_scope.set_optimize(...) end

---@xmake.prototype set_showmenu(showmenu: boolean)
---@xmake.param showmenu Whether to show in menu (boolean)
---@see https://xmake.io/api/description/configuration-option.html#set-showmenu
---@xmake.kind values
function option_scope.set_showmenu(...) end

---@xmake.prototype set_values(values: string|array, ...)
---@xmake.param values Option value string or array
---@xmake.param ... Variable parameters, can pass multiple values
---@see https://xmake.io/api/description/configuration-option.html#set-values
---@xmake.kind values
function option_scope.set_values(...) end

---@xmake.prototype set_warnings(warnings: string, {private})
---@xmake.param warnings Warning level string, optional values: none, less, more, all, error
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#set-warnings
---@xmake.kind values
function option_scope.set_warnings(...) end

---@class xmake.package
local package_scope = {}

---@xmake.kind values
function package_scope.add_aicpuflags(...) end

---@xmake.prototype add_arflags(arflags: string|array, ..., {private})
---@xmake.param arflags Static library archive option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple static library archive option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-arflags
---@xmake.kind values
function package_scope.add_arflags(...) end

---@xmake.kind values
function package_scope.add_ascflags(...) end

---@xmake.prototype add_asflags(asflags: string|array, ..., {private})
---@xmake.param asflags Assembly compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple assembly compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-asflags
---@xmake.kind values
function package_scope.add_asflags(...) end

---@xmake.prototype add_bindirs(dirs: string|array, ...)
---@xmake.param dirs Executable directory path string or array
---@xmake.param ... Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/package-dependencies.html#add-bindirs
---@xmake.kind values
function package_scope.add_bindirs(...) end

---@xmake.prototype add_cflags(cflags: string|array, ..., {private})
---@xmake.param cflags C compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple C compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cflags
---@xmake.kind values
function package_scope.add_cflags(...) end

---@xmake.prototype add_components(components: string|array, ..., {deps})
---@xmake.param components Component name string or array
---@xmake.param ... Variable parameters, can pass multiple component names
---@xmake.param deps Component dependencies array
---@see https://xmake.io/api/description/package-dependencies.html#add-components
---@xmake.kind values
function package_scope.add_components(...) end

---@xmake.prototype add_configs(name: string, {description, default, values, type, readonly})
---@xmake.param name Configuration parameter name
---@xmake.param description Configuration description string
---@xmake.param default Default value for the configuration
---@xmake.param values Allowed values array
---@xmake.param type Configuration type: "string", "boolean", "number"
---@xmake.param readonly Prevent the modification of the configuration value
---@see https://xmake.io/api/description/package-dependencies.html#add-configs
---@xmake.kind values
function package_scope.add_configs(...) end

---@xmake.prototype add_cuflags(cuflags: string|array, ..., {private})
---@xmake.param cuflags CUDA compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple CUDA compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cuflags
---@xmake.kind values
function package_scope.add_cuflags(...) end

---@xmake.prototype add_culdflags(culdflags: string|array, ..., {private})
---@xmake.param culdflags CUDA device link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple CUDA device link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-culdflags
---@xmake.kind values
function package_scope.add_culdflags(...) end

---@see https://xmake.io/api/description/project-target.html#add-cxflags
---@xmake.kind values
function package_scope.add_cxflags(...) end

---@xmake.prototype add_cxxflags(cxxflags: string|array, ..., {private})
---@xmake.param cxxflags C++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple C++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cxxflags
---@xmake.kind values
function package_scope.add_cxxflags(...) end

---@xmake.prototype add_dcflags(dcflags: string|array, ..., {private})
---@xmake.param dcflags D language compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple D language compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-dcflags
---@xmake.kind values
function package_scope.add_dcflags(...) end

---@xmake.prototype add_defines(defines: string|array, ...)
---@xmake.param defines Macro definition string or array
---@xmake.param ... Variable parameters, can pass multiple definitions
---@see https://xmake.io/api/description/package-dependencies.html#add-defines
---@xmake.kind values
function package_scope.add_defines(...) end

---@xmake.prototype add_deps(deps: string|array, ...)
---@xmake.param deps Dependency package name string or array
---@xmake.param ... Variable parameters, can pass multiple dependency names
---@see https://xmake.io/api/description/package-dependencies.html#add-deps
---@xmake.kind values
function package_scope.add_deps(...) end

---@xmake.prototype add_extsources(sources: string|array, ...)
---@xmake.param sources External source string or array, format: "pkgconfig::name" or "brew::name"
---@xmake.param ... Variable parameters, can pass multiple external sources
---@see https://xmake.io/api/description/package-dependencies.html#add-extsources
---@xmake.kind values
function package_scope.add_extsources(...) end

---@xmake.prototype add_fcflags(fcflags: string|array, ..., {private})
---@xmake.param fcflags Fortran compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Fortran compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-fcflags
---@xmake.kind values
function package_scope.add_fcflags(...) end

---@xmake.prototype add_frameworkdirs(frameworkdirs: string|array, ..., {private})
---@xmake.param frameworkdirs Framework search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple framework search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-frameworkdirs
---@xmake.kind values
function package_scope.add_frameworkdirs(...) end

---@xmake.prototype add_frameworks(frameworks: string|array, ...)
---@xmake.param frameworks Framework name string or array
---@xmake.param ... Variable parameters, can pass multiple framework names
---@see https://xmake.io/api/description/package-dependencies.html#add-frameworks
---@xmake.kind values
function package_scope.add_frameworks(...) end

---@xmake.prototype add_imports(modules: string|array, ...)
---@xmake.param modules Module name string or array
---@xmake.param ... Variable parameters, can pass multiple module names
---@see https://xmake.io/api/description/project-target.html#add-imports
---@xmake.kind values
function package_scope.add_imports(...) end

---@xmake.prototype add_includedirs(dirs: string|array, ...)
---@xmake.param dirs Include directory path string or array
---@xmake.param ... Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/package-dependencies.html#add-includedirs
---@xmake.kind values
function package_scope.add_includedirs(...) end

---@xmake.prototype add_ldflags(ldflags: string|array, ..., {private})
---@xmake.param ldflags Link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-ldflags
---@xmake.kind values
function package_scope.add_ldflags(...) end

---@xmake.prototype add_linkdirs(dirs: string|array, ...)
---@xmake.param dirs Link directory path string or array
---@xmake.param ... Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/package-dependencies.html#add-linkdirs
---@xmake.kind values
function package_scope.add_linkdirs(...) end

---@xmake.prototype add_linkgroups(groups: string|array, ..., {name, group})
---@xmake.param groups Link group name string or array
---@xmake.param ... Variable parameters, can pass multiple group names
---@xmake.param name Group name for linking
---@xmake.param group Whether to treat as a group
---@see https://xmake.io/api/description/package-dependencies.html#add-linkgroups
---@xmake.kind groups
function package_scope.add_linkgroups(...) end

---@xmake.prototype add_linkorders(orders: string|array, ...)
---@xmake.param orders Link order string or array
---@xmake.param ... Variable parameters, can pass multiple order specifications
---@see https://xmake.io/api/description/package-dependencies.html#add-linkorders
---@xmake.kind groups
function package_scope.add_linkorders(...) end

---@xmake.prototype add_links(links: string|array, ...)
---@xmake.param links Library link name string or array
---@xmake.param ... Variable parameters, can pass multiple link names
---@see https://xmake.io/api/description/package-dependencies.html#add-links
---@xmake.kind values
function package_scope.add_links(...) end

---@xmake.prototype add_mflags(mflags: string|array, ..., {private})
---@xmake.param mflags ObjC compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mflags
---@xmake.kind values
function package_scope.add_mflags(...) end

---@xmake.prototype add_mxflags(mxflags: string|array, ..., {private})
---@xmake.param mxflags ObjC/ObjC++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC/ObjC++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mxflags
---@xmake.kind values
function package_scope.add_mxflags(...) end

---@xmake.prototype add_mxxflags(mxxflags: string|array, ..., {private})
---@xmake.param mxxflags ObjC++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mxxflags
---@xmake.kind values
function package_scope.add_mxxflags(...) end

---@xmake.prototype add_patches(version: string, url: string, hash: string)
---@xmake.param version Package version for which patch applies
---@xmake.param url Patch file URL
---@xmake.param hash SHA256 hash value for patch verification
---@see https://xmake.io/api/description/package-dependencies.html#add-patches
---@xmake.kind keyvalues
function package_scope.add_patches(...) end

---@xmake.kind values
function package_scope.add_pcflags(...) end

---@xmake.kind keyvalues
function package_scope.add_resources(...) end

---@xmake.prototype add_rpathdirs(rpathdirs: string|array, ..., {private})
---@xmake.param rpathdirs Runtime library search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple runtime library search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-rpathdirs
---@xmake.kind values
function package_scope.add_rpathdirs(...) end

---@xmake.prototype add_schemes(schemes: string|array, ...)
---@xmake.param schemes Scheme name string or array
---@xmake.param ... Variable parameters, can pass multiple scheme names
---@see https://xmake.io/api/description/package-dependencies.html#add-schemes
---@xmake.kind values
function package_scope.add_schemes(...) end

---@xmake.prototype add_shflags(shflags: string|array, ..., {private})
---@xmake.param shflags Dynamic library link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple dynamic library link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-shflags
---@xmake.kind values
function package_scope.add_shflags(...) end

---@xmake.prototype add_sysincludedirs(includedirs: string|array, ..., {private})
---@xmake.param includedirs System header file search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple system header file search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-sysincludedirs
---@xmake.kind values
function package_scope.add_sysincludedirs(...) end

---@xmake.prototype add_syslinks(syslinks: string|array, ...)
---@xmake.param syslinks System library name string or array
---@xmake.param ... Variable parameters, can pass multiple system library names
---@see https://xmake.io/api/description/package-dependencies.html#add-syslinks
---@xmake.kind values
function package_scope.add_syslinks(...) end

---@xmake.prototype add_undefines(undefines: string|array, ..., {private})
---@xmake.param undefines Macro definition name string or array, such as "DEBUG"
---@xmake.param ... Variable parameters, can pass multiple macro definition name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-undefines
---@xmake.kind values
function package_scope.add_undefines(...) end

---@xmake.prototype add_urls(urls: string|array, ..., {alias, excludes, version, http_headers})
---@xmake.param urls Package source URL string or array
---@xmake.param ... Variable parameters, can pass multiple URLs
---@xmake.param alias URL alias for different sources
---@xmake.param excludes Files to exclude from extraction
---@xmake.param version Version transformation function
---@xmake.param http_headers HTTP headers for download
---@see https://xmake.io/api/description/package-dependencies.html#add-urls
---@xmake.kind values
function package_scope.add_urls(...) end

---@xmake.prototype add_vectorexts(vectorexts: string|array, ..., {private})
---@xmake.param vectorexts Vector extension instruction string or array, such as "mmx", "neon", "avx"
---@xmake.param ... Variable parameters, can pass multiple vector extension instruction strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-vectorexts
---@xmake.kind values
function package_scope.add_vectorexts(...) end

---@xmake.prototype add_versionfiles(file: string)
---@xmake.param file Version file path containing version and hash pairs
---@see https://xmake.io/api/description/package-dependencies.html#add-versionfiles
---@xmake.kind paths
function package_scope.add_versionfiles(...) end

---@xmake.prototype add_versions(version: string, hash: string)
---@xmake.param version Package version string
---@xmake.param hash SHA256 hash value for verification
---@see https://xmake.io/api/description/package-dependencies.html#add-versions
---@xmake.kind dictionary
function package_scope.add_versions(...) end

---@xmake.prototype add_zcflags(zcflags: string|array, ..., {private})
---@xmake.param zcflags Zig compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Zig compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-zcflags
---@xmake.kind values
function package_scope.add_zcflags(...) end

---@xmake.prototype is_arch(arch: string, ...)
---@xmake.param arch Architecture name string
---@xmake.param ... Variable parameters, can pass multiple architecture names
---@see https://xmake.io/api/description/conditions.html#is-arch
---@xmake.kind values
function package_scope.is_arch(...) end

---@xmake.prototype is_host(host: string, ...)
---@xmake.param host Host system name string
---@xmake.param ... Variable parameters, can pass multiple host names
---@see https://xmake.io/api/description/conditions.html#is-host
---@xmake.kind values
function package_scope.is_host(...) end

---@xmake.prototype is_mode(mode: string, ...)
---@xmake.param mode Compilation mode name string
---@xmake.param ... Variable parameters, can pass multiple mode names
---@see https://xmake.io/api/description/conditions.html#is-mode
---@xmake.kind values
function package_scope.is_mode(...) end

---@xmake.prototype is_plat(plat: string, ...)
---@xmake.param plat Platform name string
---@xmake.param ... Variable parameters, can pass multiple platform names
---@see https://xmake.io/api/description/conditions.html#is-plat
---@xmake.kind values
function package_scope.is_plat(...) end

---@xmake.prototype on_check(platforms: string|array, ..., script: function (package))
---@xmake.param platforms Platform filter string or array, optional
---@xmake.param ... Variable parameters, can pass multiple platform filters
---@xmake.param script Check script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-check
---@xmake.kind script
function package_scope.on_check(...) end

---@xmake.prototype on_component(component: string, script: function (package, component))
---@xmake.param component Component name string, optional (if not provided, applies to all components)
---@xmake.param script Component configuration script function with package and component parameters
---@see https://xmake.io/api/description/package-dependencies.html#on-component
---@xmake.kind script
function package_scope.on_component(...) end

---@xmake.prototype on_download(script: function (package, opt))
---@xmake.param script Download script function with package and opt parameters
---@see https://xmake.io/api/description/package-dependencies.html#on-download
---@xmake.kind script
function package_scope.on_download(...) end

---@xmake.prototype on_fetch(platforms: string|array, ..., script: function (package, opt))
---@xmake.param platforms Platform filter string or array, optional
---@xmake.param ... Variable parameters, can pass multiple platform filters
---@xmake.param script Fetch script function with package and opt parameters
---@see https://xmake.io/api/description/package-dependencies.html#on-fetch
---@xmake.kind script
function package_scope.on_fetch(...) end

---@xmake.prototype on_install(platforms: string|array, ..., script: function (package))
---@xmake.param platforms Platform filter string or array, optional
---@xmake.param ... Variable parameters, can pass multiple platform filters
---@xmake.param script Install script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-install
---@xmake.kind script
function package_scope.on_install(...) end

---@xmake.prototype on_load(script: function (package))
---@xmake.param script Package load script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-load
---@xmake.kind script
function package_scope.on_load(...) end

---@xmake.kind script
function package_scope.on_source(...) end

---@xmake.prototype on_test(script: function (package))
---@xmake.param script Test script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-test
---@xmake.kind script
function package_scope.on_test(...) end

---@xmake.prototype set_arch(arch: string)
---@xmake.param arch Architecture name string, such as "x86", "x64", "arm64", "armv7"
---@see https://xmake.io/api/description/project-target.html#set-arch
---@xmake.kind values
function package_scope.set_arch(...) end

---@xmake.prototype set_base(package: string)
---@xmake.param package Base package name to inherit from
---@see https://xmake.io/api/description/package-dependencies.html#set-base
---@xmake.kind values
function package_scope.set_base(...) end

---@xmake.kind values
function package_scope.set_cachedir(...) end

---@xmake.prototype set_description(description: string)
---@xmake.param description Package description string
---@see https://xmake.io/api/description/package-dependencies.html#set-description
---@xmake.kind values
function package_scope.set_description(...) end

---@xmake.prototype set_homepage(url: string)
---@xmake.param url Package homepage URL string
---@see https://xmake.io/api/description/package-dependencies.html#set-homepage
---@xmake.kind values
function package_scope.set_homepage(...) end

---@xmake.prototype set_installdir(installdir: string)
---@xmake.param installdir Installation directory path string
---@see https://xmake.io/api/description/project-target.html#set-installdir
---@xmake.kind values
function package_scope.set_installdir(...) end

---@xmake.kind values
function package_scope.set_installtips(...) end

---@xmake.prototype set_kind(kind: string, {headeronly})
---@xmake.param kind Package type: "library", "binary", "toolchain"
---@xmake.param headeronly For library type, whether it's header-only library
---@see https://xmake.io/api/description/package-dependencies.html#set-kind
---@xmake.kind values
function package_scope.set_kind(...) end

---@xmake.prototype set_license(license: string)
---@xmake.param license License name string
---@see https://xmake.io/api/description/xpack-interfaces.html#set-license
---@xmake.kind values
function package_scope.set_license(...) end

---@xmake.kind values
function package_scope.set_parallelize(...) end

---@xmake.prototype set_plat(plat: string)
---@xmake.param plat Platform name string, such as "linux", "macosx", "windows", "android"
---@see https://xmake.io/api/description/project-target.html#set-plat
---@xmake.kind values
function package_scope.set_plat(...) end

---@xmake.prototype set_policy(policy: string, value: boolean)
---@xmake.param policy Policy name string, such as "check.auto_ignore_flags", "build.warning"
---@xmake.param value Policy value, true means enable, false means disable
---@see https://xmake.io/api/description/project-target.html#set-policy
---@xmake.kind keyvalues
function package_scope.set_policy(...) end

---@xmake.prototype set_sourcedir(sourcedir: string)
---@xmake.param sourcedir Package source directory path
---@see https://xmake.io/api/description/package-dependencies.html#set-sourcedir
---@xmake.kind values
function package_scope.set_sourcedir(...) end

---@xmake.prototype set_urls(urls: string|array, ..., {excludes, version, http_headers})
---@xmake.param urls Package source URL string or array
---@xmake.param ... Variable parameters, can pass multiple URLs
---@xmake.param excludes Files to exclude from extraction
---@xmake.param version Version transformation function
---@xmake.param http_headers HTTP headers for download
---@see https://xmake.io/api/description/package-dependencies.html#set-urls
---@xmake.kind values
function package_scope.set_urls(...) end

---@class xmake.root
local root_scope = {}

---@xmake.kind values
function root_scope.add_addons(...) end

---@xmake.prototype add_moduledirs(dirs: string|array, ...)
---@xmake.param dirs Module directory path string or array
---@xmake.param ... Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/global-interfaces.html#add-moduledirs
---@xmake.kind custom
function root_scope.add_moduledirs(...) end

---@xmake.kind paths
function root_scope.add_packagedirs(...) end

---@xmake.kind custom
function root_scope.add_platformdirs(...) end

---@xmake.prototype add_plugindirs(dirs: string|array, ...)
---@xmake.param dirs Plugin directory path string or array
---@xmake.param ... Variable parameters, can pass multiple directory paths
---@see https://xmake.io/api/description/global-interfaces.html#add-plugindirs
---@xmake.kind custom
function root_scope.add_plugindirs(...) end

---@xmake.prototype add_repositories(repos: string|array, ..., {rootdir})
---@xmake.param repos Repository name and URL string or array, format: "name url"
---@xmake.param ... Variable parameters, can pass multiple repositories
---@xmake.param rootdir Root directory for relative path resolution, optional
---@see https://xmake.io/api/description/global-interfaces.html#add-repositories
---@xmake.kind values
function root_scope.add_repositories(...) end

---@xmake.prototype add_requireconfs(packages: string|array, ..., {configs, override, version, debug})
---@xmake.param packages Package name string or array, supports pattern matching like "*" or "libpng.zlib"
---@xmake.param ... Variable parameters, can pass multiple package names
---@xmake.param configs Package configuration table
---@xmake.param override Override existing configuration, boolean value
---@xmake.param version Package version string
---@xmake.param debug Use debug version, boolean value
---@xmake.param ... Other package-specific configuration parameters
---@see https://xmake.io/api/description/global-interfaces.html#add-requireconfs
---@xmake.kind values
function root_scope.add_requireconfs(...) end

---@xmake.prototype add_requires(packages: string|array, ..., {optional, system, verify, debug, private, configs, alias})
---@xmake.param packages Package name string or array, supports semantic version like "tbox 1.6.*"
---@xmake.param ... Variable parameters, can pass multiple package names
---@xmake.param optional Set as optional package, boolean value
---@xmake.param system Disable system package detection, boolean value
---@xmake.param verify Disable package verification, boolean value
---@xmake.param debug Use debug version of package, boolean value
---@xmake.param private Use as private package, boolean value
---@xmake.param configs Package-specific configuration table
---@xmake.param alias Package alias name
---@xmake.param ... Other package-specific configuration parameters
---@see https://xmake.io/api/description/global-interfaces.html#add-requires
---@xmake.kind values
function root_scope.add_requires(...) end

---@xmake.kind custom
function root_scope.add_toolchaindirs(...) end

---@xmake.prototype get_config(name: string)
---@xmake.param name Configuration option name string
---@see https://xmake.io/api/description/global-interfaces.html#get-config
---@xmake.kind custom
function root_scope.get_config(...) end

---@xmake.prototype has_config(configs: string, ...)
---@xmake.param configs Configuration name string
---@xmake.param ... Variable parameters, can pass multiple configuration names
---@see https://xmake.io/api/description/conditions.html#has-config
---@xmake.kind custom
function root_scope.has_config(...) end

---@xmake.prototype has_package(packages: string, ...)
---@xmake.param packages Package name string
---@xmake.param ... Variable parameters, can pass multiple package names
---@see https://xmake.io/api/description/conditions.html#has-package
---@xmake.kind custom
function root_scope.has_package(...) end

---@xmake.prototype is_arch(arch: string, ...)
---@xmake.param arch Architecture name string
---@xmake.param ... Variable parameters, can pass multiple architecture names
---@see https://xmake.io/api/description/conditions.html#is-arch
---@xmake.kind custom
function root_scope.is_arch(...) end

---@xmake.prototype is_config(name: string, values: string, ...)
---@xmake.param name Configuration option name string
---@xmake.param values Configuration value string
---@xmake.param ... Variable parameters, can pass multiple values
---@see https://xmake.io/api/description/conditions.html#is-config
---@xmake.kind custom
function root_scope.is_config(...) end

---@xmake.prototype is_cross()
---@see https://xmake.io/api/description/conditions.html#is-cross
---@xmake.kind custom
function root_scope.is_cross(...) end

---@xmake.prototype is_host(host: string, ...)
---@xmake.param host Host system name string
---@xmake.param ... Variable parameters, can pass multiple host names
---@see https://xmake.io/api/description/conditions.html#is-host
---@xmake.kind custom
function root_scope.is_host(...) end

---@xmake.prototype is_kind(kind: string, ...)
---@xmake.param kind Target kind name string
---@xmake.param ... Variable parameters, can pass multiple kind names
---@see https://xmake.io/api/description/conditions.html#is-kind
---@xmake.kind custom
function root_scope.is_kind(...) end

---@xmake.prototype is_mode(mode: string, ...)
---@xmake.param mode Compilation mode name string
---@xmake.param ... Variable parameters, can pass multiple mode names
---@see https://xmake.io/api/description/conditions.html#is-mode
---@xmake.kind custom
function root_scope.is_mode(...) end

---@xmake.prototype is_os(os: string, ...)
---@xmake.param os Operating system name string
---@xmake.param ... Variable parameters, can pass multiple OS names
---@see https://xmake.io/api/description/conditions.html#is-os
---@xmake.kind custom
function root_scope.is_os(...) end

---@xmake.prototype is_plat(plat: string, ...)
---@xmake.param plat Platform name string
---@xmake.param ... Variable parameters, can pass multiple platform names
---@see https://xmake.io/api/description/conditions.html#is-plat
---@xmake.kind custom
function root_scope.is_plat(...) end

---@xmake.prototype set_allowedarchs(archs: string|array, ...)
---@xmake.param archs Allowed compilation architecture string or array, supports platform-specific format like "windows
---@xmake.param ... Variable parameters, can pass multiple architecture specifications
---@see https://xmake.io/api/description/global-interfaces.html#set-allowedarchs
---@xmake.kind values
function root_scope.set_allowedarchs(...) end

---@xmake.prototype set_allowedmodes(modes: string|array, ...)
---@xmake.param modes Allowed compilation mode name string or array
---@xmake.param ... Variable parameters, can pass multiple mode names
---@see https://xmake.io/api/description/global-interfaces.html#set-allowedmodes
---@xmake.kind values
function root_scope.set_allowedmodes(...) end

---@xmake.prototype set_allowedplats(platforms: string|array, ...)
---@xmake.param platforms Allowed compilation platform name string or array
---@xmake.param ... Variable parameters, can pass multiple platform names
---@see https://xmake.io/api/description/global-interfaces.html#set-allowedplats
---@xmake.kind values
function root_scope.set_allowedplats(...) end

---@xmake.prototype set_config(name: string, value: string|boolean|number)
---@xmake.param name Configuration option name string
---@xmake.param value Configuration value, can be string, boolean, or number
---@see https://xmake.io/api/description/global-interfaces.html#set-config
---@xmake.kind keyvalues
function root_scope.set_config(...) end

---@xmake.prototype set_defaultarchs(archs: string|array, ...)
---@xmake.param archs Default compilation architecture string or array, supports platform-specific format like "iphoneos
---@xmake.param ... Variable parameters, can pass multiple architecture specifications
---@see https://xmake.io/api/description/global-interfaces.html#set-defaultarchs
---@xmake.kind values
function root_scope.set_defaultarchs(...) end

---@xmake.prototype set_defaultmode(mode: string)
---@xmake.param mode Default compilation mode name, e.g. "release", "debug", "releasedbg"
---@see https://xmake.io/api/description/global-interfaces.html#set-defaultmode
---@xmake.kind values
function root_scope.set_defaultmode(...) end

---@xmake.prototype set_defaultplat(platform: string)
---@xmake.param platform Default compilation platform name, e.g. "iphoneos", "windows"
---@see https://xmake.io/api/description/global-interfaces.html#set-defaultplat
---@xmake.kind values
function root_scope.set_defaultplat(...) end

---@xmake.prototype set_description(description: string|array, ...)
---@xmake.param description Description string or array
---@xmake.param ... Variable parameters, can pass multiple description lines
---@see https://xmake.io/api/description/configuration-option.html#set-description
---@xmake.kind values
function root_scope.set_description(...) end

---@xmake.prototype set_project(name: string)
---@xmake.param name Project name string
---@see https://xmake.io/api/description/global-interfaces.html#set-project
---@xmake.kind values
function root_scope.set_project(...) end

---@class xmake.rule
local rule_scope = {}

---@xmake.prototype add_deps(deps: string|array, ..., {order})
---@xmake.param deps Dependency rule name string or array
---@xmake.param ... Variable parameters, can pass multiple dependency names
---@xmake.param order Whether to execute dependencies in order
---@see https://xmake.io/api/description/custom-rule.html#add-deps
---@xmake.kind values
function rule_scope.add_deps(...) end

---@xmake.prototype add_imports(modules: string|array, ...)
---@xmake.param modules Module name string or array
---@xmake.param ... Variable parameters, can pass multiple module names
---@see https://xmake.io/api/description/custom-rule.html#add-imports
---@xmake.kind values
function rule_scope.add_imports(...) end

---@xmake.kind groups
function rule_scope.add_orders(...) end

---@xmake.prototype after_build(script: function (target))
---@xmake.param script After build script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-build
---@xmake.kind script
function rule_scope.after_build(...) end

---@xmake.prototype after_build_file(script: function (target, sourcefile, opt))
---@xmake.param script After build file script function with target, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-build-file
---@xmake.kind script
function rule_scope.after_build_file(...) end

---@xmake.prototype after_build_files(script: function (target, sourcebatch, opt))
---@xmake.param script After build files script function with target, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-build-files
---@xmake.kind script
function rule_scope.after_build_files(...) end

---@xmake.prototype after_buildcmd(script: function (package, batchcmds))
---@xmake.param script After build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#after-buildcmd
---@xmake.kind script
function rule_scope.after_buildcmd(...) end

---@xmake.prototype after_buildcmd_file(script: function (target, batchcmds, sourcefile, opt))
---@xmake.param script After build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-buildcmd-file
---@xmake.kind script
function rule_scope.after_buildcmd_file(...) end

---@xmake.prototype after_buildcmd_files(script: function (target, batchcmds, sourcebatch, opt))
---@xmake.param script After build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-buildcmd-files
---@xmake.kind script
function rule_scope.after_buildcmd_files(...) end

---@xmake.prototype after_clean(script: function (target))
---@xmake.param script After clean script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-clean
---@xmake.kind script
function rule_scope.after_clean(...) end

---@xmake.prototype after_config(script: function (target))
---@xmake.param script After config script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-config
---@xmake.kind script
function rule_scope.after_config(...) end

---@xmake.prototype after_install(script: function (target))
---@xmake.param script After install script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-install
---@xmake.kind script
function rule_scope.after_install(...) end

---@xmake.prototype after_installcmd(script: function (component, batchcmds))
---@xmake.param script After install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#after-installcmd
---@xmake.kind script
function rule_scope.after_installcmd(...) end

---@xmake.prototype after_link(script: function (target))
---@xmake.param script After link script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-link
---@xmake.kind script
function rule_scope.after_link(...) end

---@xmake.kind script
function rule_scope.after_linkcmd(...) end

---@xmake.kind script
function rule_scope.after_load(...) end

---@xmake.prototype after_package(script: function (target))
---@xmake.param script After package script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-package
---@xmake.kind script
function rule_scope.after_package(...) end

---@xmake.prototype after_prepare(script: function (target))
---@xmake.param script After prepare script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-prepare
---@xmake.kind script
function rule_scope.after_prepare(...) end

---@xmake.prototype after_prepare_file(script: function (target, sourcefile, opt))
---@xmake.param script After prepare single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-prepare-file
---@xmake.kind script
function rule_scope.after_prepare_file(...) end

---@xmake.prototype after_prepare_files(script: function (target, sourcebatch, opt))
---@xmake.param script After prepare batch files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-prepare-files
---@xmake.kind script
function rule_scope.after_prepare_files(...) end

---@xmake.kind script
function rule_scope.after_preparecmd(...) end

---@xmake.kind script
function rule_scope.after_preparecmd_file(...) end

---@xmake.kind script
function rule_scope.after_preparecmd_files(...) end

---@xmake.prototype after_run(script: function (target))
---@xmake.param script After run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-run
---@xmake.kind script
function rule_scope.after_run(...) end

---@xmake.kind script
function rule_scope.after_test(...) end

---@xmake.prototype after_uninstall(script: function (target))
---@xmake.param script After uninstall script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-uninstall
---@xmake.kind script
function rule_scope.after_uninstall(...) end

---@xmake.prototype after_uninstallcmd(script: function (component, batchcmds))
---@xmake.param script After uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#after-uninstallcmd
---@xmake.kind script
function rule_scope.after_uninstallcmd(...) end

---@xmake.prototype before_build(script: function (target))
---@xmake.param script Before build script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-build
---@xmake.kind script
function rule_scope.before_build(...) end

---@xmake.prototype before_build_file(script: function (target, sourcefile, opt))
---@xmake.param script Before build file script function with target, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-build-file
---@xmake.kind script
function rule_scope.before_build_file(...) end

---@xmake.prototype before_build_files(script: function (target, sourcebatch, opt))
---@xmake.param script Before build files script function with target, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-build-files
---@xmake.kind script
function rule_scope.before_build_files(...) end

---@xmake.prototype before_buildcmd(script: function (package, batchcmds))
---@xmake.param script Before build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#before-buildcmd
---@xmake.kind script
function rule_scope.before_buildcmd(...) end

---@xmake.prototype before_buildcmd_file(script: function (target, batchcmds, sourcefile, opt))
---@xmake.param script Before build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-buildcmd-file
---@xmake.kind script
function rule_scope.before_buildcmd_file(...) end

---@xmake.prototype before_buildcmd_files(script: function (target, batchcmds, sourcebatch, opt))
---@xmake.param script Before build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-buildcmd-files
---@xmake.kind script
function rule_scope.before_buildcmd_files(...) end

---@xmake.prototype before_clean(script: function (target))
---@xmake.param script Before clean script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-clean
---@xmake.kind script
function rule_scope.before_clean(...) end

---@xmake.prototype before_config(script: function (target))
---@xmake.param script Before config script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-config
---@xmake.kind script
function rule_scope.before_config(...) end

---@xmake.prototype before_install(script: function (target))
---@xmake.param script Before install script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-install
---@xmake.kind script
function rule_scope.before_install(...) end

---@xmake.prototype before_installcmd(script: function (component, batchcmds))
---@xmake.param script Before install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#before-installcmd
---@xmake.kind script
function rule_scope.before_installcmd(...) end

---@xmake.prototype before_link(script: function (target))
---@xmake.param script Before link script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-link
---@xmake.kind script
function rule_scope.before_link(...) end

---@xmake.kind script
function rule_scope.before_linkcmd(...) end

---@xmake.kind script
function rule_scope.before_load(...) end

---@xmake.prototype before_package(script: function (target))
---@xmake.param script Before package script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-package
---@xmake.kind script
function rule_scope.before_package(...) end

---@xmake.prototype before_prepare(script: function (target))
---@xmake.param script Before prepare script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-prepare
---@xmake.kind script
function rule_scope.before_prepare(...) end

---@xmake.prototype before_prepare_file(script: function (target, sourcefile, opt))
---@xmake.param script Before prepare single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-prepare-file
---@xmake.kind script
function rule_scope.before_prepare_file(...) end

---@xmake.prototype before_prepare_files(script: function (target, sourcebatch, opt))
---@xmake.param script Before prepare batch files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-prepare-files
---@xmake.kind script
function rule_scope.before_prepare_files(...) end

---@xmake.kind script
function rule_scope.before_preparecmd(...) end

---@xmake.kind script
function rule_scope.before_preparecmd_file(...) end

---@xmake.kind script
function rule_scope.before_preparecmd_files(...) end

---@xmake.prototype before_run(script: function (target))
---@xmake.param script Before run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-run
---@xmake.kind script
function rule_scope.before_run(...) end

---@xmake.kind script
function rule_scope.before_test(...) end

---@xmake.prototype before_uninstall(script: function (target))
---@xmake.param script Before uninstall script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-uninstall
---@xmake.kind script
function rule_scope.before_uninstall(...) end

---@xmake.prototype before_uninstallcmd(script: function (component, batchcmds))
---@xmake.param script Before uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#before-uninstallcmd
---@xmake.kind script
function rule_scope.before_uninstallcmd(...) end

---@xmake.prototype on_build(script: function (target))
---@xmake.param script Build script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-build
---@xmake.kind script
function rule_scope.on_build(...) end

---@xmake.prototype on_build_file(script: function (target, sourcefile, opt))
---@xmake.param script Build file script function with target, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-build-file
---@xmake.kind script
function rule_scope.on_build_file(...) end

---@xmake.prototype on_build_files(script: function (target, sourcebatch, opt))
---@xmake.param script Build files script function with target, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-build-files
---@xmake.kind script
function rule_scope.on_build_files(...) end

---@xmake.prototype on_buildcmd(script: function (package, batchcmds))
---@xmake.param script Build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#on-buildcmd
---@xmake.kind script
function rule_scope.on_buildcmd(...) end

---@xmake.prototype on_buildcmd_file(script: function (target, batchcmds, sourcefile, opt))
---@xmake.param script Build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-buildcmd-file
---@xmake.kind script
function rule_scope.on_buildcmd_file(...) end

---@xmake.prototype on_buildcmd_files(script: function (target, batchcmds, sourcebatch, opt))
---@xmake.param script Build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-buildcmd-files
---@xmake.kind script
function rule_scope.on_buildcmd_files(...) end

---@xmake.prototype on_clean(script: function (target))
---@xmake.param script Clean script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-clean
---@xmake.kind script
function rule_scope.on_clean(...) end

---@xmake.prototype on_config(script: function (target))
---@xmake.param script Configuration script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-config
---@xmake.kind script
function rule_scope.on_config(...) end

---@xmake.prototype on_install(script: function (target))
---@xmake.param script Install script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-install
---@xmake.kind script
function rule_scope.on_install(...) end

---@xmake.prototype on_installcmd(script: function (component, batchcmds))
---@xmake.param script Install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#on-installcmd
---@xmake.kind script
function rule_scope.on_installcmd(...) end

---@xmake.prototype on_link(script: function (target))
---@xmake.param script Link script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-link
---@xmake.kind script
function rule_scope.on_link(...) end

---@xmake.kind script
function rule_scope.on_linkcmd(...) end

---@xmake.prototype on_load(script: function (target))
---@xmake.param script Load script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-load
---@xmake.kind script
function rule_scope.on_load(...) end

---@xmake.prototype on_package(script: function (target))
---@xmake.param script Package script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-package
---@xmake.kind script
function rule_scope.on_package(...) end

---@xmake.prototype on_prepare(script: function (target, opt))
---@xmake.param script Prepare phase script function, receives target, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-prepare
---@xmake.kind script
function rule_scope.on_prepare(...) end

---@xmake.prototype on_prepare_file(script: function (target, sourcefile, opt))
---@xmake.param script Single file processing script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-prepare-file
---@xmake.kind script
function rule_scope.on_prepare_file(...) end

---@xmake.prototype on_prepare_files(script: function (target, jobgraph, sourcebatch, opt), {jobgraph})
---@xmake.param script Batch file processing script function, receives target, jobgraph, sourcebatch, opt parameters
---@xmake.param jobgraph Whether to enable parallel task processing, optional values: true, false
---@see https://xmake.io/api/description/project-target.html#on-prepare-files
---@xmake.kind script
function rule_scope.on_prepare_files(...) end

---@xmake.kind script
function rule_scope.on_preparecmd(...) end

---@xmake.kind script
function rule_scope.on_preparecmd_file(...) end

---@xmake.kind script
function rule_scope.on_preparecmd_files(...) end

---@xmake.prototype on_run(script: function (target))
---@xmake.param script Run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-run
---@xmake.kind script
function rule_scope.on_run(...) end

---@xmake.prototype on_test(script: function (package))
---@xmake.param script Test script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-test
---@xmake.kind script
function rule_scope.on_test(...) end

---@xmake.prototype on_uninstall(script: function (target))
---@xmake.param script Uninstall script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#on-uninstall
---@xmake.kind script
function rule_scope.on_uninstall(...) end

---@xmake.prototype on_uninstallcmd(script: function (component, batchcmds))
---@xmake.param script Uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#on-uninstallcmd
---@xmake.kind script
function rule_scope.on_uninstallcmd(...) end

---@xmake.prototype set_extensions(extensions: string|array, ...)
---@xmake.param extensions File extension string or array
---@xmake.param ... Variable parameters, can pass multiple extensions
---@see https://xmake.io/api/description/custom-rule.html#set-extensions
---@xmake.kind values
function rule_scope.set_extensions(...) end

---@xmake.prototype set_kind(kind: string)
---@xmake.param kind Target type string, specifies the type of compilation target
---@see https://xmake.io/api/description/project-target.html#set-kind
---@xmake.kind values
function rule_scope.set_kind(...) end

---@xmake.kind values
function rule_scope.set_sourcekinds(...) end

---@class xmake.target
local target_scope = {}

---@xmake.kind values
function target_scope.add_aicpuflags(...) end

---@xmake.prototype add_arflags(arflags: string|array, ..., {private})
---@xmake.param arflags Static library archive option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple static library archive option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-arflags
---@xmake.kind values
function target_scope.add_arflags(...) end

---@xmake.kind values
function target_scope.add_ascflags(...) end

---@xmake.kind values
function target_scope.add_ascnpuarchs(...) end

---@xmake.prototype add_asflags(asflags: string|array, ..., {private})
---@xmake.param asflags Assembly compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple assembly compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-asflags
---@xmake.kind values
function target_scope.add_asflags(...) end

---@xmake.prototype add_cflags(cflags: string|array, ..., {private})
---@xmake.param cflags C compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple C compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cflags
---@xmake.kind values
function target_scope.add_cflags(...) end

---@xmake.kind paths
function target_scope.add_cleanfiles(...) end

---@xmake.prototype add_configfiles(configfiles: string|array, ..., {prefixdir, rootdir, filename})
---@xmake.param configfiles Configuration file path string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple configuration file path strings
---@xmake.param prefixdir Installation prefix directory, optional
---@xmake.param rootdir Root directory, optional
---@xmake.param filename Filename, optional
---@see https://xmake.io/api/description/project-target.html#add-configfiles
---@xmake.kind paths
function target_scope.add_configfiles(...) end

---@xmake.kind values
function target_scope.add_csflags(...) end

---@xmake.prototype add_cuflags(cuflags: string|array, ..., {private})
---@xmake.param cuflags CUDA compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple CUDA compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cuflags
---@xmake.kind values
function target_scope.add_cuflags(...) end

---@xmake.prototype add_cugencodes(cugencodes: string|array, ..., {private})
---@xmake.param cugencodes CUDA device gencode setting string or array, such as "sm_30", "sm_50"
---@xmake.param ... Variable parameters, can pass multiple CUDA device gencode setting strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cugencodes
---@xmake.kind values
function target_scope.add_cugencodes(...) end

---@xmake.prototype add_culdflags(culdflags: string|array, ..., {private})
---@xmake.param culdflags CUDA device link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple CUDA device link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-culdflags
---@xmake.kind values
function target_scope.add_culdflags(...) end

---@see https://xmake.io/api/description/project-target.html#add-cxflags
---@xmake.kind values
function target_scope.add_cxflags(...) end

---@xmake.prototype add_cxxflags(cxxflags: string|array, ..., {private})
---@xmake.param cxxflags C++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple C++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cxxflags
---@xmake.kind values
function target_scope.add_cxxflags(...) end

---@xmake.prototype add_dcflags(dcflags: string|array, ..., {private})
---@xmake.param dcflags D language compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple D language compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-dcflags
---@xmake.kind values
function target_scope.add_dcflags(...) end

---@xmake.prototype add_defines(defines: string|array, ..., {private})
---@xmake.param defines Macro definition string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple macro definition strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-defines
---@xmake.kind values
function target_scope.add_defines(...) end

---@xmake.prototype add_deps(deps: string|array, ..., {inherit})
---@xmake.param deps Dependency target name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple dependency target name strings
---@xmake.param inherit Whether to inherit dependency target configuration, optional values: true (inherit), false (not inherit)
---@see https://xmake.io/api/description/project-target.html#add-deps
---@xmake.kind values
function target_scope.add_deps(...) end

---@xmake.prototype add_embeddirs(embeddirs: string|array, ..., {private})
---@xmake.param embeddirs #embed search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple #embed search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-embeddirs
---@xmake.kind paths
function target_scope.add_embeddirs(...) end

---@xmake.prototype add_extrafiles(extrafiles: string|array, ...)
---@xmake.param extrafiles Extra file path string or array, such as "assets/other.txt"
---@xmake.param ... Variable parameters, can pass multiple extra file path strings
---@see https://xmake.io/api/description/project-target.html#add-extrafiles
---@xmake.kind paths
function target_scope.add_extrafiles(...) end

---@xmake.prototype add_fcflags(fcflags: string|array, ..., {private})
---@xmake.param fcflags Fortran compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Fortran compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-fcflags
---@xmake.kind values
function target_scope.add_fcflags(...) end

---@xmake.prototype add_filegroups(name: string, files: string|array, ...)
---@xmake.param name Group name string, such as "source", "header", "test"
---@xmake.param files Source file path string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple source file path strings
---@see https://xmake.io/api/description/project-target.html#add-filegroups
---@xmake.kind values
function target_scope.add_filegroups(...) end

---@xmake.prototype add_files(files: string|array, ..., {defines, languages, includedirs, rules, force, sourcekind, $flags})
---@xmake.param files File path string or file path array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple file path strings
---@xmake.param defines Macro definition string or array, set compilation macro definitions for specified files
---@xmake.param languages Language standard string, such as "c99", "c++11", etc.
---@xmake.param includedirs Header file search directory string or array
---@xmake.param rules Custom build rule name string
---@xmake.param force Force compilation option object, disable automatic detection, can contain various compilation options
---@xmake.param sourcekind Force specified source file type string, such as "cc", "cxx", etc.
---@xmake.param $flags Various compilation and linking options, including cflags, cxflags, cxxflags, mflags, mxflags, mxxflags, scflags, asflags, gcflags, dcflags, rcflags, fcflags, zcflags, cuflags, culdflags, cugencodes, ldflags, arflags, shflags, etc.
---@see https://xmake.io/api/description/project-target.html#add-files
---@xmake.kind paths
function target_scope.add_files(...) end

---@xmake.prototype add_forceincludes(includes: string|array, ..., {private})
---@xmake.param includes Header file path string or array, such as "config.h"
---@xmake.param ... Variable parameters, can pass multiple header file path strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-forceincludes
---@xmake.kind values
function target_scope.add_forceincludes(...) end

---@xmake.prototype add_frameworkdirs(frameworkdirs: string|array, ..., {private})
---@xmake.param frameworkdirs Framework search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple framework search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-frameworkdirs
---@xmake.kind paths
function target_scope.add_frameworkdirs(...) end

---@xmake.prototype add_frameworks(frameworks: string|array, ..., {private})
---@xmake.param frameworks Framework name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple framework name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-frameworks
---@xmake.kind values
function target_scope.add_frameworks(...) end

---@xmake.prototype add_gcflags(gcflags: string|array, ..., {private})
---@xmake.param gcflags Go compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Go compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-gcflags
---@xmake.kind values
function target_scope.add_gcflags(...) end

---@xmake.prototype add_headerfiles(headerfiles: string|array, ..., {prefixdir, rootdir, filename})
---@xmake.param headerfiles Header file path string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple header file path strings
---@xmake.param prefixdir Installation prefix directory, optional
---@xmake.param rootdir Root directory, optional
---@xmake.param filename Filename, optional
---@see https://xmake.io/api/description/project-target.html#add-headerfiles
---@xmake.kind paths
function target_scope.add_headerfiles(...) end

---@xmake.prototype add_imports(modules: string|array, ...)
---@xmake.param modules Module name string or array
---@xmake.param ... Variable parameters, can pass multiple module names
---@see https://xmake.io/api/description/project-target.html#add-imports
---@xmake.kind values
function target_scope.add_imports(...) end

---@xmake.prototype add_includedirs(includedirs: string|array, ..., {private})
---@xmake.param includedirs Header file search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple header file search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-includedirs
---@xmake.kind paths
function target_scope.add_includedirs(...) end

---@xmake.prototype add_installfiles(installfiles: string|array, ..., {prefixdir, rootdir, filename})
---@xmake.param installfiles Installation file path string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple installation file path strings
---@xmake.param prefixdir Installation prefix directory, optional
---@xmake.param rootdir Root directory, optional
---@xmake.param filename Filename, optional
---@see https://xmake.io/api/description/project-target.html#add-installfiles
---@xmake.kind paths
function target_scope.add_installfiles(...) end

---@xmake.kind values
function target_scope.add_kcflags(...) end

---@xmake.prototype add_languages(languages: string|array, ..., {private})
---@xmake.param languages Language standard string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple language standard strings
---@see https://xmake.io/api/description/project-target.html#add-languages
---@xmake.kind values
function target_scope.add_languages(...) end

---@xmake.prototype add_ldflags(ldflags: string|array, ..., {private})
---@xmake.param ldflags Link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-ldflags
---@xmake.kind values
function target_scope.add_ldflags(...) end

---@xmake.prototype add_linkdirs(linkdirs: string|array, ..., {private})
---@xmake.param linkdirs Link library search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple link library search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-linkdirs
---@xmake.kind paths
function target_scope.add_linkdirs(...) end

---@xmake.prototype add_linkgroups(linkgroups: string|array, ..., {private})
---@xmake.param linkgroups Link group name string or array, such as "group1", "group2"
---@xmake.param ... Variable parameters, can pass multiple link group name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-linkgroups
---@xmake.kind groups
function target_scope.add_linkgroups(...) end

---@xmake.prototype add_linkorders(linkorders: string|array, ...)
---@xmake.param linkorders Link order string or array, such as "dep1", "dep2"
---@xmake.param ... Variable parameters, can pass multiple link order strings
---@see https://xmake.io/api/description/project-target.html#add-linkorders
---@xmake.kind groups
function target_scope.add_linkorders(...) end

---@xmake.prototype add_links(links: string|array, ..., {private})
---@xmake.param links Link library name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple link library name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-links
---@xmake.kind values
function target_scope.add_links(...) end

---@xmake.prototype add_mflags(mflags: string|array, ..., {private})
---@xmake.param mflags ObjC compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mflags
---@xmake.kind values
function target_scope.add_mflags(...) end

---@xmake.kind values
function target_scope.add_mrcflags(...) end

---@xmake.prototype add_mxflags(mxflags: string|array, ..., {private})
---@xmake.param mxflags ObjC/ObjC++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC/ObjC++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mxflags
---@xmake.kind values
function target_scope.add_mxflags(...) end

---@xmake.prototype add_mxxflags(mxxflags: string|array, ..., {private})
---@xmake.param mxxflags ObjC++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mxxflags
---@xmake.kind values
function target_scope.add_mxxflags(...) end

---@xmake.kind values
function target_scope.add_ncflags(...) end

---@see https://xmake.io/api/description/project-target.html#add-options
---@xmake.kind values
function target_scope.add_options(...) end

---@xmake.prototype add_packages(packages: string|array, ..., {private})
---@xmake.param packages Package name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple package name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-packages
---@xmake.kind values
function target_scope.add_packages(...) end

---@xmake.kind values
function target_scope.add_pcflags(...) end

---@xmake.prototype add_rcflags(rcflags: string|array, ..., {private})
---@xmake.param rcflags Rust compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Rust compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-rcflags
---@xmake.kind values
function target_scope.add_rcflags(...) end

---@xmake.prototype add_rpathdirs(rpathdirs: string|array, ..., {private})
---@xmake.param rpathdirs Runtime library search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple runtime library search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-rpathdirs
---@xmake.kind values
function target_scope.add_rpathdirs(...) end

---@xmake.prototype add_rules(rules: string|array, ..., {private})
---@xmake.param rules Rule name string or array, such as "markdown"
---@xmake.param ... Variable parameters, can pass multiple rule name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-rules
---@xmake.kind values
function target_scope.add_rules(...) end

---@xmake.prototype add_runenvs(name: string, values: string|array, ...)
---@xmake.param name Environment variable name string, such as "PATH", "LD_LIBRARY_PATH"
---@xmake.param values Environment variable value string or array, supports multiple values
---@xmake.param ... Variable parameters, can pass multiple environment variable values
---@see https://xmake.io/api/description/project-target.html#add-runenvs
---@xmake.kind keyvalues
function target_scope.add_runenvs(...) end

---@xmake.prototype add_scflags(scflags: string|array, ..., {private})
---@xmake.param scflags Swift compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Swift compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-scflags
---@xmake.kind values
function target_scope.add_scflags(...) end

---@xmake.prototype add_shflags(shflags: string|array, ..., {private})
---@xmake.param shflags Dynamic library link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple dynamic library link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-shflags
---@xmake.kind values
function target_scope.add_shflags(...) end

---@xmake.prototype add_sysincludedirs(includedirs: string|array, ..., {private})
---@xmake.param includedirs System header file search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple system header file search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-sysincludedirs
---@xmake.kind paths
function target_scope.add_sysincludedirs(...) end

---@xmake.prototype add_syslinks(syslinks: string|array, ..., {private})
---@xmake.param syslinks System link library name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple system link library name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-syslinks
---@xmake.kind values
function target_scope.add_syslinks(...) end

---@xmake.prototype add_tests(tests: string|array, ..., {runargs, runenvs, timeout})
---@xmake.param tests Test case name string or array, such as "test1", "test2"
---@xmake.param ... Variable parameters, can pass multiple test case name strings
---@xmake.param runargs Test run argument string or array, optional
---@xmake.param runenvs Test run environment variable table, optional
---@xmake.param timeout Test timeout time in seconds, optional
---@xmake.param ... Other test configuration parameters, optional
---@see https://xmake.io/api/description/project-target.html#add-tests
---@xmake.kind values
function target_scope.add_tests(...) end

---@xmake.kind values
function target_scope.add_toolchains(...) end

---@xmake.prototype add_undefines(undefines: string|array, ..., {private})
---@xmake.param undefines Macro definition name string or array, such as "DEBUG"
---@xmake.param ... Variable parameters, can pass multiple macro definition name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-undefines
---@xmake.kind values
function target_scope.add_undefines(...) end

---@xmake.prototype add_values(name: string, values: any, ...)
---@xmake.param name Configuration name string, such as "markdown_flags"
---@xmake.param values Configuration value, can be any type
---@xmake.param ... Variable parameters, can pass multiple configuration values
---@see https://xmake.io/api/description/project-target.html#add-values
---@xmake.kind keyvalues
function target_scope.add_values(...) end

---@xmake.prototype add_vectorexts(vectorexts: string|array, ..., {private})
---@xmake.param vectorexts Vector extension instruction string or array, such as "mmx", "neon", "avx"
---@xmake.param ... Variable parameters, can pass multiple vector extension instruction strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-vectorexts
---@xmake.kind values
function target_scope.add_vectorexts(...) end

---@xmake.prototype add_zcflags(zcflags: string|array, ..., {private})
---@xmake.param zcflags Zig compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Zig compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-zcflags
---@xmake.kind values
function target_scope.add_zcflags(...) end

---@xmake.prototype after_build(script: function (target))
---@xmake.param script After build script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-build
---@xmake.kind script
function target_scope.after_build(...) end

---@xmake.prototype after_build_file(script: function (target, sourcefile, opt))
---@xmake.param script After build single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-build-file
---@xmake.kind script
function target_scope.after_build_file(...) end

---@xmake.prototype after_build_files(script: function (target, sourcebatch, opt))
---@xmake.param script After build files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-build-files
---@xmake.kind script
function target_scope.after_build_files(...) end

---@xmake.prototype after_buildcmd(script: function (package, batchcmds))
---@xmake.param script After build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#after-buildcmd
---@xmake.kind script
function target_scope.after_buildcmd(...) end

---@xmake.prototype after_buildcmd_file(script: function (target, batchcmds, sourcefile, opt))
---@xmake.param script After build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-buildcmd-file
---@xmake.kind script
function target_scope.after_buildcmd_file(...) end

---@xmake.prototype after_buildcmd_files(script: function (target, batchcmds, sourcebatch, opt))
---@xmake.param script After build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#after-buildcmd-files
---@xmake.kind script
function target_scope.after_buildcmd_files(...) end

---@xmake.prototype after_clean(script: function (target))
---@xmake.param script After clean script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-clean
---@xmake.kind script
function target_scope.after_clean(...) end

---@xmake.prototype after_config(script: function (target))
---@xmake.param script After config script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#after-config
---@xmake.kind script
function target_scope.after_config(...) end

---@xmake.prototype after_install(script: function (target))
---@xmake.param script After install script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-install
---@xmake.kind script
function target_scope.after_install(...) end

---@xmake.prototype after_installcmd(script: function (component, batchcmds))
---@xmake.param script After install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#after-installcmd
---@xmake.kind script
function target_scope.after_installcmd(...) end

---@xmake.prototype after_link(script: function (target))
---@xmake.param script After link script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-link
---@xmake.kind script
function target_scope.after_link(...) end

---@xmake.kind script
function target_scope.after_linkcmd(...) end

---@xmake.kind script
function target_scope.after_load(...) end

---@xmake.prototype after_package(script: function (target))
---@xmake.param script After package script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-package
---@xmake.kind script
function target_scope.after_package(...) end

---@xmake.prototype after_prepare(script: function (target))
---@xmake.param script After prepare script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-prepare
---@xmake.kind script
function target_scope.after_prepare(...) end

---@xmake.prototype after_prepare_file(script: function (target, sourcefile, opt))
---@xmake.param script After prepare single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-prepare-file
---@xmake.kind script
function target_scope.after_prepare_file(...) end

---@xmake.prototype after_prepare_files(script: function (target, sourcebatch, opt))
---@xmake.param script After prepare batch files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#after-prepare-files
---@xmake.kind script
function target_scope.after_prepare_files(...) end

---@xmake.kind script
function target_scope.after_preparecmd(...) end

---@xmake.kind script
function target_scope.after_preparecmd_file(...) end

---@xmake.kind script
function target_scope.after_preparecmd_files(...) end

---@xmake.prototype after_run(script: function (target))
---@xmake.param script After run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-run
---@xmake.kind script
function target_scope.after_run(...) end

---@xmake.kind script
function target_scope.after_test(...) end

---@xmake.prototype after_uninstall(script: function (target))
---@xmake.param script After uninstall script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#after-uninstall
---@xmake.kind script
function target_scope.after_uninstall(...) end

---@xmake.prototype after_uninstallcmd(script: function (component, batchcmds))
---@xmake.param script After uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#after-uninstallcmd
---@xmake.kind script
function target_scope.after_uninstallcmd(...) end

---@xmake.prototype before_build(script: function (target))
---@xmake.param script Before build script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-build
---@xmake.kind script
function target_scope.before_build(...) end

---@xmake.prototype before_build_file(script: function (target, sourcefile, opt))
---@xmake.param script Before build single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-build-file
---@xmake.kind script
function target_scope.before_build_file(...) end

---@xmake.prototype before_build_files(script: function (target, sourcebatch, opt))
---@xmake.param script Before build files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-build-files
---@xmake.kind script
function target_scope.before_build_files(...) end

---@xmake.prototype before_buildcmd(script: function (package, batchcmds))
---@xmake.param script Before build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#before-buildcmd
---@xmake.kind script
function target_scope.before_buildcmd(...) end

---@xmake.prototype before_buildcmd_file(script: function (target, batchcmds, sourcefile, opt))
---@xmake.param script Before build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-buildcmd-file
---@xmake.kind script
function target_scope.before_buildcmd_file(...) end

---@xmake.prototype before_buildcmd_files(script: function (target, batchcmds, sourcebatch, opt))
---@xmake.param script Before build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#before-buildcmd-files
---@xmake.kind script
function target_scope.before_buildcmd_files(...) end

---@xmake.prototype before_clean(script: function (target))
---@xmake.param script Before clean script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-clean
---@xmake.kind script
function target_scope.before_clean(...) end

---@xmake.prototype before_config(script: function (target))
---@xmake.param script Before config script function with target parameter
---@see https://xmake.io/api/description/custom-rule.html#before-config
---@xmake.kind script
function target_scope.before_config(...) end

---@xmake.prototype before_install(script: function (target))
---@xmake.param script Before install script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-install
---@xmake.kind script
function target_scope.before_install(...) end

---@xmake.prototype before_installcmd(script: function (component, batchcmds))
---@xmake.param script Before install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#before-installcmd
---@xmake.kind script
function target_scope.before_installcmd(...) end

---@xmake.prototype before_link(script: function (target))
---@xmake.param script Before link script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-link
---@xmake.kind script
function target_scope.before_link(...) end

---@xmake.kind script
function target_scope.before_linkcmd(...) end

---@xmake.prototype before_package(script: function (target))
---@xmake.param script Before package script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-package
---@xmake.kind script
function target_scope.before_package(...) end

---@xmake.prototype before_prepare(script: function (target))
---@xmake.param script Before prepare script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-prepare
---@xmake.kind script
function target_scope.before_prepare(...) end

---@xmake.prototype before_prepare_file(script: function (target, sourcefile, opt))
---@xmake.param script Before prepare single file script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-prepare-file
---@xmake.kind script
function target_scope.before_prepare_file(...) end

---@xmake.prototype before_prepare_files(script: function (target, sourcebatch, opt))
---@xmake.param script Before prepare batch files script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#before-prepare-files
---@xmake.kind script
function target_scope.before_prepare_files(...) end

---@xmake.kind script
function target_scope.before_preparecmd(...) end

---@xmake.kind script
function target_scope.before_preparecmd_file(...) end

---@xmake.kind script
function target_scope.before_preparecmd_files(...) end

---@xmake.prototype before_run(script: function (target))
---@xmake.param script Before run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-run
---@xmake.kind script
function target_scope.before_run(...) end

---@xmake.kind script
function target_scope.before_test(...) end

---@xmake.prototype before_uninstall(script: function (target))
---@xmake.param script Before uninstall script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#before-uninstall
---@xmake.kind script
function target_scope.before_uninstall(...) end

---@xmake.prototype before_uninstallcmd(script: function (component, batchcmds))
---@xmake.param script Before uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#before-uninstallcmd
---@xmake.kind script
function target_scope.before_uninstallcmd(...) end

---@xmake.kind paths
function target_scope.del_files(...) end

---@xmake.prototype on_build(script: function (target))
---@xmake.param script Build script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-build
---@xmake.kind script
function target_scope.on_build(...) end

---@xmake.prototype on_build_file(script: function (target, sourcefile, opt))
---@xmake.param script Single file build script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-build-file
---@xmake.kind script
function target_scope.on_build_file(...) end

---@xmake.prototype on_build_files(script: function (target, sourcebatch, opt))
---@xmake.param script Multi-file build script function, receives target, sourcebatch, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-build-files
---@xmake.kind script
function target_scope.on_build_files(...) end

---@xmake.prototype on_buildcmd(script: function (package, batchcmds))
---@xmake.param script Build script function with package and batchcmds parameters
---@see https://xmake.io/api/description/xpack-interfaces.html#on-buildcmd
---@xmake.kind script
function target_scope.on_buildcmd(...) end

---@xmake.prototype on_buildcmd_file(script: function (target, batchcmds, sourcefile, opt))
---@xmake.param script Build command file script function with target, batchcmds, sourcefile and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-buildcmd-file
---@xmake.kind script
function target_scope.on_buildcmd_file(...) end

---@xmake.prototype on_buildcmd_files(script: function (target, batchcmds, sourcebatch, opt))
---@xmake.param script Build command files script function with target, batchcmds, sourcebatch and opt parameters
---@see https://xmake.io/api/description/custom-rule.html#on-buildcmd-files
---@xmake.kind script
function target_scope.on_buildcmd_files(...) end

---@xmake.prototype on_clean(script: function (target))
---@xmake.param script Clean script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-clean
---@xmake.kind script
function target_scope.on_clean(...) end

---@xmake.prototype on_config(script: function (target))
---@xmake.param script Configuration script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-config
---@xmake.kind script
function target_scope.on_config(...) end

---@xmake.prototype on_install(script: function (target))
---@xmake.param script Install script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-install
---@xmake.kind script
function target_scope.on_install(...) end

---@xmake.prototype on_installcmd(script: function (component, batchcmds))
---@xmake.param script Install script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#on-installcmd
---@xmake.kind script
function target_scope.on_installcmd(...) end

---@xmake.prototype on_link(script: function (target))
---@xmake.param script Link script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-link
---@xmake.kind script
function target_scope.on_link(...) end

---@xmake.kind script
function target_scope.on_linkcmd(...) end

---@xmake.prototype on_load(script: function (target))
---@xmake.param script Load script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-load
---@xmake.kind script
function target_scope.on_load(...) end

---@xmake.prototype on_package(script: function (target))
---@xmake.param script Package script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-package
---@xmake.kind script
function target_scope.on_package(...) end

---@xmake.prototype on_prepare(script: function (target, opt))
---@xmake.param script Prepare phase script function, receives target, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-prepare
---@xmake.kind script
function target_scope.on_prepare(...) end

---@xmake.prototype on_prepare_file(script: function (target, sourcefile, opt))
---@xmake.param script Single file processing script function, receives target, sourcefile, opt parameters
---@see https://xmake.io/api/description/project-target.html#on-prepare-file
---@xmake.kind script
function target_scope.on_prepare_file(...) end

---@xmake.prototype on_prepare_files(script: function (target, jobgraph, sourcebatch, opt), {jobgraph})
---@xmake.param script Batch file processing script function, receives target, jobgraph, sourcebatch, opt parameters
---@xmake.param jobgraph Whether to enable parallel task processing, optional values: true, false
---@see https://xmake.io/api/description/project-target.html#on-prepare-files
---@xmake.kind script
function target_scope.on_prepare_files(...) end

---@xmake.kind script
function target_scope.on_preparecmd(...) end

---@xmake.kind script
function target_scope.on_preparecmd_file(...) end

---@xmake.kind script
function target_scope.on_preparecmd_files(...) end

---@xmake.prototype on_run(script: function (target))
---@xmake.param script Run script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-run
---@xmake.kind script
function target_scope.on_run(...) end

---@xmake.prototype on_test(script: function (package))
---@xmake.param script Test script function with package parameter
---@see https://xmake.io/api/description/package-dependencies.html#on-test
---@xmake.kind script
function target_scope.on_test(...) end

---@xmake.prototype on_uninstall(script: function (target))
---@xmake.param script Uninstall script function, receives target parameter
---@see https://xmake.io/api/description/project-target.html#on-uninstall
---@xmake.kind script
function target_scope.on_uninstall(...) end

---@xmake.prototype on_uninstallcmd(script: function (component, batchcmds))
---@xmake.param script Uninstall script function with component and batchcmds parameters
---@see https://xmake.io/api/description/xpack-component-interfaces.html#on-uninstallcmd
---@xmake.kind script
function target_scope.on_uninstallcmd(...) end

---@xmake.kind paths
function target_scope.remove_configfiles(...) end

---@xmake.kind paths
function target_scope.remove_extrafiles(...) end

---@xmake.prototype remove_files(files: string|array, ...)
---@xmake.param files File path string or file path array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple file path strings
---@see https://xmake.io/api/description/project-target.html#remove-files
---@xmake.kind paths
function target_scope.remove_files(...) end

---@xmake.prototype remove_headerfiles(headerfiles: string|array, ...)
---@xmake.param headerfiles Header file path string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple header file path strings
---@see https://xmake.io/api/description/project-target.html#remove-headerfiles
---@xmake.kind paths
function target_scope.remove_headerfiles(...) end

---@xmake.kind paths
function target_scope.remove_installfiles(...) end

---@xmake.prototype set_arch(arch: string)
---@xmake.param arch Architecture name string, such as "x86", "x64", "arm64", "armv7"
---@see https://xmake.io/api/description/project-target.html#set-arch
---@xmake.kind values
function target_scope.set_arch(...) end

---@xmake.kind paths
function target_scope.set_autogendir(...) end

---@xmake.prototype set_basename(basename: string)
---@xmake.param basename Target file base name string
---@see https://xmake.io/api/description/project-target.html#set-basename
---@xmake.kind values
function target_scope.set_basename(...) end

---@xmake.prototype set_configdir(configdir: string)
---@xmake.param configdir Template configuration file output directory path string
---@see https://xmake.io/api/description/project-target.html#set-configdir
---@xmake.kind paths
function target_scope.set_configdir(...) end

---@xmake.prototype set_configvar(name: string, value: any)
---@xmake.param name Configuration variable name string, such as "HAS_FOO"
---@xmake.param value Configuration variable value, can be any type
---@see https://xmake.io/api/description/project-target.html#set-configvar
---@xmake.kind keyvalues
function target_scope.set_configvar(...) end

---@xmake.prototype set_default(default: boolean)
---@xmake.param default Whether to mark as default build target, true means default build, false means not default build
---@see https://xmake.io/api/description/project-target.html#set-default
---@xmake.kind values
function target_scope.set_default(...) end

---@see https://xmake.io/api/description/project-target.html#set-dependir
---@xmake.kind paths
function target_scope.set_dependir(...) end

---@xmake.prototype set_enabled(enabled: boolean)
---@xmake.param enabled Whether to enable the target, true means enabled, false means disabled
---@see https://xmake.io/api/description/project-target.html#set-enabled
---@xmake.kind values
function target_scope.set_enabled(...) end

---@xmake.prototype set_encodings(encodings: string|array, ...)
---@xmake.param encodings Encoding string or array, such as "utf-8", "gb2312"
---@xmake.param ... Variable parameters, can pass multiple encoding strings
---@see https://xmake.io/api/description/project-target.html#set-encodings
---@xmake.kind values
function target_scope.set_encodings(...) end

---@xmake.prototype set_exceptions(exceptions: string|array, ...)
---@xmake.param exceptions Exception mode string or array, such as "cxx", "objc", "no-cxx"
---@xmake.param ... Variable parameters, can pass multiple exception mode strings
---@see https://xmake.io/api/description/project-target.html#set-exceptions
---@xmake.kind values
function target_scope.set_exceptions(...) end

---@xmake.prototype set_extension(extension: string)
---@xmake.param extension Target file extension string, such as ".dll" or ".so"
---@see https://xmake.io/api/description/project-target.html#set-extension
---@xmake.kind values
function target_scope.set_extension(...) end

---@xmake.prototype set_filename(filename: string)
---@xmake.param filename Target file full name string, including prefix and suffix
---@see https://xmake.io/api/description/project-target.html#set-filename
---@xmake.kind values
function target_scope.set_filename(...) end

---@xmake.prototype set_fpmodels(fpmodels: string|array, ..., {private})
---@xmake.param fpmodels Float-point mode string or array, optional values: fast, strict, except, precise
---@xmake.param ... Variable parameters, can pass multiple float-point mode strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#set-fpmodels
---@xmake.kind values
function target_scope.set_fpmodels(...) end

---@xmake.prototype set_group(group: string)
---@xmake.param group Group name string, such as "test", "libs", "tools"
---@see https://xmake.io/api/description/project-target.html#set-group
---@xmake.kind values
function target_scope.set_group(...) end

---@xmake.prototype set_installdir(installdir: string)
---@xmake.param installdir Installation directory path string
---@see https://xmake.io/api/description/project-target.html#set-installdir
---@xmake.kind paths
function target_scope.set_installdir(...) end

---@xmake.prototype set_kind(kind: string)
---@xmake.param kind Target type string, specifies the type of compilation target
---@see https://xmake.io/api/description/project-target.html#set-kind
---@xmake.kind values
function target_scope.set_kind(...) end

---@xmake.prototype set_languages(languages: string|array, ..., {private})
---@xmake.param languages Language standard string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple language standard strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#set-languages
---@xmake.kind values
function target_scope.set_languages(...) end

---@xmake.prototype set_license(license: string)
---@xmake.param license License name string
---@see https://xmake.io/api/description/xpack-interfaces.html#set-license
---@xmake.kind values
function target_scope.set_license(...) end

---@see https://xmake.io/api/description/project-target.html#set-objectdir
---@xmake.kind paths
function target_scope.set_objectdir(...) end

---@xmake.prototype set_optimize(optimize: string)
---@xmake.param optimize Optimization level string, optional values: none, fast, faster, fastest, smallest, aggressive
---@see https://xmake.io/api/description/project-target.html#set-optimize
---@xmake.kind values
function target_scope.set_optimize(...) end

---@xmake.prototype set_options(options: string|array, ...)
---@xmake.param options Option name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple option name strings
---@see https://xmake.io/api/description/project-target.html#set-options
---@xmake.kind values
function target_scope.set_options(...) end

---@xmake.prototype set_pcheader(header: string)
---@xmake.param header C precompiled header file path string
---@see https://xmake.io/api/description/project-target.html#set-pcheader
---@xmake.kind paths
function target_scope.set_pcheader(...) end

---@xmake.prototype set_pcxxheader(header: string)
---@xmake.param header C++ precompiled header file path string
---@see https://xmake.io/api/description/project-target.html#set-pcxxheader
---@xmake.kind paths
function target_scope.set_pcxxheader(...) end

---@xmake.prototype set_plat(plat: string)
---@xmake.param plat Platform name string, such as "linux", "macosx", "windows", "android"
---@see https://xmake.io/api/description/project-target.html#set-plat
---@xmake.kind values
function target_scope.set_plat(...) end

---@xmake.prototype set_pmheader(header: string)
---@xmake.param header ObjC precompiled header file path string
---@see https://xmake.io/api/description/project-target.html#set-pmheader
---@xmake.kind paths
function target_scope.set_pmheader(...) end

---@xmake.prototype set_pmxxheader(header: string)
---@xmake.param header ObjC++ precompiled header file path string
---@see https://xmake.io/api/description/project-target.html#set-pmxxheader
---@xmake.kind paths
function target_scope.set_pmxxheader(...) end

---@xmake.prototype set_policy(policy: string, value: boolean)
---@xmake.param policy Policy name string, such as "check.auto_ignore_flags", "build.warning"
---@xmake.param value Policy value, true means enable, false means disable
---@see https://xmake.io/api/description/project-target.html#set-policy
---@xmake.kind keyvalues
function target_scope.set_policy(...) end

---@xmake.prototype set_prefixdir(prefixdir: string)
---@xmake.param prefixdir Installation prefix subdirectory path string
---@see https://xmake.io/api/description/project-target.html#set-prefixdir
---@xmake.kind values
function target_scope.set_prefixdir(...) end

---@xmake.prototype set_prefixname(prefixname: string)
---@xmake.param prefixname Target file prefix name string, such as "lib" or ""
---@see https://xmake.io/api/description/project-target.html#set-prefixname
---@xmake.kind values
function target_scope.set_prefixname(...) end

---@xmake.kind values
function target_scope.set_rules(...) end

---@xmake.prototype set_runargs(runargs: string|array, ...)
---@xmake.param runargs Run argument string or array, such as "-x", "--arg1=val"
---@xmake.param ... Variable parameters, can pass multiple run argument strings
---@see https://xmake.io/api/description/project-target.html#set-runargs
---@xmake.kind values
function target_scope.set_runargs(...) end

---@xmake.prototype set_rundir(rundir: string)
---@xmake.param rundir Running directory path string
---@see https://xmake.io/api/description/project-target.html#set-rundir
---@xmake.kind paths
function target_scope.set_rundir(...) end

---@xmake.prototype set_runenv(name: string, value: string)
---@xmake.param name Environment variable name string, such as "PATH", "LD_LIBRARY_PATH"
---@xmake.param value Environment variable value string
---@see https://xmake.io/api/description/project-target.html#set-runenv
---@xmake.kind keyvalues
function target_scope.set_runenv(...) end

---@xmake.prototype set_runtimes(runtimes: string|array, ...)
---@xmake.param runtimes Runtime library string or array, such as "MT", "MD", "MTd", "MDd"
---@xmake.param ... Variable parameters, can pass multiple runtime library strings
---@see https://xmake.io/api/description/project-target.html#set-runtimes
---@xmake.kind values
function target_scope.set_runtimes(...) end

---@xmake.prototype set_strip(strip: string)
---@xmake.param strip Strip mode string, optional values: debug, all
---@see https://xmake.io/api/description/project-target.html#set-strip
---@xmake.kind values
function target_scope.set_strip(...) end

---@xmake.prototype set_suffixname(suffixname: string)
---@xmake.param suffixname Target file suffix name string, such as "-d" or ""
---@see https://xmake.io/api/description/project-target.html#set-suffixname
---@xmake.kind values
function target_scope.set_suffixname(...) end

---@xmake.prototype set_symbols(symbols: string, {private})
---@xmake.param symbols Symbol mode string, optional values: debug, hidden, none
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#set-symbols
---@xmake.kind values
function target_scope.set_symbols(...) end

---@see https://xmake.io/api/description/project-target.html#set-targetdir
---@xmake.kind paths
function target_scope.set_targetdir(...) end

---@xmake.prototype set_toolchains(toolchains: string|array, ..., {vs, plat, arch, clang, gcc, vs_sdkver, vs_toolset})
---@xmake.param toolchains Toolchain name string or array, such as "gcc", "clang", "msvc", "ndk[gcc]", "mingw[clang]@llvm-mingw", "msvc[vs=2025]"
---@xmake.param vs Visual Studio version, such as "2022", "2025"
---@xmake.param plat Platform name, such as "android", "ios"
---@xmake.param arch Architecture name, such as "arm64", "x64"
---@xmake.param clang Use Clang compiler in MinGW toolchain, boolean value
---@xmake.param gcc Use GCC compiler in Android NDK toolchain, boolean value
---@xmake.param vs_sdkver Visual Studio SDK version
---@xmake.param vs_toolset Visual Studio toolset version
---@xmake.param ... Variable parameters, can pass multiple toolchain name strings
---@see https://xmake.io/api/description/project-target.html#set-toolchains
---@xmake.kind values
function target_scope.set_toolchains(...) end

---@xmake.prototype set_toolset(toolname: string, tool: string)
---@xmake.param toolname Tool name string, such as "cc", "cxx", "ld", "ar"
---@xmake.param tool Tool path string, such as "/usr/bin/gcc"
---@see https://xmake.io/api/description/project-target.html#set-toolset
---@xmake.kind keyvalues
function target_scope.set_toolset(...) end

---@xmake.prototype set_values(name: string, values: any, ...)
---@xmake.param name Configuration name string, such as "markdown_flags"
---@xmake.param values Configuration value, can be any type
---@xmake.param ... Variable parameters, can pass multiple configuration values
---@see https://xmake.io/api/description/project-target.html#set-values
---@xmake.kind keyvalues
function target_scope.set_values(...) end

---@xmake.prototype set_version(version: string, {build, soname})
---@xmake.param version Project version string, e.g. "1.5.1"
---@xmake.param build Build version string, supports time format like "%Y%m%d%H%M"
---@xmake.param soname Soname version for dynamic library compatibility control, can be string or boolean
---@see https://xmake.io/api/description/global-interfaces.html#set-version
---@xmake.kind values
function target_scope.set_version(...) end

---@xmake.prototype set_warnings(warnings: string, {private})
---@xmake.param warnings Warning level string, optional values: none, less, more, all, error
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#set-warnings
---@xmake.kind values
function target_scope.set_warnings(...) end

---@class xmake.task
local task_scope = {}

---@xmake.prototype on_run(script: function)
---@xmake.param script Task execution function
---@see https://xmake.io/api/description/plugin-and-task.html#on-run
---@xmake.kind script
function task_scope.on_run(...) end

---@xmake.prototype set_category(category: string)
---@xmake.param category Category name string
---@see https://xmake.io/api/description/plugin-and-task.html#set-category
---@xmake.kind values
function task_scope.set_category(...) end

---@xmake.prototype set_menu(options: table)
---@xmake.param options Menu options table
---@see https://xmake.io/api/description/plugin-and-task.html#set-menu
---@xmake.kind dictionary
function task_scope.set_menu(...) end

---@class xmake.toolchain
local toolchain_scope = {}

---@xmake.kind values
function toolchain_scope.add_aicpuflags(...) end

---@xmake.prototype add_arflags(arflags: string|array, ..., {private})
---@xmake.param arflags Static library archive option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple static library archive option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-arflags
---@xmake.kind values
function toolchain_scope.add_arflags(...) end

---@xmake.kind values
function toolchain_scope.add_ascflags(...) end

---@xmake.kind values
function toolchain_scope.add_ascnpuarchs(...) end

---@xmake.prototype add_asflags(asflags: string|array, ..., {private})
---@xmake.param asflags Assembly compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple assembly compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-asflags
---@xmake.kind values
function toolchain_scope.add_asflags(...) end

---@xmake.prototype add_cflags(cflags: string|array, ..., {private})
---@xmake.param cflags C compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple C compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cflags
---@xmake.kind values
function toolchain_scope.add_cflags(...) end

---@xmake.kind values
function toolchain_scope.add_csflags(...) end

---@xmake.prototype add_cuflags(cuflags: string|array, ..., {private})
---@xmake.param cuflags CUDA compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple CUDA compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cuflags
---@xmake.kind values
function toolchain_scope.add_cuflags(...) end

---@xmake.prototype add_cugencodes(cugencodes: string|array, ..., {private})
---@xmake.param cugencodes CUDA device gencode setting string or array, such as "sm_30", "sm_50"
---@xmake.param ... Variable parameters, can pass multiple CUDA device gencode setting strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cugencodes
---@xmake.kind values
function toolchain_scope.add_cugencodes(...) end

---@xmake.prototype add_culdflags(culdflags: string|array, ..., {private})
---@xmake.param culdflags CUDA device link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple CUDA device link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-culdflags
---@xmake.kind values
function toolchain_scope.add_culdflags(...) end

---@see https://xmake.io/api/description/project-target.html#add-cxflags
---@xmake.kind values
function toolchain_scope.add_cxflags(...) end

---@xmake.prototype add_cxxflags(cxxflags: string|array, ..., {private})
---@xmake.param cxxflags C++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple C++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-cxxflags
---@xmake.kind values
function toolchain_scope.add_cxxflags(...) end

---@xmake.prototype add_dcflags(dcflags: string|array, ..., {private})
---@xmake.param dcflags D language compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple D language compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-dcflags
---@xmake.kind values
function toolchain_scope.add_dcflags(...) end

---@xmake.prototype add_defines(defines: string|array, ..., {private})
---@xmake.param defines Macro definition string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple macro definition strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-defines
---@xmake.kind values
function toolchain_scope.add_defines(...) end

---@xmake.prototype add_fcflags(fcflags: string|array, ..., {private})
---@xmake.param fcflags Fortran compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Fortran compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-fcflags
---@xmake.kind values
function toolchain_scope.add_fcflags(...) end

---@xmake.prototype add_frameworkdirs(frameworkdirs: string|array, ..., {private})
---@xmake.param frameworkdirs Framework search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple framework search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-frameworkdirs
---@xmake.kind values
function toolchain_scope.add_frameworkdirs(...) end

---@xmake.prototype add_frameworks(frameworks: string|array, ..., {private})
---@xmake.param frameworks Framework name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple framework name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-frameworks
---@xmake.kind values
function toolchain_scope.add_frameworks(...) end

---@xmake.prototype add_gcflags(gcflags: string|array, ..., {private})
---@xmake.param gcflags Go compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Go compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-gcflags
---@xmake.kind values
function toolchain_scope.add_gcflags(...) end

---@xmake.prototype add_includedirs(includedirs: string|array, ..., {private})
---@xmake.param includedirs Header file search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple header file search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-includedirs
---@xmake.kind values
function toolchain_scope.add_includedirs(...) end

---@xmake.kind values
function toolchain_scope.add_kcflags(...) end

---@xmake.prototype add_ldflags(ldflags: string|array, ..., {private})
---@xmake.param ldflags Link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-ldflags
---@xmake.kind values
function toolchain_scope.add_ldflags(...) end

---@xmake.prototype add_linkdirs(linkdirs: string|array, ..., {private})
---@xmake.param linkdirs Link library search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple link library search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-linkdirs
---@xmake.kind values
function toolchain_scope.add_linkdirs(...) end

---@xmake.prototype add_links(links: string|array, ..., {private})
---@xmake.param links Link library name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple link library name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-links
---@xmake.kind values
function toolchain_scope.add_links(...) end

---@xmake.prototype add_mflags(mflags: string|array, ..., {private})
---@xmake.param mflags ObjC compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mflags
---@xmake.kind values
function toolchain_scope.add_mflags(...) end

---@xmake.kind values
function toolchain_scope.add_mrcflags(...) end

---@xmake.prototype add_mxflags(mxflags: string|array, ..., {private})
---@xmake.param mxflags ObjC/ObjC++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC/ObjC++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mxflags
---@xmake.kind values
function toolchain_scope.add_mxflags(...) end

---@xmake.prototype add_mxxflags(mxxflags: string|array, ..., {private})
---@xmake.param mxxflags ObjC++ compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple ObjC++ compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-mxxflags
---@xmake.kind values
function toolchain_scope.add_mxxflags(...) end

---@xmake.kind values
function toolchain_scope.add_ncflags(...) end

---@xmake.kind values
function toolchain_scope.add_pcflags(...) end

---@xmake.prototype add_rcflags(rcflags: string|array, ..., {private})
---@xmake.param rcflags Rust compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Rust compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-rcflags
---@xmake.kind values
function toolchain_scope.add_rcflags(...) end

---@xmake.prototype add_rpathdirs(rpathdirs: string|array, ..., {private})
---@xmake.param rpathdirs Runtime library search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple runtime library search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-rpathdirs
---@xmake.kind values
function toolchain_scope.add_rpathdirs(...) end

---@xmake.prototype add_runenvs(name: string, values: string|array, ...)
---@xmake.param name Environment variable name string, such as "PATH", "LD_LIBRARY_PATH"
---@xmake.param values Environment variable value string or array, supports multiple values
---@xmake.param ... Variable parameters, can pass multiple environment variable values
---@see https://xmake.io/api/description/project-target.html#add-runenvs
---@xmake.kind keyvalues
function toolchain_scope.add_runenvs(...) end

---@xmake.prototype add_scflags(scflags: string|array, ..., {private})
---@xmake.param scflags Swift compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Swift compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-scflags
---@xmake.kind values
function toolchain_scope.add_scflags(...) end

---@xmake.prototype add_shflags(shflags: string|array, ..., {private})
---@xmake.param shflags Dynamic library link option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple dynamic library link option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-shflags
---@xmake.kind values
function toolchain_scope.add_shflags(...) end

---@xmake.prototype add_sysincludedirs(includedirs: string|array, ..., {private})
---@xmake.param includedirs System header file search directory string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple system header file search directory strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-sysincludedirs
---@xmake.kind values
function toolchain_scope.add_sysincludedirs(...) end

---@xmake.prototype add_syslinks(syslinks: string|array, ..., {private})
---@xmake.param syslinks System link library name string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple system link library name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-syslinks
---@xmake.kind values
function toolchain_scope.add_syslinks(...) end

---@xmake.kind keyvalues
function toolchain_scope.add_toolset(...) end

---@xmake.prototype add_undefines(undefines: string|array, ..., {private})
---@xmake.param undefines Macro definition name string or array, such as "DEBUG"
---@xmake.param ... Variable parameters, can pass multiple macro definition name strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-undefines
---@xmake.kind values
function toolchain_scope.add_undefines(...) end

---@xmake.prototype add_zcflags(zcflags: string|array, ..., {private})
---@xmake.param zcflags Zig compilation option string or array, supports wildcard matching patterns
---@xmake.param ... Variable parameters, can pass multiple Zig compilation option strings
---@xmake.param public\ interface\
---@see https://xmake.io/api/description/project-target.html#add-zcflags
---@xmake.kind values
function toolchain_scope.add_zcflags(...) end

---@xmake.prototype on_check(script: function (toolchain))
---@xmake.param script Check script function with toolchain parameter
---@see https://xmake.io/api/description/custom-toolchain.html#on-check
---@xmake.kind script
function toolchain_scope.on_check(...) end

---@xmake.prototype on_load(script: function (toolchain) | string)
---@xmake.param script Load script function with toolchain parameter, or a string referring to a sibling `.lua` file (v3.0.9 and above)
---@see https://xmake.io/api/description/custom-toolchain.html#on-load
---@xmake.kind script
function toolchain_scope.on_load(...) end

---@xmake.kind values
function toolchain_scope.set_archs(...) end

---@xmake.prototype set_bindir(bindir: string)
---@xmake.param bindir Binary directory path string
---@see https://xmake.io/api/description/custom-toolchain.html#set-bindir
---@xmake.kind values
function toolchain_scope.set_bindir(...) end

---@xmake.kind values
function toolchain_scope.set_cross(...) end

---@xmake.prototype set_description(description: string|array, ...)
---@xmake.param description Description string or array
---@xmake.param ... Variable parameters, can pass multiple description lines
---@see https://xmake.io/api/description/configuration-option.html#set-description
---@xmake.kind values
function toolchain_scope.set_description(...) end

---@xmake.prototype set_formats(formats: string|array, ...)
---@xmake.param formats Package format name string or array
---@xmake.param ... Variable parameters, can pass multiple format names
---@xmake.param Format Description
---@xmake.param nsis Windows NSIS installation package, binary installation
---@xmake.param inno Windows Inno Setup installer, supported since v3.1.2
---@xmake.param zip Binary zip package, does not contain installation script
---@xmake.param targz Binary tar.gz package, does not contain the installation script
---@xmake.param srczip zip source package
---@xmake.param srctargz tar.gz source package
---@xmake.param runself self-running shell script package, source code compilation and installation
---@xmake.param rpm rpm binary installation package
---@xmake.param srpm rpm source code installation package
---@xmake.param deb deb binary installation package (TODO)
---@xmake.param Others Customizable formats and installation scripts
---@see https://xmake.io/api/description/xpack-interfaces.html#set-formats
---@xmake.kind keyvalues
function toolchain_scope.set_formats(...) end

---@xmake.prototype set_homepage(url: string)
---@xmake.param url Package homepage URL string
---@see https://xmake.io/api/description/package-dependencies.html#set-homepage
---@xmake.kind values
function toolchain_scope.set_homepage(...) end

---@xmake.prototype set_kind(kind: string)
---@xmake.param kind Toolchain type: "standalone"
---@see https://xmake.io/api/description/custom-toolchain.html#set-kind
---@xmake.kind values
function toolchain_scope.set_kind(...) end

---@xmake.prototype set_runtimes(runtimes: string|array, ...)
---@xmake.param runtimes Runtime library string or array, such as "MT", "MD", "MTd", "MDd"
---@xmake.param ... Variable parameters, can pass multiple runtime library strings
---@see https://xmake.io/api/description/project-target.html#set-runtimes
---@xmake.kind values
function toolchain_scope.set_runtimes(...) end

---@xmake.prototype set_sdkdir(sdkdir: string)
---@xmake.param sdkdir SDK directory path string
---@see https://xmake.io/api/description/custom-toolchain.html#set-sdkdir
---@xmake.kind values
function toolchain_scope.set_sdkdir(...) end

---@xmake.prototype set_toolset(tool: string, tools: string|array, ...)
---@xmake.param tool Tool name string (cc, cxx, ld, sh, ar, ex, strip, mm, mxx, as)
---@xmake.param tools Tool program name string or array
---@xmake.param ... Variable parameters, can pass multiple tool names
---@see https://xmake.io/api/description/custom-toolchain.html#set-toolset
---@xmake.kind keyvalues
function toolchain_scope.set_toolset(...) end
