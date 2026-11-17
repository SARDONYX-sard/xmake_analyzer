//! The values some APIs accept as string arguments, for completion and checks.

/// Where the accepted values of an argument come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgValues {
    /// A fixed list of `(value, description)`.
    Fixed(&'static [(&'static str, &'static str)]),
    /// [`crate::KnownValues::platforms`].
    Platforms,
    /// [`crate::KnownValues::architectures`].
    Architectures,
    /// [`crate::KnownValues::modes`].
    Modes,
    /// [`crate::KnownValues::rules`] plus the rules of the project.
    Rules,
    /// [`crate::KnownValues::policies`].
    Policies,
    /// [`crate::KnownValues::toolchains`] plus those of the project.
    Toolchains,
    /// The targets of the project.
    Targets,
    /// The options of the project.
    Options,
    /// The options of the project and the builtin configurations
    /// ([`BUILTIN_CONFIGS`]): what `get_config` and `is_config` read.
    Configs,
    /// The packages required by the project.
    Packages,
    /// Importable modules: `core.base.json`, ...
    ImportModules,
}

impl ArgValues {
    /// Whether every valid value is known, so others can be reported.
    pub fn is_closed(self) -> bool {
        matches!(self, ArgValues::Fixed(_))
    }
}

const KINDS: &[(&str, &str)] = &[
    ("binary", "An executable program."),
    ("static", "A static library."),
    ("shared", "A shared (dynamic) library."),
    ("object", "Object files only, no link."),
    ("headeronly", "A header only library."),
    ("moduleonly", "A C++20 module only library."),
    ("phony", "No output: groups dependencies."),
];

const LANGUAGES: &[(&str, &str)] = &[
    ("ansi", "ANSI C"),
    ("c89", "C89"),
    ("c90", "C90"),
    ("c99", "C99"),
    ("c11", "C11"),
    ("c17", "C17"),
    ("c23", "C23"),
    ("clatest", "The latest C standard supported by the compiler."),
    ("gnu89", "C89 with GNU extensions"),
    ("gnu99", "C99 with GNU extensions"),
    ("gnu11", "C11 with GNU extensions"),
    ("gnu17", "C17 with GNU extensions"),
    ("gnu23", "C23 with GNU extensions"),
    ("gnulatest", "The latest C standard with GNU extensions."),
    ("cxx98", "C++98"),
    ("cxx03", "C++03"),
    ("cxx11", "C++11"),
    ("cxx14", "C++14"),
    ("cxx17", "C++17"),
    ("cxx20", "C++20"),
    ("cxx23", "C++23"),
    ("cxx26", "C++26"),
    ("cxxlatest", "The latest C++ standard supported by the compiler."),
    ("c++11", "C++11"),
    ("c++14", "C++14"),
    ("c++17", "C++17"),
    ("c++20", "C++20"),
    ("c++23", "C++23"),
    ("c++26", "C++26"),
    ("c++latest", "The latest C++ standard supported by the compiler."),
    ("gnuxx11", "C++11 with GNU extensions"),
    ("gnuxx14", "C++14 with GNU extensions"),
    ("gnuxx17", "C++17 with GNU extensions"),
    ("gnuxx20", "C++20 with GNU extensions"),
    ("gnuxx23", "C++23 with GNU extensions"),
    ("gnuxxlatest", "The latest C++ standard with GNU extensions."),
];

const WARNINGS: &[(&str, &str)] = &[
    ("none", "Disable all warnings."),
    ("less", "Enable fewer warnings."),
    ("more", "Enable more warnings."),
    ("extra", "Enable extra warnings."),
    ("all", "Enable all warnings."),
    ("allextra", "Enable all and extra warnings."),
    ("everything", "Enable every warning the compiler has."),
    ("pedantic", "Enable pedantic warnings."),
    ("error", "Treat warnings as errors."),
];

const OPTIMIZE: &[(&str, &str)] = &[
    ("none", "No optimization."),
    ("fast", "Fast optimization."),
    ("faster", "Faster optimization."),
    ("fastest", "Fastest optimization."),
    ("smallest", "Optimize for size."),
    ("aggressive", "Aggressive, possibly unsafe, optimization."),
];

