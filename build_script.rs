fn main() {
    pkg_config::Config::new()
        .probe("libarchive")
        .expect("libarchive not found!");
}
