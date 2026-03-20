# ocel-rs

OCEL 2.0 データモデル・I/O・グラフライブラリ (Rust)。

## プロジェクト状態

- フェーズ: リサーチ完了 → プロダクトブリーフ作成中
- BMAD ワークフロー: ~~リサーチ~~ → プロダクトブリーフ → PRD → アーキテクチャ → 実装

## スコープ

ocel-rs は OCEL 2.0 のコアライブラリに徹する。ETL やコネクタは別リポジトリ。

- OCEL 2.0 データモデル
- I/O（SQLite, JSON, XML）
- オブジェクトグラフ構築・クエリ
- フィルタリング・サンプリング
- バリデーション
- CLI
- WASM / Python バインディング（後のフェーズ）

**スコープ外（別リポジトリ）:**
- ETL パイプラインエンジン → `ocel-etl`
- データソースコネクタ → 各コネクタリポジトリ

## 技術スタック

- 言語: Rust (1.94.0, pinned via rust-toolchain.toml)
- バインディング: WASM (ブラウザ/Node.js), Python (PyO3)
- OCEL 2.0 フォーマット: SQLite, JSON, XML
- パッケージマネージャ: cargo

## ワークスペース構造

```
crates/
├── ocel-core/    # データモデル + I/O + グラフ + バリデーション
└── ocel-cli/     # CLI ツール
```

## ドキュメント

- `docs/development-guide.md` - 開発ガイド（TDD、コード品質、コミット規約）
- `docs/research-notes.md` - リサーチノート（OCEL 2.0 仕様詳細、テストデータ戦略含む）
- `docs/ocel2-comprehensive-research.md` - OCEL 2.0 包括的リサーチ（背景、歴史、産業動向）
- `docs/research-etl-architecture-2026-03-16.md` - ETL アーキテクチャリサーチ
- `docs/product-brief-ocel-workspace-2026-03-18.md` - ワークスペースプロダクトブリーフ

## 開発規約

`docs/development-guide.md` を参照。要点:

- TDD（テスト駆動開発）
- `cargo nextest run` / `cargo clippy -- -D warnings` / `cargo fmt --check` / `cargo deny check` を全て通過させる
- conventional commits（英語）

## 次のアクション

1. プロダクトブリーフ完成
2. PRD 作成
3. アーキテクチャ設計