const SYMBOLS: &[(&str, &str)] = &[
    ("debug", "Generate debug symbols."),
    ("hidden", "Hide exported symbols by default."),
    ("none", "No debug symbols."),
    ("edit", "Debug symbols for edit and continue (MSVC)."),
    ("embed", "Embed debug symbols in object files (MSVC)."),
];

const STRIP: &[(&str, &str)] =
    &[("none", "Do not strip."), ("debug", "Strip debug symbols."), ("all", "Strip all symbols.")];

const FPMODELS: &[(&str, &str)] = &[
    ("fast", "Fast floating point."),
    ("strict", "Strict floating point."),
    ("except", "Floating point exceptions."),
    ("precise", "Precise floating point."),
];

const RUNTIMES: &[(&str, &str)] = &[
    ("MT", "MSVC static runtime."),
    ("MTd", "MSVC static debug runtime."),
    ("MD", "MSVC dynamic runtime."),
    ("MDd", "MSVC dynamic debug runtime."),
    ("c++_static", "LLVM libc++, static."),
    ("c++_shared", "LLVM libc++, shared."),
    ("stdc++_static", "GNU libstdc++, static."),
    ("stdc++_shared", "GNU libstdc++, shared."),
];

const EXCEPTIONS: &[(&str, &str)] = &[
    ("cxx", "Enable C++ exceptions."),
    ("objc", "Enable Objective-C exceptions."),
    ("no-cxx", "Disable C++ exceptions."),
    ("no-objc", "Disable Objective-C exceptions."),
    ("none", "Disable exceptions."),
];

const ENCODINGS: &[(&str, &str)] = &[
    ("utf-8", "UTF-8 for source and execution."),
    ("source.utf-8", "UTF-8 source files."),
    ("target.utf-8", "UTF-8 execution character set."),
    ("gb2312", "GB2312"),
];

const TASK_CATEGORIES: &[(&str, &str)] = &[
    ("task", "A plain task (default)."),
    ("action", "A builtin action, like `build`."),
    ("plugin", "A plugin, shown by `xmake --help`."),
    ("main", "The main menu."),
];

