# ANC-DB: AI-Native Binary Storage Engine

ANC-DB (AI-Native Core Database) is an innovative binary storage engine optimized for direct manipulation by AI agents.

[日本語版はこちら (#japanese-version)]

## Concept: Complete Elimination of SQL

Traditional databases are operated through "SQL," which is designed for humans. ANC-DB is built on the premise that AI agents directly manipulate B-Tree structures at a low level. This provides the following benefits:

1.  **Deterministic Behavior**: Independent of the whims of SQL parsers or query planners, executing direct B-Tree operations exactly as the AI intended.
2.  **Total Transparency**: The mapping of data to the physical layer is clear to the AI, enabling more advanced reasoning and optimization.
3.  **Efficient Communication**: Minimal token consumption and parsing costs via a MessagePack-based binary protocol.

## Technical Stack

- **Core**: SQLite B-Tree API (Direct Manipulation)
- **Language**: Rust / C (Shim)
- **Protocol**: MessagePack (Length-prefixed Frame)

## Quick Start

### 1. Build
```bash
cargo build --release
```

### 2. Run (CLI)
```bash
./target/release/ancdb-cli --db-path my-ai-storage.db
```

### Protocol Operations
AI agents or clients send commands such as `CreateTable`, `Put`, `DirectRead`, and `RangeScan` in binary format. For details, refer to `ANC-DB-SPEC-STRICT-v1.2.md`.

---

<a name="japanese-version"></a>
# ANC-DB: AIネイティブ・バイナリストレージエンジン (Japanese)

ANC-DB (AI-Native Core Database) は、AIエージェントによる直接操作に最適化された、革新的なバイナリストレージエンジンです。

## コンセプト: 徹底した SQL の排除

従来のデータベースは人間が読み書きする「SQL」を介して操作しますが、ANC-DB は AI エージェントが直接 B-Tree 構造を低レイヤーで操作することを前提に設計されています。

1.  **決定論的な動作**: SQL パーサーやクエリプランナーの気まかさに左右されず、AI が意図した通りの B-Tree 操作を直接実行。
2.  **圧倒的な透過性**: データの物理層へのマッピングが AI にとって明白であり、より高度な推論と最適化が可能。
3.  **効率的な通信**: MessagePack ベースのバイナリプロトコルにより、トークン消費とパースコストを最小化。

## 技術スタック

- **Core**: SQLite B-Tree API (Direct Manipulation)
- **Language**: Rust / C (Shim)
- **Protocol**: MessagePack (Length-prefixed Frame)

## ライセンス / License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.

### Third-party Acknowledgement

ANC-DB utilizes the [SQLite](https://www.sqlite.org/) storage engine core. SQLite is dedicated to the public domain and is used under its [Public Domain dedication](https://www.sqlite.org/copyright.html).
