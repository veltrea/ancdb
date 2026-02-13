# ANC-DB Implementation Task List

**Last Updated**: 2026-02-13

## 1. Current Progress Summary

- **Completed**: Workspace initialization, `ancdb-core` basic Tx, `direct_read/put/delete`, `range_scan`, `atomic_update`, `batch_write`, MessagePack I/F (with version/meta), `ancdb-cli --stdio` (length-prefixed multiple commands), integration tests, benchmarks & report generation, in-DB catalog persistence for table metadata (restore after restart).
- **Remaining**: None (v1 task list complete).

## 2. Task Checklist

### A. Infrastructure Setup
- [x] Create Cargo workspace (`ancdb-core`, `ancdb-protocol`, `ancdb-cli`).
- [x] Create `docs/sqlite-minimal-btree-api.md`.
- [x] Implement `open/close/begin/commit/rollback` API skeleton.

### B. Core API (Required for v1)
- [x] `direct_read`
- [x] `put`
- [x] `delete`
- [x] `range_scan`
- [x] `atomic_update`
- [x] `batch_write` + `on_conflict` (`replace/ignore/fail`).
- [x] Table metadata management per `table_id` (create/check existence).
- [x] `table_id -> root_page` catalog persistence (restore after restart).
- [x] Separation of `begin_read_tx` / `begin_write_tx`.

### C. Protocol (MessagePack)
- [x] Serde support for `Command` / `CommandResult`.
- [x] Encode/decode commands.
- [x] Encode/decode responses.
- [x] `handle_messagepack_command`.
- [x] Basic error code mapping (3, 4, 5, 6, 7, 100).
- [x] Add protocol versioning field (`major.minor`).
- [x] Standardize metadata (`execution_time_us`, `rows_affected`).
- [x] `BatchWrite` count validation (e.g., max 1000 items).

### D. CLI / Execution Interface
- [x] Demo execution path (non-stdio).
- [x] `--stdio` single command processing.
- [x] `--stdio` multiple command sequential processing (length-prefixed).
- [x] Unified error response for invalid input.
- [x] support for `--db-path` argument.

### E. SQLite Engine Integration (Highest Priority)
- [x] Place fixed SQLite version in `third_party/sqlite`.
- [x] Statically link SQLite core in `build.rs`.
- [x] Replace mock implementation in `ffi.rs` with actual SQLite C shim calls.

### F. Testing Enhancements
- [x] Unit tests (core/protocol).
- [x] Add `tests/integration/smoke.rs`.
- [x] Add integration tests for stdio round-trip.
- [x] Add tests for contention retries (`TransactionConflict`).
- [x] Add skeleton for corruption/recovery tests.

### G. Quality & Commercialization Gates
- [x] Crash recovery test.
- [x] Long-running continuous write test.
- [x] Concurrent read + single write test.
- [x] Benchmark implementation (`p50/p95`).
- [x] Create `docs/benchmark-report.md`.
- [x] Create `docs/open-issues.md` and start operations.

## 3. Recommended Order for Next Sprint
- [x] 1) Replace with actual SQLite FFI (Late E).
- [x] 2) Implement stdio framing (Early D).
- [x] 3) Polish integration tests (F).
- [x] 4) Add minimal versions of crash/benchmark (G).