/// The configurations of `xmake f` every project has, besides its options
/// (from `xmake f --help` of xmake 3.1.1, platforms and toolchains included).
pub const BUILTIN_CONFIGS: &[(&str, &str)] = &[
    ("plat", "The target platform (`xmake f -p`)."),
    ("arch", "The target architecture (`xmake f -a`)."),
    ("mode", "The build mode (`xmake f -m`)."),
    ("kind", "The default kind of targets (`xmake f -k`)."),
    ("toolchain", "The toolchain (`xmake f --toolchain`)."),
    ("buildir", "The build directory (`xmake f -o`)."),
    ("builddir", "The build directory (`xmake f -o`)."),
    ("host", "The host system."),
    ("cc", "The C compiler."),
    ("cxx", "The C++ compiler."),
    ("ld", "The linker."),
    ("sh", "The shared library linker."),
    ("ar", "The static library archiver."),
    ("sdk", "The SDK directory (`xmake f --sdk`)."),
    ("vs", "The Visual Studio version (`xmake f --vs`)."),
    ("vs_toolset", "The Visual Studio toolset version."),
    ("vs_sdkver", "The Windows SDK version."),
    ("runtimes", "The C/C++ runtimes (`xmake f --runtimes`)."),
    ("policies", "The build policies (`xmake f --policies`)."),
    ("ccache", "Whether ccache is enabled."),
    ("require", "A configuration of `xmake f --require`."),
    ("pkg_searchdirs", "A configuration of `xmake f --pkg_searchdirs`."),
    ("cross", "A configuration of `xmake f --cross`."),
    ("target_os", "A configuration of `xmake f --target_os`."),
    ("bin", "A configuration of `xmake f --bin`."),
    ("toolchain_host", "A configuration of `xmake f --toolchain_host`."),
    ("ascflags", "A configuration of `xmake f --ascflags`."),
    ("aicpuflags", "A configuration of `xmake f --aicpuflags`."),
    ("ldflags", "A configuration of `xmake f --ldflags`."),
    ("arflags", "A configuration of `xmake f --arflags`."),
    ("shflags", "A configuration of `xmake f --shflags`."),
    ("links", "A configuration of `xmake f --links`."),
    ("syslinks", "A configuration of `xmake f --syslinks`."),
    ("linkdirs", "A configuration of `xmake f --linkdirs`."),
    ("includedirs", "A configuration of `xmake f --includedirs`."),
    ("pc", "A configuration of `xmake f --pc`."),
    ("pcld", "A configuration of `xmake f --pcld`."),
    ("pcsh", "A configuration of `xmake f --pcsh`."),
    ("as", "A configuration of `xmake f --as`."),
    ("asflags", "A configuration of `xmake f --asflags`."),
    ("zc", "A configuration of `xmake f --zc`."),
    ("zcld", "A configuration of `xmake f --zcld`."),
    ("zcar", "A configuration of `xmake f --zcar`."),
    ("zcsh", "A configuration of `xmake f --zcsh`."),
    ("cpp", "A configuration of `xmake f --cpp`."),
    ("ranlib", "A configuration of `xmake f --ranlib`."),
    ("cxflags", "A configuration of `xmake f --cxflags`."),
    ("cxxflags", "A configuration of `xmake f --cxxflags`."),
    ("frameworks", "A configuration of `xmake f --frameworks`."),
    ("frameworkdirs", "A configuration of `xmake f --frameworkdirs`."),
    ("dc", "A configuration of `xmake f --dc`."),
    ("dcld", "A configuration of `xmake f --dcld`."),
    ("dcar", "A configuration of `xmake f --dcar`."),
    ("dcsh", "A configuration of `xmake f --dcsh`."),
    ("sc", "A configuration of `xmake f --sc`."),
    ("scld", "A configuration of `xmake f --scld`."),
    ("scsh", "A configuration of `xmake f --scsh`."),
    ("scar", "A configuration of `xmake f --scar`."),
    ("mm", "A configuration of `xmake f --mm`."),
    ("mxx", "A configuration of `xmake f --mxx`."),
    ("mflags", "A configuration of `xmake f --mflags`."),
    ("mxflags", "A configuration of `xmake f --mxflags`."),
    ("mxxflags", "A configuration of `xmake f --mxxflags`."),
    ("cu", "A configuration of `xmake f --cu`."),
    ("cu-ccbin", "A configuration of `xmake f --cu-ccbin`."),
    ("culd", "A configuration of `xmake f --culd`."),
    ("cuflags", "A configuration of `xmake f --cuflags`."),
    ("culdflags", "A configuration of `xmake f --culdflags`."),
    ("cs", "A configuration of `xmake f --cs`."),
    ("csflags", "A configuration of `xmake f --csflags`."),
    ("mrc", "A configuration of `xmake f --mrc`."),
    ("mrcflags", "A configuration of `xmake f --mrcflags`."),
    ("fc", "A configuration of `xmake f --fc`."),
    ("fcld", "A configuration of `xmake f --fcld`."),
    ("fcsh", "A configuration of `xmake f --fcsh`."),
    ("go", "A configuration of `xmake f --go`."),
    ("gcld", "A configuration of `xmake f --gcld`."),
    ("go-ar", "A configuration of `xmake f --go-ar`."),
    ("cflags", "A configuration of `xmake f --cflags`."),
    ("nc", "A configuration of `xmake f --nc`."),
    ("ncld", "A configuration of `xmake f --ncld`."),
    ("ncar", "A configuration of `xmake f --ncar`."),
    ("ncsh", "A configuration of `xmake f --ncsh`."),
    ("rc", "A configuration of `xmake f --rc`."),
    ("rcld", "A configuration of `xmake f --rcld`."),
    ("rcar", "A configuration of `xmake f --rcar`."),
    ("rcsh", "A configuration of `xmake f --rcsh`."),
    ("kc", "A configuration of `xmake f --kc`."),
    ("kcld", "A configuration of `xmake f --kcld`."),
    ("kcar", "A configuration of `xmake f --kcar`."),
    ("kcsh", "A configuration of `xmake f --kcsh`."),
    ("ndk", "A configuration of `xmake f --ndk`."),
    ("ndk_sdkver", "A configuration of `xmake f --ndk_sdkver`."),
    ("android_sdk", "A configuration of `xmake f --android_sdk`."),
    ("build_toolver", "A configuration of `xmake f --build_toolver`."),
    ("ndk_stdcxx", "A configuration of `xmake f --ndk_stdcxx`."),
    ("ndk_cxxstl", "A configuration of `xmake f --ndk_cxxstl`."),
    ("cuda", "A configuration of `xmake f --cuda`."),
    ("cuda_sdkver", "A configuration of `xmake f --cuda_sdkver`."),
    ("qt", "A configuration of `xmake f --qt`."),
    ("qt_host", "A configuration of `xmake f --qt_host`."),
    ("qt_sdkver", "A configuration of `xmake f --qt_sdkver`."),
    ("vcpkg", "A configuration of `xmake f --vcpkg`."),
    ("mingw", "A configuration of `xmake f --mingw`."),
    ("emsdk", "A configuration of `xmake f --emsdk`."),
    ("vs_runtime", "A configuration of `xmake f --vs_runtime`."),
    ("wdk", "A configuration of `xmake f --wdk`."),
    ("wdk_sdkver", "A configuration of `xmake f --wdk_sdkver`."),
    ("wdk_winver", "A configuration of `xmake f --wdk_winver`."),
    ("debugger", "A configuration of `xmake f --debugger`."),
    ("ccachedir", "A configuration of `xmake f --ccachedir`."),
    ("trybuild", "A configuration of `xmake f --trybuild`."),
    ("tryconfigs", "A configuration of `xmake f --tryconfigs`."),
    ("xcode", "A configuration of `xmake f --xcode`."),
    ("xcode_sdkver", "A configuration of `xmake f --xcode_sdkver`."),
    ("xcode_bundle_identifier", "A configuration of `xmake f --xcode_bundle_identifier`."),
    ("xcode_codesign_identity", "A configuration of `xmake f --xcode_codesign_identity`."),
    ("xcode_mobile_provision", "A configuration of `xmake f --xcode_mobile_provision`."),
    ("target_minver", "A configuration of `xmake f --target_minver`."),
    ("appledev", "A configuration of `xmake f --appledev`."),
    ("xcode_sysroot", "A configuration of `xmake f --xcode_sysroot`."),
];

