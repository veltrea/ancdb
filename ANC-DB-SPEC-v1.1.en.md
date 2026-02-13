# AI-Native Core Database (ANC-DB) Integrated Specification v1.1

**Last Updated**: 2026-02-13  
**Status**: Draft (Implementation Ready for v1 Core)

## 1. Purpose and Scope

ANC-DB is an embedded database in the same class as SQLite, aiming to achieve the following:

- Eliminate SQL string generation and parsing from the application public API.
- Provide a binary API that AI agents can operate reliably with low token consumption.
- Inherit the ACID properties of the SQLite core to ensure stability for commercial use.

Non-Goals:

- Distributed execution at the level of PostgreSQL, complex query optimization, or full SQL compatibility.

## 2. Architecture

A 3-layer structure is adopted:

- **Storage Engine (C)**: SQLite B-Tree + Pager + VFS.
- **Safety Layer (Rust)**: FFI encapsulation, handle management, and type validation.
- **Protocol Layer**: MessagePack-based AI-native API.

Key Policies:

- In v1, the Pager API is not exposed externally. ACID is utilized via the B-Tree API.
- v1 focuses on `INTKEY(rowid)` tables.
- Complex secondary indexes will be phased in starting from v2.

## 3. Data Model (v1)

- `TableId`: `u32`
- `PrimaryKey`: `i64` (SQLite rowid compatible)
- `Value`: `bytes`
- Schema validation is performed on the Rust side; the storage layer remains as byte sequences.

Note:

- Adopt `#[derive(Schema)]` and `#[indexed]` as proposed.
- However, in v1, the mandatory scope is limited to "Primary Key + Basic Attribute Validation."

## 4. AI Binary Protocol (ANBP)

### 4.1 Encoding

- v1: Fixed at MessagePack.
- v2 Candidates: Addition of Protobuf.

### 4.2 Commands

Required:

- `DirectRead {table_id, key}`
- `RangeScan {table_id, start_key, end_key, limit, direction}`
- `Put {table_id, key, value}`
- `Delete {table_id, key}`
- `AtomicUpdate {table_id, key, precondition, op}`
- `BeginTx`, `CommitTx`, `RollbackTx`

Recommended:

- `BatchWrite {table_id, records[], on_conflict}` (Max 1000 items)

### 4.3 Response

- `status`: 0=success, >0=error.
- `data`: Result payload.
- `meta`: `rows_affected`, `execution_time_us`.
- `error`: Human-readable message (optional).

### 4.4 Error Codes

- 0: Success
- 1: InvalidCommand
- 2: SchemaNotFound
- 3: KeyNotFound
- 4: ConstraintViolation
- 5: TransactionConflict
- 100+: InternalError

## 5. SQLite Minimal Function Set (v1)

Minimal functions required to directly operate B-Tree without passing through the SQL parser.

**Connection**:
- `sqlite3BtreeOpen`
- `sqlite3BtreeClose`

**Transaction**:
- `sqlite3BtreeBeginTrans`
- `sqlite3BtreeCommitPhaseOne`
- `sqlite3BtreeCommitPhaseTwo`
- `sqlite3BtreeRollback`

**Cursor**:
- `sqlite3BtreeCursor`
- `sqlite3BtreeCloseCursor`
- `sqlite3BtreeTableMoveto`
- `sqlite3BtreeFirst`
- `sqlite3BtreeLast`
- `sqlite3BtreeNext`
- `sqlite3BtreePrevious`
- `sqlite3BtreeEof`

**Read**:
- `sqlite3BtreeIntegerKey`
- `sqlite3BtreePayloadSize`
- `sqlite3BtreePayload` (or `sqlite3BtreePayloadFetch`)

**Update**:
- `sqlite3BtreeInsert`
- `sqlite3BtreeDelete`

**Table Management**:
- `sqlite3BtreeCreateTable`
- `sqlite3BtreeClearTable` (Optional)

Note:
- Since `sqlite3BtreeCommit` has internal implementation variations, v1 uses `CommitPhaseOne/Two` as the standard.
- `sqlite3BtreeMovetoUnpacked` is not mandatory in the v1 binary API focused on INTKEY.

## 6. Rust Safety Layer

Adopted:
- `Arc<RwLock<...>>` (actually `Mutex` based handles in current impl).
- `unsafe` limited to the `ffi` module.
- RAII Transactions (automatic rollback on `Drop`).
- Unify error codes into `Result<T, AncError>`.

Constraints:
- v1 Concurrency: Multiple Readers + Single Writer.
- Busy/Retry provided via exponential backoff.

## 7. Decisions on Anthropic Proposals

Adopted as-is:
- 3-layer structure.
- MessagePack-centric binary protocol.
- Emphasis on `BatchWrite`.
- Rust type-driven schema.
- Error code system.
- Phased roadmap.

Adopted with modifications:
- "Zero token" claim for interaction is refined to "Significantly reduced token consumption vs. String SQL."
- "10x Faster" is not a fixed KPI; `p50/p95` benchmark evaluation is used instead.
- WAL enablement is managed via SQLite internal APIs/settings, not SQL execution.

Postponed to v2+:
- Full-Text index.
- Native embedding quantization.
- Lock-free structure.
- Advanced optimization of auto-derivation macros.

## 8. Commercial Quality Requirements

- **OS**: macOS / Linux / Windows.
- **Durability**: Mandatory crash recovery tests.
- **Reliability**: Long-running write/reboot cycle tests.
- **Observability**: Operation logs and internal metrics.
- **Backward Compatibility**: Define Protocol version (`major.minor`).

## 9. Implementation Phases

- **M0**: Minimal B-Tree Implementation (`DirectRead/Put/Delete/RangeScan`).
- **M1**: Tx + `AtomicUpdate` + Retry.
- **M2**: Protocol Stabilization + Python Client.
- **M3**: Basic Schema derive.
- **M4**: Fault tolerance testing + Commercial release prep.

## 10. Initial Specific Instructions for AI Agents (Revised)

Execute the following instructions first:

1. Refer to `src/btree.h` as primary information and extract required functions assuming `INTKEY`.
2. For each operation (`DirectRead/RangeScan/Put/Delete/AtomicUpdate`):
   - Required functions
   - Call sequence
   - Rollback procedure on failure
   Output in tabular format.
3. Explain why SQL-related dependencies (`parse`, `tokenize`, `prepare`, `vdbe`) are unnecessary.
4. Output File: `docs/sqlite-minimal-btree-api.en.md` (and the Japanese counterpart).
