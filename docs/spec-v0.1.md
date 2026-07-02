# ocel-rs v0.1 仕様

- **バージョン:** v0.1
- **目的:** OCEL 2.0 の 3 フォーマット（JSON / SQLite / XML）を Rust で壊れずに読み書き・検証できるライブラリを crates.io に公開する。
- **上位方針:** [architecture.md](architecture.md), [ADR 0001: OCEL ネイティブ core](adr/0001-ocel-native-core.md)

---

## 1. スコープ

**In:**
- OCEL 2.0 データモデル（Event / Object / E2O / O2O / 動的オブジェクト属性）
- JSON / SQLite / XML の read + write
- バリデーション（仕様準拠）
- CLI（`convert` / `validate`）
- crates.io 公開品質（docs.rs / examples / README / semver）

**Out（v0.2 以降）:**
- Python (PyO3) バインディング、Arrow/Polars 列出力 → v0.2
- ETL パイプライン、Backlog コネクタ → v0.3+
- グラフクエリ / フィルタリング / サンプリングの高度な機能 → v0.1 では最小限（round-trip に必要な範囲）

## 2. データモデル要件

- 仕様 arXiv:2403.01975 の 6 コアエンティティ（Event Types / Object Types / Events / Objects / E2O / O2O）を表現する。
- **属性値の型:** `string` / `integer` / `float` / `boolean` / `time`（ISO 8601）。
- **動的オブジェクト属性:** タイムスタンプ付きで変更履歴を保持し、任意時点の値を forward-fill で復元できる。初期値はエポック時刻。
- **E2O / O2O の qualifier:** 任意文字列。同一 (event, object) ペアが異なる qualifier で複数関連できる。
- **公開 API:** `OcelBuilder`（fallible）・イテレータ走査・列アクセサを提供（[ADR 0001](adr/0001-ocel-native-core.md)）。

## 3. I/O 要件

| フォーマット | read | write | 参照 |
|-------------|------|-------|------|
| JSON | 必須 | 必須 | `ocel20-schema-json.json` |
| SQLite | 必須 | 必須 | `ocel20-schema-relational.pdf`（型別テーブル） |
| XML | 必須 | 必須 | `ocel20-schema-xml.xsd` |

- **round-trip 一致（最重要）:** 公式データを `read → write → read` して意味的に一致すること。フォーマット横断（例: JSON→メモリ→SQLite→メモリ）でも一致すること。
- JSON の属性値は文字列で格納されるため、型定義に基づいて read 時に `AttrValue` へ変換する。
- SQLite は型別テーブル名（`event_{Type}` / `object_{Type}`）を動的に扱い、テーブル名をサニタイズする。

## 4. バリデーション要件

- **JSON / XML:** 公式スキーマ（JSON Schema / XSD）準拠チェック。
- **SQLite:** 公式の 22 項目検証（テーブル存在・カラム・主キー・外部キー）。
- バリデーションは `TryFrom` ゲート相当の役割を持ち、失敗は型付きエラー（`thiserror`）で返す。

## 5. CLI 要件

```
ocel convert <input> <output>     # フォーマット変換（拡張子で判定）
ocel validate <input>             # 仕様準拠チェック（結果を人間可読で出力）
```

- 終了コードで成否を表す（validate 失敗は非ゼロ）。

## 6. 受け入れ基準

- [ ] PM4Py `ocel20_example`（JSON / XML / SQLite）を読み込み、3 形式間で round-trip 一致する。
- [ ] Zenodo の Order Management（21K events, SQLite）を読み込み・書き出しできる。
- [ ] 公式スキーマバリデーション（JSON / XML / SQLite 22 項目）が通る。
- [ ] `ocel convert` / `ocel validate` が動作する。
- [ ] `cargo nextest run` / `cargo clippy -- -D warnings` / `cargo fmt --check` / `cargo deny check` が全て通る。
- [ ] README にクイックスタート（5 行程度で JSON を読んで SQLite に書く例）と `examples/` がある。
- [ ] docs.rs でドキュメントがビルドされ、crates.io に公開できる。

## 7. テストデータ

| 用途 | データ | 形式 |
|------|--------|------|
| ユニット / round-trip | PM4Py `ocel20_example`（13 events / 9 objects、全機能網羅） | JSON / XML / SQLite |
| スキーマ検証 | 公式 JSON Schema / XSD / 22 SQL queries | — |
| 中規模 | Order Management（Zenodo, 21K events） | SQLite |
| エッジケース | 最小 OCEL（`{"eventTypes":[],...}`）、同一 qualifier 重複、空属性 | JSON |

## 8. 非機能要件

- 依存クレートは最小限（[architecture.md](architecture.md) の方針）。
- `unsafe` を使わない。`unwrap()` は初期化時のみ。
- semver 遵守。0.1 公開前に公開 API（特に `AttrValue` と動的属性表現）を固める。
