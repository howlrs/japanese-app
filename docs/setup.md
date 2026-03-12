# japanese-app セットアップガイド

## 前提条件

- Rust (Edition 2024 対応版)
- Google Gemini API キー

## 環境変数

| 変数名 | 必須 | 説明 |
|--------|------|------|
| `GOOGLE_GEMINI_API_KEY` | Yes | Google Gemini API の認証キー |
| `GEMINI_MODELS` | Yes | 使用するGeminiモデル名（カンマ区切り、2つ指定） |
| `RUST_LOG` | No | ログレベル（デフォルト: INFO） |

## ビルド

```bash
cd scripts
cargo build --release
```

## 実行

### 問題生成

```bash
cargo run --bin create_questions
```

- N2/N3レベルの問題をGemini APIで生成
- レベルあたり30回のAPIリクエスト
- 出力: `output/questions/{level}/{timestamp}.json`

### ファイル結合

```bash
cargo run --bin template
```

- 生成された全JSONファイルを1つに結合
- 出力: `concat_all.md`

### 構造化パース

```bash
cargo run --bin json_read_to_struct
```

- 結合JSONを構造体にパース・検証
- 出力: `concat_with_struct.json`

## 出力ディレクトリ

```
output/questions/
├── n2/          # N2レベル問題
│   └── *.json   # タイムスタンプ名のJSONファイル群
└── n3/          # N3レベル問題
    └── *.json
```
