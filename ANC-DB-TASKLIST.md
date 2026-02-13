# ANC-DB 実装タスクリスト

最終更新: 2026-02-13

## 1. 現在の進捗サマリ

- 完了: ワークスペース初期化、`ancdb-core` 基本Tx、`direct_read/put/delete`、`range_scan`、`atomic_update`、`batch_write`、MessagePack I/F（version/metaつき）、`ancdb-cli --stdio`（長さプレフィックス複数コマンド）、統合テスト、ベンチ実行とレポート作成、テーブルメタのDB内カタログ永続化（再起動復元）
- 残タスク: なし（v1タスクリスト完了）

## 2. タスク一覧（チェック式）

### A. 基盤セットアップ

- [x] Cargo workspace 作成（`ancdb-core`, `ancdb-protocol`, `ancdb-cli`）
- [x] `docs/sqlite-minimal-btree-api.md` 作成
- [x] `open/close/begin/commit/rollback` API骨格実装

### B. コアAPI（v1必須）

- [x] `direct_read`
- [x] `put`
- [x] `delete`
- [x] `range_scan`
- [x] `atomic_update`
- [x] `batch_write` + `on_conflict` (`replace/ignore/fail`)
- [x] `table_id` ごとのテーブルメタ管理（作成・存在確認）
- [x] `table_id -> root_page` カタログ永続化（再起動後復元）
- [x] `begin_read_tx` / `begin_write_tx` の明示API分離

### C. Protocol（MessagePack）

- [x] `Command` / `CommandResult` のserde対応
- [x] `encode/decode command`
- [x] `encode/decode response`
- [x] `handle_messagepack_command`
- [x] エラーコード基本マッピング（3,4,5,6,7,100）
- [x] プロトコルバージョニング字段 (`major.minor`) 追加
- [x] メタ情報（`execution_time_us`, `rows_affected`）標準化
- [x] `BatchWrite` の上限件数バリデーション（例: 1000件）

### D. CLI / 実行インターフェース

- [x] デモ実行パス（非stdio）
- [x] `--stdio` 1コマンド処理
- [x] `--stdio` 複数コマンド連続処理（長さプレフィックス）
- [x] 不正入力時のエラーレスポンス統一
- [x] `--db-path` 引数対応

### E. SQLite実エンジン接続（最重要）

- [x] `third_party/sqlite` に固定版SQLite配置
- [x] `build.rs` でSQLiteコア静的リンク
- [x] `ffi.rs` のモック実装を実SQLite C shim呼び出しへ置換

### F. テスト強化

- [x] ユニットテスト（core/protocol）
- [x] `tests/integration/smoke.rs` 追加
- [x] stdio往復の統合テスト追加
- [x] 競合時リトライのテスト追加（`TransactionConflict`）
- [x] 破損/回復のテスト雛形追加

### G. 品質・商用化ゲート

- [x] Crash recovery テスト
- [x] 長時間連続書き込みテスト
- [x] 同時read + single write テスト
- [x] ベンチマーク (`p50/p95`) 実装
- [x] `docs/benchmark-report.md` 作成
- [x] `docs/open-issues.md` 作成と運用開始

## 3. 直近スプリントでやる順番（推奨）

- [x] 1) SQLite実FFI置換（Eの後半）
- [x] 2) stdioフレーミング実装（Dの前半）
- [x] 3) integrationテスト整備（F）
- [x] 4) crash/benchmarkの最小版を追加（G）