/// The values the `arg`-th argument of `name` accepts in `scope` (`None` in
/// scripts): some APIs mean other things in some scopes.
pub fn scoped_arg_values(scope: Option<crate::Scope>, name: &str, arg: usize) -> Option<ArgValues> {
    use crate::Scope;
    match (scope, name) {
        // The dependencies of a rule are rules; of a package, packages of
        // the repository.
        (Some(Scope::Rule), "add_deps") => Some(ArgValues::Rules),
        (Some(Scope::Package), "add_deps") => None,
        // Rules, packages and toolchains have their own kinds.
        (
            Some(Scope::Rule | Scope::Package | Scope::Toolchain | Scope::Option | Scope::Task),
            "set_kind",
        ) => None,
        _ => arg_values(name, arg),
    }
}

/// The tools of xmake (`modules/core/tools/*.lua`, xmake 3.1.1): what
/// `{tools = "cl"}` and `"cl::/W4"` of the flags name. A flag naming another
/// is never given: xmake does not complain.
pub const TOOLS: &[&str] = &[
    "ar",
    "ar2000",
    "ar6x",
    "armar",
    "armasm",
    "armasm64_msvc",
    "armasm_msvc",
    "armcc",
    "armclang",
    "armlink",
    "bisheng",
    "bl51",
    "c51",
    "cc",
    "circle",
    "cl",
    "cl2000",
    "cl6x",
    "cl_json",
    "clang",
    "clang_cl",
    "clangxx",
    "cosmoar",
    "cosmocc",
    "cosmocxx",
    "cparser",
    "cxx",
    "dmd",
    "dotnet",
    "dpcpp",
    "emar",
    "emcc",
    "emxx",
    "fasm",
    "filcc",
    "filxx",
    "flang",
    "fpc",
    "gcc",
    "gcc_ar",
    "gccgo",
    "gdc",
    "gfortran",
    "go",
    "gxx",
    "iarchive",
    "icc",
    "iccarm",
    "icl",
    "icpc",
    "icpx",
    "icx",
    "icx_cc",
    "ifort",
    "ifx",
    "ilinkarm",
    "kotlinc_native",
    "ld",
    "ld64_lld",
    "ld_lld",
    "ldc2",
    "link",
    "lld_link",
    "llvm_ar",
    "llvm_rc",
    "ml",
    "ml64",
    "mold",
    "nasm",
    "nim",
    "nvc",
    "nvcc",
    "nvcxx",
    "nvfortran",
    "rc",
    "rustc",
    "sdar",
    "sdasstm8",
    "sdcc",
    "swift_frontend",
    "swiftc",
    "tcc",
    "windres",
    "yasm",
    "zig",
    "zig_cc",
    "zig_cxx",
];

