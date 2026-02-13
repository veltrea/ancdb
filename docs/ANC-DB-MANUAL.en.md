# ANC-DB: ARCHITECTURAL MANUAL

**Last Updated**: 2026-02-13  
**Status**: General Release

## 1. Concept

ANC-DB stands for "AI-Native Core Database." It is a storage engine designed to be directly driven by AI agents, eliminating the overhead and uncertainty of SQL.

- **Non-SQL Interface**: Uses the Atomic Native Binary Protocol (ANBP) via MessagePack.
- **Direct B-Tree Access**: Skips the SQL parser/optimizer to manipulate the underlying storage engine directly.
- **Embedded Performance**: Built on SQLite's robust B-Tree/Pager layer but accessed via an optimized Rust/C bypass.

## 2. Architecture

- **Protocol Layer**: Handles length-prefixed MessagePack commands.
- **Core Layer (Rust)**: Manages transactions, concurrency, and safety.
- **FFI Layer (C Shim)**: Provides direct access to internal `sqlite3Btree*` functions.
- **Storage Layer**: SQLite standard file format.

## 3. Build & Run

### Prerequisites
- Rust 1.70+
- Clang/LLVM (for C shim compilation)

### Build
```bash
cargo build --release
```

### Execution
```bash
./target/release/ancdb-cli --db-path ./my-database.db --stdio
```

## 4. Testing

- **Smoke Tests**: Basic CRUD verification.
- **Integration Tests**: stdio framing and recovery.
- **Benchmarks**: P50/P95 latency measurement.

Run all tests:
```bash
cargo test --workspace
```
