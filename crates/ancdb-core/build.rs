fn main() {
    cc::Build::new()
        .file("src/c_shim/ancdb_sqlite_shim.c")
        .include("../../third_party/sqlite")
        .define("SQLITE_ENABLE_BTREE_API", Some("1")) // Marker if useful
        .compile("ancdb_sqlite_shim");

    println!("cargo:rerun-if-changed=src/c_shim/ancdb_sqlite_shim.c");
}