/// `{public = true}` of the values of a target: who else gets them.
const VISIBILITY: &[(&str, &str)] = &[
    ("public", "Also for the targets depending on this one."),
    ("interface", "Only for the targets depending on this one."),
    ("private", "Only for this target (the default)."),
];

/// Options of the compilation flags.
const FLAG_OPTIONS: &[(&str, &str)] = &[
    ("force", "Do not check whether the compiler supports the flags."),
    ("tools", "Only for these tools: `\"cl\"`, `{\"gcc\", \"clang\"}`."),
    ("public", "Also for the targets depending on this one."),
    ("interface", "Only for the targets depending on this one."),
];

const FILE_OPTIONS: &[(&str, &str)] = &[
    ("defines", "Macro definitions for these files only."),
    ("undefines", "Undefined macros for these files only."),
    ("includedirs", "Include directories for these files only."),
    ("cflags", "C flags for these files only."),
    ("cxflags", "C and C++ flags for these files only."),
    ("cxxflags", "C++ flags for these files only."),
    ("mflags", "Objective-C flags for these files only."),
    ("asflags", "Assembler flags for these files only."),
    ("languages", "Language standards for these files only."),
    ("force", "Flags not checked: `{force = {cxflags = \"-O3\"}}`."),
    ("sourcekind", "Compile as another kind of source: `\"cxx\"`, `\"cc\"`, ..."),
    ("rule", "Build these files with a rule."),
    ("always_added", "Add the files even if no rule claims their extension."),
];

const INSTALL_OPTIONS: &[(&str, &str)] = &[
    ("prefixdir", "The directory under the install directory."),
    ("rootdir", "Keep the paths relative to this directory."),
    ("filename", "Install under another file name."),
];

const CONFIGFILE_OPTIONS: &[(&str, &str)] = &[
    ("prefixdir", "The directory under the configuration directory."),
    ("rootdir", "Keep the paths relative to this directory."),
    ("filename", "Write under another file name."),
    ("pattern", "The pattern of the variables, default `\"${(.-)}\"`; e.g. `\"@(.-)@\"`."),
    ("variables", "Extra variables: `{NAME = value}`."),
    ("onlycopy", "Copy the file without replacing variables."),
];

const TEST_OPTIONS: &[(&str, &str)] = &[
    ("runargs", "The arguments of the program: `{\"--fast\"}`."),
    ("runenvs", "Environment variables of the program."),
    ("rundir", "The working directory of the program."),
    ("pass_outputs", "Pass when the output matches (string or Lua pattern list)."),
    ("fail_outputs", "Fail when the output matches."),
    ("trim_output", "Trim the output before matching."),
    ("plain", "Match the outputs as plain text, not patterns."),
    ("timeout", "Fail after this many milliseconds."),
    ("realtime_output", "Show the output while the test runs."),
    ("build_should_pass", "Only build: the test passes when the build does."),
    ("build_should_fail", "Only build: the test passes when the build fails."),
    ("kind", "Build the test as another kind of target."),
    ("files", "Extra source files of the test."),
    ("remove_files", "Source files the test does not build."),
    ("defines", "Extra macro definitions of the test."),
    ("packages", "Extra packages of the test."),
    ("languages", "Language standards of the test."),
    ("group", "The group of the test, for `xmake test -g`."),
];

const DEP_OPTIONS: &[(&str, &str)] = &[
    ("inherit", "`false`: only the build order, nothing inherited."),
    ("public", "Also for the targets depending on this one."),
    ("interface", "Only for the targets depending on this one."),
];

