use ancdb_core::{Database, TxMode};
use tempfile::tempdir;

#[test]
fn test_smoke_write_read() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Database::open(&db_path).unwrap();

    db.begin_tx(TxMode::Write).unwrap();
    db.create_table(10, "test_table").unwrap();
    db.put(10, 1, b"hello").unwrap();
    db.commit_tx().unwrap();

    let val = db.read(10, 1).unwrap();
    assert_eq!(val, Some(b"hello".to_vec()));
}
