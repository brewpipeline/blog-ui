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

    println!("cargo:rerun-if-changed=index.template.html");

    let template = std::fs::read_to_string(format!("{manifest_dir}/index.template.html")).unwrap();
    let output = template.replace("__LANG__", lang_code);

    let output_path = format!("{manifest_dir}/index.html");
    if std::fs::read_to_string(&output_path).ok().as_deref() != Some(output.as_str()) {
        std::fs::write(&output_path, &output).unwrap();
    }
}
