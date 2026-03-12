# japanese-app 概要

## プロジェクト情報

| 項目 | 内容 |
|------|------|
| リポジトリ | [howlrs/japanese-app](https://github.com/howlrs/japanese-app) |
| 言語 | Rust (Edition 2024) |
| ライセンス | Non-Commercial License |
| 本番URL | https://jlpt.howlrs.net/ |
| 紹介動画 | https://www.youtube.com/watch?v=I4o_v7d3yR8 |

## 概要

JLPT（日本語能力試験）対策学習アプリのメインリポジトリ。現在はGoogle Gemini APIを利用したJLPT問題自動生成スクリプト群を格納している。

## 全体システム構成

本プロジェクトは複数コンポーネントで構成される：

| コンポーネント | 役割 | リポジトリ |
|---------------|------|-----------|
| **scripts (本リポジトリ)** | 問題生成パイプライン | japanese-app |
| **backend** | 問題・回答提供API | [jlpt-app-backend](../../jlpt-app-backend/docs/) |
| **scripts (拡張)** | データ加工・DB投入パイプライン | [jlpt-app-scripts](../../jlpt-app-scripts/docs/) |

## 主要技術スタック

- **Rust** - メイン言語
- **Google Generative AI (Gemini)** - AI問題生成
- **Tokio** - 非同期ランタイム
- **Serde / serde_json** - JSON シリアライゼーション
- **Chrono** - 日時処理

## このリポジトリの責務

1. Google Gemini APIを呼び出してJLPT問題をJSON形式で自動生成
2. 生成されたJSONを構造体にパースし検証
3. 複数ファイルの結合・整形
