# japanese-app アーキテクチャ

## ディレクトリ構成

```
japanese-app/
├── README.md              # プロジェクト概要
├── LICENSE                # Non-Commercial License
├── scripts/
│   ├── Cargo.toml         # Rust パッケージ定義
│   ├── Cargo.lock         # 依存関係ロック
│   ├── README.md          # スクリプト説明
│   ├── bin/               # 実行可能スクリプト群
│   │   ├── 00_template.rs         # ファイルI/Oテンプレート
│   │   ├── 0_create_questions.rs  # AI問題生成
│   │   └── 1_json_read_to_struct.rs # JSON構造化
│   └── prompts/           # AIプロンプトテンプレート
│       ├── base-info.md           # JLPTの基本情報
│       ├── n2/                    # N2レベルプロンプト
│       └── n3/                    # N3レベルプロンプト
```

## スクリプトパイプライン

```
[Google Gemini API]
        │
        ▼
0_create_questions.rs
  ├── プロンプト読込 (base-info.md + レベル別)
  ├── Gemini APIに30回リクエスト/レベル
  ├── 15秒間隔、失敗時60秒リトライ
  └── output/questions/{level}/{timestamp}.json に保存
        │
        ▼
00_template.rs
  ├── 指定ディレクトリの全ファイルを走査
  └── 全ファイル内容を concat_all.md に結合
        │
        ▼
1_json_read_to_struct.rs
  ├── concat_all.json を読込
  ├── Vec<Question> にデシリアライズ
  ├── 構造検証
  └── concat_with_struct.json に保存（整形済み）
```

## バイナリ定義

| バイナリ名 | ソースファイル | 機能 |
|-----------|--------------|------|
| `template` | `bin/00_template.rs` | ファイル読込・結合ユーティリティ |
| `create_questions` | `bin/0_create_questions.rs` | Gemini APIによる問題生成 |
| `json_read_to_struct` | `bin/1_json_read_to_struct.rs` | JSONの構造体パース・検証 |

## データモデル

### Question

```rust
struct Question {
    id: u32,                              // 問題ID
    level_id: u32,                        // レベルID
    level_name: String,                   // "N2", "N3" 等
    category_id: u32,                     // カテゴリID
    category_name: String,                // カテゴリ名
    chapter: String,                      // 章・セクション
    sentence: String,                     // 問題文
    prerequisites: String,               // 前提条件
    sub_questions: Vec<SubQuestion>,      // 小問リスト
}
```

### SubQuestion

```rust
struct SubQuestion {
    id: u32,                              // 小問ID
    hint_id: u32,                         // ヒントID
    answer_id: u32,                       // 回答ID
    sentence: String,                     // 小問文
    select_answer: Vec<HashMap<String, String>>,  // 選択肢
    answer: String,                       // 正解
}
```

## 依存関係

| クレート | バージョン | 用途 |
|---------|-----------|------|
| tokio | 1.43.0 | 非同期ランタイム |
| google-generative-ai-rs | 0.3.4 | Gemini APIクライアント |
| serde | 1.0.218 | シリアライゼーション |
| serde_json | 1.0.140 | JSON処理 |
| chrono | 0.4.40 | タイムスタンプ生成 |
| env_logger | 0.11.6 | ログ出力 |
| log | 0.4.26 | ログファサード |
