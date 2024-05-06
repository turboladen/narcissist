use std::fs;
use std::path::Path;

use narcissist::Specification;

#[test]
fn validate_files_test() {
    let dir = Path::new("packages/");

    for entry in fs::read_dir(dir).expect("Couldn't read_dir") {
        let entry = entry.unwrap();
        let _ = Specification::open(entry.path())
            .unwrap_or_else(|_| panic!("File is bad: `{}`", entry.path().display()));
    }
}
