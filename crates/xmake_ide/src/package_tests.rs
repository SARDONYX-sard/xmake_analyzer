//! Package features end to end: a repository on disk, then completion,
//! diagnostics, hover and navigation.

use std::{path::PathBuf, sync::Arc};

use xmake_syntax::Dialect;

use crate::PackageIndex;

/// A project with a repository holding `zlib`.
fn project_with_repository(tag: &str) -> (crate::Project, PathBuf) {
    let dir = std::env::temp_dir().join(format!("xmake-ide-pkg-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let package = dir.join("repositories/xmake-repo/packages/z/zlib");
    std::fs::create_dir_all(&package).expect("dirs");
    let definition = "package('zlib')
set_description('Compression')
add_versions('v1.2.13', 'a')
add_versions('v1.3', 'b')
add_configs('zutil', {description = 'Export zutil.h'})
";
    std::fs::write(package.join("xmake.lua"), definition).expect("file");
    let mut project = crate::Project::new(xmake_api::ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
    project.packages = Arc::new(PackageIndex::load(Some(&dir), &[]));
    (project, dir)
}

fn labels(project: &mut crate::Project, text: &str) -> Vec<String> {
    let offset = text.find('$').expect("marker");
    let text = text.replacen('$', "", 1);
    let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text.as_str()));
    let items =
        crate::completions(project, &file, None, xmake_syntax::TextSize::new(offset as u32));
    items.into_iter().map(|i| i.label.into_owned()).collect()
}

#[test]
fn with_a_repository() {
    let (mut project, dir) = project_with_repository("all");

    let text = "add_requires('zlbi', 'zlib 1.4', 'zlib v1.3', 'zlib 1.2.x', 'conan::foo')
target('a')
add_packages('zlib', 'zlbi', 'conan::foo')
";
    let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text));
    let found: Vec<_> = crate::diagnostics(&project, &file, &crate::LintLevels::default())
        .into_iter()
        .filter(|d| d.code.starts_with("unknown-package"))
        .map(|d| (d.message.into_owned(), d.fixes.first().map(|f| f.replacement.to_string())))
        .collect();
    assert_eq!(
        found,
        [
            ("unknown package `zlbi`: no repository has it".to_owned(), Some("zlib".to_owned())),
            (
                "package `zlib` has no version `1.4` (latest: `v1.3`)".to_owned(),
                Some("v1.3".to_owned())
            ),
        ]
    );

    assert_eq!(labels(&mut project, "add_requires('zl$')"), ["zlib"]);
    assert_eq!(labels(&mut project, "add_requires('zlib $')"), ["v1.3", "v1.2.13"]);
    let options = labels(&mut project, "add_requires('zlib', {$})");
    assert!(
        options.iter().any(|o| o == "configs") && options.iter().any(|o| o == "optional"),
        "{options:?}"
    );
    let configs = labels(&mut project, "add_requires('zlib', {configs = {$}})");
    assert_eq!(configs.first().map(String::as_str), Some("zutil"), "{configs:?}");
    assert!(configs.iter().any(|c| c == "shared"));

    let text = "add_requires('zlib')";
    let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text));
    let hover = crate::hover(&project, &file, xmake_syntax::TextSize::new(15)).expect("hover");
    assert!(
        hover.markdown.contains("Compression") && hover.markdown.contains("`v1.3`, `v1.2.13`"),
        "{}",
        hover.markdown
    );
    let path = PathBuf::from("xmake.lua");
    let targets = crate::goto_definition(&project, &file, &path, xmake_syntax::TextSize::new(15));
    assert!(targets[0].path.ends_with("packages/z/zlib/xmake.lua"), "{targets:?}");

    let _ = std::fs::remove_dir_all(&dir);
}
