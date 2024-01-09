use emerald::{DefaultEmerald, Emerald};
use std::path::PathBuf;

#[test]
fn test_read_in_test_vault() {
    let vault_path = PathBuf::from("./tests/test_vault");
    let emerald = DefaultEmerald::new(&vault_path).unwrap();

    assert_eq!(emerald.file_count(), 11, "check number of files");
    assert_eq!(emerald.md_file_count(), 10, "check number of md files");
    assert_eq!(
        emerald.valid_backlink_count(),
        14,
        "check number of valid backlinks"
    );
    assert_eq!(
        emerald.invalid_backlink_count(),
        1,
        "check number of invalid backlinks"
    );
}
