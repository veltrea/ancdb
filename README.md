# ANC-DB: AI-Native Binary Storage Engine

ANC-DB (AI-Native Core Database) は、AIエージェントによる直接操作に最適化された、革新的なバイナリストレージエンジンです。

## コンセプト: 徹底した SQL の排除

従来のデータベースは人間が読み書きする「SQL」を介して操作しますが、ANC-DB は AI エージェントが直接 B-Tree 構造を低レイヤーで操作することを前提に設計されています。これにより、以下のメリットを享受できます。

1.  **決定論的な動作**: SQL パーサーやクエリプランナーの気まかさに左右されず、AI が意図した通りの B-Tree 操作を直接実行。
2.  **圧倒的な透過性**: データの物理層へのマッピングが AI にとって明白であり、より高度な推論と最適化が可能。
3.  **効率的な通信**: MessagePack ベースのバイナリプロトコルにより、トークン消費とパースコストを最小化。

## 技術スタック

- **Core**: SQLite B-Tree API (Direct Manipulation)
- **Language**: Rust / C (Shim)
- **Protocol**: MessagePack (Length-prefixed Frame)

## クイックスタート

### 1. ビルド
```bash
cargo build --release
```

### 2. 起動 (CLI)
```bash
./target/release/ancdb-cli --db-path my-ai-storage.db
```

### プロトコル経由の操作
AIエージェントやクライアントは、`CreateTable`, `Put`, `DirectRead`, `RangeScan` といったコマンドをバイナリ形式で送出します。詳細は `ANC-DB-SPEC-STRICT-v1.2.md` を参照してください。

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.

### Third-party Acknowledgement

ANC-DB utilizes the [SQLite](https://www.sqlite.org/) storage engine core. SQLite is dedicated to the public domain and is used under its [Public Domain dedication](https://www.sqlite.org/copyright.html).
