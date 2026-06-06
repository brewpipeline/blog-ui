fn main() {
    let git_hash = std::process::Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap())
        .unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    #[cfg(feature = "lang_ru")]
    let lang_code = "ru";
    #[cfg(not(feature = "lang_ru"))]
    let lang_code = "en";

    #[cfg(feature = "tikitko")]
    let tikitko = "1";
    #[cfg(not(feature = "tikitko"))]
    let tikitko = "0";

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let title = std::env::var("TITLE").unwrap_or_default();
    let description = std::env::var("DESCRIPTION").unwrap_or_default();

    write_if_changed(format!("{manifest_dir}/.lang_code"), lang_code);
    write_if_changed(format!("{manifest_dir}/.tikitko"), tikitko);
    write_if_changed(format!("{manifest_dir}/.title"), &title);
    write_if_changed(format!("{manifest_dir}/.description"), &description);
}

fn write_if_changed(path: String, content: &str) {
    if std::fs::read_to_string(&path).ok().as_deref() != Some(content) {
        std::fs::write(&path, content).unwrap();
    }
}
