use std::fs;

fn main() {
    let cargo_toml = fs::read_to_string("Cargo.toml").unwrap();
    let release_date = cargo_toml
        .lines()
        .find(|line| line.trim().starts_with("release-date"))
        .map(|line| {
            line.split('=')
                .nth(1)
                .unwrap_or("unknown")
                .trim()
                .trim_matches('"')
                .to_string()
        })
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=CARGO_PKG_RELEASE_DATE={}", release_date);
}
