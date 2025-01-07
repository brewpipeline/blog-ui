fn main() {
    let git_hash = std::process::Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap())
        .unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
