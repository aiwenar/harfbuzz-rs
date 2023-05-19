use pkg_config::Config;
use semver::{Version, VersionReq};

fn main() {
    let lib = Config::new()
        .probe("harfbuzz")
        .expect("could not locate HarfBuzz");
    let version = Version::parse(&lib.version)
        .expect("HarfBuzz was found but has invalid version");
    let req = VersionReq::parse(&">= 2.0.0, < 8.0.0").unwrap();

    if !req.matches(&version) {
        panic!("Incompatible HarfBuzz version. Found {} but we require {}", version, req);
    }
}