const VERSION_OPTIONS: &[(&str, &str)] = &[
    ("build", "The format of the build version (`os.date`): `\"%Y%m%d%H%M\"`."),
    ("soname", "Version the shared library file names (`true`, or a version)."),
];

/// The keys of the options table an API takes as last argument:
/// `add_files("a.c", {defines = "A"})`. Empty when unknown.
pub fn option_keys(
    name: &str,
    kind: Option<crate::ApiKind>,
) -> &'static [(&'static str, &'static str)] {
    match name {
        "add_files" | "add_extrafiles" => FILE_OPTIONS,
        "add_tests" => TEST_OPTIONS,
        "add_deps" => DEP_OPTIONS,
        "set_version" => VERSION_OPTIONS,
        "add_headerfiles" | "add_installfiles" => INSTALL_OPTIONS,
        "add_configfiles" => CONFIGFILE_OPTIONS,
        _ if name.starts_with("add_") && name.ends_with("flags") => FLAG_OPTIONS,
        _ if name.starts_with("add_")
            && matches!(kind, Some(crate::ApiKind::Values | crate::ApiKind::Paths)) =>
        {
            VISIBILITY
        }
        _ => &[],
    }
}

/// The values the `arg`-th argument of the API `name` accepts.
///
/// Values APIs take any number of values, so `arg` only matters for APIs
/// whose later arguments differ (`set_policy(name, value)`).
pub fn arg_values(name: &str, arg: usize) -> Option<ArgValues> {
    let values = match name {
        "set_kind" | "is_kind" => ArgValues::Fixed(KINDS),
        "set_languages" => ArgValues::Fixed(LANGUAGES),
        "set_warnings" => ArgValues::Fixed(WARNINGS),
        "set_optimize" => ArgValues::Fixed(OPTIMIZE),
        "set_symbols" => ArgValues::Fixed(SYMBOLS),
        "set_strip" => ArgValues::Fixed(STRIP),
        "set_fpmodels" => ArgValues::Fixed(FPMODELS),
        "set_runtimes" => ArgValues::Fixed(RUNTIMES),
        "set_exceptions" => ArgValues::Fixed(EXCEPTIONS),
        "set_encodings" => ArgValues::Fixed(ENCODINGS),
        "set_category" => ArgValues::Fixed(TASK_CATEGORIES),
        "is_plat" | "is_os" | "is_host" | "is_subhost" | "set_plat" | "set_allowedplats"
        | "set_defaultplat" => ArgValues::Platforms,
        "is_arch" | "is_subarch" | "set_arch" | "set_allowedarchs" | "set_defaultarchs" => {
            ArgValues::Architectures
        }
        "is_mode" | "set_allowedmodes" | "set_defaultmode" => ArgValues::Modes,
        "add_rules" | "set_rules" => ArgValues::Rules,
        "set_policy" if arg == 0 => ArgValues::Policies,
        "set_toolchains" | "add_toolchains" => ArgValues::Toolchains,
        "add_deps" => ArgValues::Targets,
        "add_options" | "set_options" => ArgValues::Options,
        "has_config" => ArgValues::Configs,
        "get_config" | "is_config" | "set_config" if arg == 0 => ArgValues::Configs,
        "add_packages" | "has_package" => ArgValues::Packages,
        "import" if arg == 0 => ArgValues::ImportModules,
        _ => return None,
    };
    Some(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookups() {
        assert!(matches!(arg_values("set_kind", 0), Some(ArgValues::Fixed(_))));
        assert_eq!(arg_values("set_policy", 1), None);
        assert_eq!(arg_values("add_deps", 3), Some(ArgValues::Targets));
        assert_eq!(arg_values("print", 0), None);
        assert!(option_keys("add_tests", None).iter().any(|(k, _)| *k == "runargs"));
        assert!(option_keys("add_cxxflags", None).iter().any(|(k, _)| *k == "force"));
        assert!(
            option_keys("add_includedirs", Some(crate::ApiKind::Paths))
                .iter()
                .any(|(k, _)| *k == "public")
        );
        assert!(option_keys("set_kind", Some(crate::ApiKind::Values)).is_empty());
    }
}
