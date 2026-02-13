# ANC-DB: AI-Native Core Database

AIエージェント向けに、SQL文字列生成を介さずに扱えるMessagePackベースの軽量データベース実装です。  
コアのCRUD/scanはSQLiteのB-Tree APIを直接利用し、SQL実行APIを通しません。

## 主な特徴

- **AI Native**: LLMエージェントが「直接」バイナリ操作を行えるプロトコル設計。
- **SQLite Core**: 信頼性の高いSQLiteのB-Tree APIをバックエンドに使用。
- **No SQL**: SQLパーサー、クエリプランナー、実行エンジンをバイパスし、直接B-Treeページを操作。
- **MessagePack**: 通信プロトコルにMessagePackを採用し、低遅延・高効率なデータ交換を実現。

## 構成

- `crates/ancdb-core`: データベースエンジンコア（Rust）。
- `crates/ancdb-protocol`: MessagePackベースの通信定義。
- `crates/ancdb-cli`: 評価用CLIツール。

## セットアップ

### 必要条件

- Rust 1.70.0+
- Clang / LLVM (Cシムのビルド用)

### ビルド

```bash
cargo build --release
```

### テスト

```bash
cargo test
```
