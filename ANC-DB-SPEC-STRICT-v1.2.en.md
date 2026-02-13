# ANC-DB Strict Specification v1.2 (Concept-Strict)

**Last Updated**: 2026-02-13  
**Status**: Authoritative (This specification takes precedence)

## 0. Purpose of this Specification

This specification defines the constraints to ensure the core concept of ANC-DB:  
**"Excluding SQL strings not only from external but also from internal execution paths."**

This document is a **mandatory requirement**, not a recommendation.

---

## 1. Non-Negotiables

1. SQL string execution must not be performed in any runtime data operation path.
2. Primary operations must be implemented using `sqlite3Btree*` + `BtCursor`.
3. CRUD via `sqlite3_exec` / `sqlite3_prepare*` / `sqlite3_step` is strictly prohibited.
4. The protocol must only accept structured commands via MessagePack.
5. Compliance with the above must be mechanically verified via CI gates.

---

## 2. Allowed APIs / Prohibited APIs

### 2.1 Allowed APIs (v1)

- `sqlite3BtreeOpen`
- `sqlite3BtreeClose`
- `sqlite3BtreeBeginTrans`
- `sqlite3BtreeCommitPhaseOne`
- `sqlite3BtreeCommitPhaseTwo`
- `sqlite3BtreeRollback`
- `sqlite3BtreeCreateTable`
- `sqlite3BtreeCursor`
- `sqlite3BtreeCloseCursor`
- `sqlite3BtreeTableMoveto`
- `sqlite3BtreeFirst`
- `sqlite3BtreeLast`
- `sqlite3BtreeNext`
- `sqlite3BtreePrevious`
- `sqlite3BtreeEof`
- `sqlite3BtreeIntegerKey`
- `sqlite3BtreePayloadSize`
- `sqlite3BtreePayload` (or `PayloadFetch`)
- `sqlite3BtreeInsert`
- `sqlite3BtreeDelete`

### 2.2 Prohibited APIs (Execution Path)

- `sqlite3_exec`
- `sqlite3_prepare`, `sqlite3_prepare_v2`, `sqlite3_prepare_v3`
- `sqlite3_step`
- `sqlite3_finalize`
- `sqlite3_get_table`
- Proprietary wrappers that accept SQL strings.

*Note: Prohibitions apply to the "production data operation path." Build helpers or debug-only tools must be isolated into separate binaries.*

---

## 3. Architectural Constraints

- **Protocol Layer**: Receives `Command` via MessagePack.
- **Rust Layer**: `Command -> Core operation` mapping only. SQL generation is prohibited.
- **C Shim Layer**: Exclusive use for `sqlite3Btree*` calls.
- **Storage Layer**: SQLite B-Tree/Pager/VFS.

*Mandatory: Prohibit any string processing that constructs SQL statements within `ancdb-core` itself.*

---

## 4. Data Model (v1 Fixed)

- `TableId = u32`
- `PrimaryKey = i64` (`INTKEY/rowid`)
- `Value = bytes`
- No secondary indexes in v1.

---

## 5. Protocol (v1 Fixed)

Required Commands:

- `CreateTable` (Experimental/Catalog managed)
- `BeginTx(write)`
- `CommitTx`
- `RollbackTx`
- `DirectRead`
- `Put`
- `Delete`
- `RangeScan`
- `AtomicUpdate`
- `BatchWrite` (max=1000)

Response:

- `ProtocolResponse::Ok { result, meta }`
- `ProtocolResponse::Err { status, message, meta }`

---

## 6. Quality Gates (Release Forbidden if Violated)

### 6.1 Static Gates

Zero occurrences of the following must be found:

```bash
rg -n "sqlite3_exec|sqlite3_prepare(_v2|_v3)?|sqlite3_step|sqlite3_finalize" crates/ancdb-core/src
```

`sqlite3Btree` calls must be confirmed via:

```bash
rg -n "sqlite3Btree(Open|Close|BeginTrans|CommitPhaseOne|CommitPhaseTwo|Rollback|CreateTable|Cursor|TableMoveto|Insert|Delete|Payload|IntegerKey|First|Next|Eof)" crates/ancdb-core/src/c_shim
```

### 6.2 Dynamic Gates

- `cargo test --workspace` all pass.
- Integration tests verify CRUD/Range/Tx.
- stdio round-trip tests pass.

---

## 7. Release Criteria

"ANC-DB v1 Official" status is granted only when all following are met:

- Non-Negotiables 1-5 are satisfied.
- All Static/Dynamic gates pass.
- `docs/benchmark-report.md` is updated.
- Known limitations are clearly stated in `docs/open-issues.md`.

---

## 8. Treatment of Current Code

Any implementation violating this specification is considered a **provisional implementation**, even if it works.  
Provisional implementations must be tagged with:

- `prototype`
- `not-concept-compliant`

---

## 9. Regression Prevention Rules

1. Specification changes must update `ANC-DB-SPEC-STRICT-v1.2.md` via PR first.
2. PRs that only change implementation without spec updates are prohibited.
3. Prohibited API grep results must be attached during review.
4. No exceptions allowed (verbal agreements are invalid).
