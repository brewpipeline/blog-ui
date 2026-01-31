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

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lang_path = format!("{manifest_dir}/.lang_code");
    if std::fs::read_to_string(&lang_path).ok().as_deref() != Some(lang_code) {
        std::fs::write(&lang_path, lang_code).unwrap();
    }
}
