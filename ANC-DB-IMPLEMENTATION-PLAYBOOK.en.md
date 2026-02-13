# ANC-DB Implementation Playbook (Efficient Path, v1)

**Last Updated**: 2026-02-13  
**Target**: Implementation of `ANC-DB-SPEC-v1.1.md`

## 0. Objective

Build a "working v1" in the shortest time possible to minimize back-tracing costs.  
The principle is `Core First`:

1. Fix the minimal SQLite B-Tree API set.
2. Encapsulate the API with a safe Rust wrapper.
3. Layer the MessagePack protocol on top.
4. Perform final performance optimizations.

## 1. Initial Repository Structure (Day 0)

Directories to create:

- `docs/`
- `crates/ancdb-core` (FFI + safe wrapper)
- `crates/ancdb-protocol` (MessagePack dispatcher)
- `crates/ancdb-cli` (For verification)
- `third_party/sqlite` (Fixed source version)
- `tests/integration`
- `benches`

Done Condition:

- `cargo check --workspace` passes.
- `cargo test --workspace` does not fail (even with 0 tests).

## 2. Phase Execution Order (Fixed)

### Phase A: Finalizing Minimal SQLite Dependencies (Highest Priority)

Steps:

1. Place the fixed version of SQLite in `third_party/sqlite`.
2. Create `docs/sqlite-minimal-btree-api.md`.
3. Define the following for each operation (`DirectRead/RangeScan/Put/Delete/AtomicUpdate`):
   - Required C functions
   - Call sequence
   - Rollback procedure on error

Deliverables:

- `docs/sqlite-minimal-btree-api.md`
- `docs/sqlite-function-dependency-notes.md`

Done Condition:

- The "Selected Function Set" is documented and agreed upon not to change during implementation.

Abort Condition:

- If requirements for anything other than `INTKEY(rowid)` become mandatory, redesign Phase A.

### Phase B: Minimal C Core Build

Steps:

1. Statically link the SQLite core in `build.rs`.
2. Fix compilation options that disable the SQL layer.
3. Minimize public shim functions on the C side (`ancdb_c_*`).

Deliverables:

- `crates/ancdb-core/build.rs`
- `crates/ancdb-core/src/ffi.rs`
- `crates/ancdb-core/src/c_shim/*`

Done Condition:

- Rust can call `open/close/begin/commit/rollback`.
- No major issues detected in memory leak checks.

### Phase C: Rust Safety Wrapper

Steps:

1. Implement `Database`, `Transaction`, and `Cursor`.
2. Isolate `unsafe` code exclusively to `ffi.rs`.
3. Unify the error type as `AncError`.

Public API (Required for v1):

- `open(path)`
- `direct_read(table_id, key)`
- `range_scan(table_id, start, end, limit, direction)`
- `put(table_id, key, value)`
- `delete(table_id, key)`
- `atomic_update(...)`

Done Condition:

- Unit tests for success/failure paths pass.
- No `panic` crashes the process.

### Phase D: Protocol Layer (MessagePack)

Steps:

1. Define Command/Response structs.
2. Implement the dispatcher with validation.
3. Verify communication via `stdin/stdout` using the CLI.

Deliverables:

- `crates/ancdb-protocol/src/command.rs`
- `crates/ancdb-protocol/src/dispatcher.rs`
- `crates/ancdb-cli`

Done Condition:

- Major commands can round-trip successfully.
- Returns appropriate error codes for invalid input.

### Phase E: Quality Gates (Minimum for Production Quality)

Required Tests:

- Crash-recovery test.
- Long-running write cycle test.
- Concurrent read + single write contention test.
- Corruption detection test.

Performance Measurement:

- `DirectRead` p50/p95.
- `RangeScan (100 items)` p50/p95.
- `BatchWrite (1000 items)` p50/p95.

Done Condition:

- Reproducible benchmark results are recorded in `docs/benchmark-report.md`.
- 0 critical bugs remains.

## 3. Parallel Work Breakdown (Efficiency)

Parallel Lane A:

- Core/FFI/Transactions.

Parallel Lane B:

- Protocol/CLI/Test Harness.

Merge Point:

- Fix `AncError` and `Command` definitions first before integration.

## 4. Implementation Rules (Preventing Indecision)

- v1 will not implement anything other than `INTKEY(rowid)`.
- "Optimization" is prohibited until Phase E.
- APIs can be added but their semantics must not be changed.
- Undecided items must be recorded immediately in `docs/adr/`.

## 5. Daily Operations Template

At Start:

1. Choose only one target phase for the day.
2. Specify deliverable files.
3. Write Done conditions first.

At End:

1. Save test results.
2. Add unresolved issues to `docs/open-issues.md`.
3. Clearly state the first task for the next day.

## 6. Initial 7 Tasks (Start in this order)

1. Create `docs/sqlite-minimal-btree-api.md`.
2. Create the `crates/ancdb-core` skeleton.
3. Implement `open/close/begin/commit/rollback`.
4. Implement `direct_read/put/delete`.
5. Implement `range_scan`.
6. Implement MessagePack `DirectRead/Put/Delete`.
7. Add an integration test (`tests/integration/smoke.rs`).

Completion of these 7 tasks defines the "v1 Minimal Working Version."
