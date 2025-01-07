fn main() {
    let git_short_hash = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap())
        .unwrap();
    println!("cargo:rustc-env=GIT_SHORT_HASH={}", git_short_hash);
}
