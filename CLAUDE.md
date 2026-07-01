# ocel-rs

OCEL 2.0 データモデル・I/O・グラフライブラリ (Rust)。

## プロジェクト状態

- フェーズ: spec / architecture 確定 → v0.1 実装着手
- North Star: OSS 実用性（crates.io で使われる OCEL 2.0 ネイティブ Rust ライブラリ）
- 計画・ADR・ロードマップは workspace リポジトリ（`ocel-workspace/docs/`）が正

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

このリポジトリは **spec と architecture のみ**を持つ（計画・研究・ADR・ロードマップは workspace が正）。

- `docs/development-guide.md` - 開発ガイド（TDD、コード品質、コミット規約）
- `docs/architecture.md` - v0.1 アーキテクチャ（クレート/モジュール構成、モデル、API 制約）
- `docs/spec-v0.1.md` - v0.1 仕様（スコープ、I/O、バリデーション、受け入れ基準）

計画・研究・意思決定は workspace リポジトリを参照:
- `ocel-workspace/docs/roadmap.md`, `ocel-workspace/docs/adr/`, product-brief, research-notes ほか

## 開発規約

`docs/development-guide.md` を参照。要点:

- TDD（テスト駆動開発）
- `cargo nextest run` / `cargo clippy -- -D warnings` / `cargo fmt --check` / `cargo deny check` を全て通過させる
- conventional commits（英語）

## 次のアクション

v0.1（core の read/write/validate）の実装。詳細は `docs/spec-v0.1.md` と GitHub Issues を参照。
