# ocel-rs 開発ガイド

## 開発方針

- **TDD（テスト駆動開発）**: テストを先に書き、テストが通るように実装する
- **仕様準拠**: OCEL 2.0 仕様（arXiv:2403.01975）と公式スキーマに忠実に実装する
- **最小限の実装**: 必要な機能だけを実装し、over-engineering を避ける

## TDD ワークフロー

1. テストケースを書く（失敗する状態）
2. 最小限のコードで通す
3. リファクタリング
4. `cargo nextest run` で全テスト通過を確認
5. `cargo clippy -- -D warnings` で lint 通過を確認
6. コミット

## テスト構成

### ユニットテスト

各モジュール内に `#[cfg(test)] mod tests` で配置。

### 統合テスト

`tests/` ディレクトリに配置。テストフィクスチャは `tests/fixtures/` に格納。

### テストフィクスチャ

PM4Py の OCEL 2.0 テストデータを使用:

| ファイル | 形式 | 元 URL |
|---------|------|--------|
| `ocel20_example.jsonocel` | JSON | PM4Py tests/input_data/ocel/ |
| `ocel20_example.xmlocel` | XML | PM4Py tests/input_data/ocel/ |
| `ocel20_example.sqlite` | SQLite | PM4Py tests/input_data/ocel/ |

### テスト種別

| 種別 | 対象 | データ |
|------|------|--------|
| パーサーテスト | JSON/XML/SQLite の読み込み | PM4Py フィクスチャ |
| ライターテスト | JSON/XML/SQLite の書き出し | 自作データ |
| ラウンドトリップテスト | 読み→書き→読みの一致 | PM4Py フィクスチャ |
| バリデーションテスト | OCEL 2.0 仕様準拠チェック | 公式スキーマ + 22 SQL クエリ |
| エッジケーステスト | 空ログ、同一qualifier重複等 | 自作最小ファイル |

## コード品質チェック

実装後、以下を全て通過させること:

```sh
cargo fmt --check          # フォーマット
cargo clippy -- -D warnings # lint (pedantic)
cargo nextest run          # テスト
cargo deny check           # ライセンス・脆弱性
```

## Clippy 設定

`Cargo.toml` の `[workspace.lints]` で管理（clippy pedantic 有効）。
詳細はルート `Cargo.toml` を参照。

## コミット規約

- conventional commits スタイル（`feat:`, `fix:`, `refactor:`, `test:`, `docs:` 等）
- コミットメッセージは英語
- 機能単位・修正単位でこまめにコミット
- 1 コマンド = 1 シェル呼び出し（`&&` でチェーンしない）

## ブランチ戦略

- `main`: 安定ブランチ
- `feat/xxx`: 新機能
- `fix/xxx`: バグ修正
- `refactor/xxx`: リファクタリング

## 依存クレートの追加

- `Cargo.toml` の `[workspace.dependencies]` で一元管理
- 追加時は `cargo deny check` でライセンス・脆弱性を確認
- `cargo machete` で未使用依存を定期チェック

## 参考資料

- OCEL 2.0 仕様: https://arxiv.org/html/2403.01975v1
- OCEL 2.0 公式: https://www.ocel-standard.org/
- JSON Schema: https://www.ocel-standard.org/2.0/ocel20-schema-json.json
- XML XSD: https://www.ocel-standard.org/2.0/ocel20-schema-xml.xsd
- SQLite 検証: https://www.ocel-standard.org/2.0/ocel20-schema-relational.pdf
