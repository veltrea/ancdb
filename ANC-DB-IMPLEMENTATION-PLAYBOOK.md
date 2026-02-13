# ANC-DB 実装手順書 (Efficient Path, v1)

最終更新: 2026-02-13  
対象: `ANC-DB-SPEC-v1.1.md` の実装

## 0. 目的

最短で「動くv1」を作り、後戻りコストを最小化する。  
方針は `Core First`:

1. SQLite B-Tree最小APIを固定  
2. Rust安全ラッパーでAPIを閉じる  
3. MessagePackプロトコルを載せる  
4. 最後に性能最適化

## 1. リポジトリ初期構成 (Day 0)

作成するディレクトリ:

- `docs/`
- `crates/ancdb-core` (FFI + safe wrapper)
- `crates/ancdb-protocol` (MessagePack dispatcher)
- `crates/ancdb-cli` (検証用)
- `third_party/sqlite` (固定版ソース)
- `tests/integration`
- `benches`

Done条件:

- `cargo check --workspace` が通る
- `cargo test --workspace` にテスト0件でも失敗しない

## 2. フェーズ別の実行順 (固定)

## Phase A: SQLite最小依存の確定 (最優先)

実施:

1. SQLiteの固定バージョンを `third_party/sqlite` に配置  
2. `docs/sqlite-minimal-btree-api.md` を作成  
3. `DirectRead/RangeScan/Put/Delete/AtomicUpdate` ごとに:
   - 必須C関数
   - 呼び順
   - エラー時rollback
   を表で確定

成果物:

- `docs/sqlite-minimal-btree-api.md`
- `docs/sqlite-function-dependency-notes.md`

Done条件:

- 「採用関数セット」が文書化され、以降の実装で変更しない合意がある

中止条件:

- `INTKEY(rowid)` 以外が必須になる要求が出たら、Phase Aを再設計

## Phase B: Cコアの最小ビルド

実施:

1. `build.rs` でSQLiteコアを静的リンク  
2. SQL層を有効化しないコンパイルオプションを固定  
3. C側の公開シム関数を最小化 (`ancdb_c_*`)

成果物:

- `crates/ancdb-core/build.rs`
- `crates/ancdb-core/src/ffi.rs`
- `crates/ancdb-core/src/c_shim/*`

Done条件:

- Rustから `open/close/begin/commit/rollback` が呼べる
- メモリリーク検出で重大問題なし

## Phase C: Rust安全ラッパー

実施:

1. `Database`, `Transaction`, `Cursor` を実装  
2. `unsafe` を `ffi.rs` のみに隔離  
3. エラー型 `AncError` を統一

公開API (v1必須):

- `open(path)`
- `direct_read(table_id, key)`
- `range_scan(table_id, start, end, limit, direction)`
- `put(table_id, key, value)`
- `delete(table_id, key)`
- `atomic_update(...)`

Done条件:

- ユニットテストで正常系/異常系が通る
- `panic` でプロセスが落ちない

## Phase D: Protocol層 (MessagePack)

実施:

1. コマンド/レスポンス構造体を定義  
2. バリデーション付きディスパッチャ実装  
3. CLIで `stdin/stdout` 経由の疎通確認

成果物:

- `crates/ancdb-protocol/src/command.rs`
- `crates/ancdb-protocol/src/dispatcher.rs`
- `crates/ancdb-cli`

Done条件:

- 主要コマンドが往復できる
- 不正入力で適切なエラーコードを返す

## Phase E: 品質ゲート (商用品質の最低線)

必須テスト:

- Crash-recoveryテスト
- 長時間書き込みテスト
- 同時read + single write競合テスト
- 破損検知テスト

性能計測:

- `DirectRead` p50/p95
- `RangeScan(100件)` p50/p95
- `BatchWrite(1000件)` p50/p95

Done条件:

- 再現可能なベンチ結果が `docs/benchmark-report.md` に出る
- クリティカルバグが0件

## 3. 並行作業の切り方 (効率化)

並行レーンA:

- Core/FFI/トランザクション

並行レーンB:

- Protocol/CLI/テストハーネス

合流ポイント:

- `AncError` と `Command` 定義を先に固定してから統合

## 4. 実装ルール (迷い防止)

- v1は `INTKEY(rowid)` 以外を実装しない
- 「最適化」はPhase Eまで禁止
- APIは増やしてよいが、意味変更はしない
- 仕様未確定事項は `docs/adr/` に1件1ファイルで即決記録

## 5. 毎日の運用テンプレート

開始時:

1. 当日対象フェーズを1つだけ選ぶ
2. 成果物ファイルを明示する
3. Done条件を先に書く

終了時:

1. テスト結果を保存
2. 未解決課題を `docs/open-issues.md` に追記
3. 翌日の最初の1タスクを明記

## 6. 最初の7タスク (この順で着手)

1. `docs/sqlite-minimal-btree-api.md` 作成  
2. `crates/ancdb-core` の雛形作成  
3. `open/close/begin/commit/rollback` 実装  
4. `direct_read/put/delete` 実装  
5. `range_scan` 実装  
6. MessagePack `DirectRead/Put/Delete` 実装  
7. Integration test 追加 (`tests/integration/smoke.rs`)

この7タスク完了時点で「v1最小動作品」とする。
