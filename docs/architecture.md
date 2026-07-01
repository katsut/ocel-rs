# ocel-rs アーキテクチャ

- **対象バージョン:** v0.1
- **上位方針:** ワークスペースの [roadmap](https://github.com/katsut/ocel-workspace/blob/main/docs/roadmap.md) と [ADR 0001](https://github.com/katsut/ocel-workspace/blob/main/docs/adr/0001-core-model-and-etl-boundary.md)（コアは OCEL 2.0 ネイティブ / 中間表現は ETL 側）に従う。

---

## 位置付け

`ocel-rs` は **OCEL 2.0 のコアライブラリに徹する**。ETL パイプラインやデータソースコネクタは別リポジトリ（`ocel-etl`, `ocel-etl-backlog`）の責務。ETL の「緩い中間表現（`StagingLog`）」は `ocel-rs` には持ち込まず、`ocel-etl` 側から `TryFrom<StagingLog> for Ocel` で本ライブラリのモデルに変換される。

## クレート構成

```
crates/
├── ocel-core/    # データモデル + I/O + バリデーション（ライブラリ本体）
└── ocel-cli/     # CLI ツール（convert / validate）
```

## モジュール構成（ocel-core）

```
ocel-core/src/
├── lib.rs           # 公開 API の再エクスポート
├── model/           # OCEL 2.0 データモデル（型定義）
│   ├── mod.rs       # Ocel, Event, Object, E2O, O2O
│   ├── attr.rs      # AttrValue（型付き属性値）、動的オブジェクト属性
│   └── builder.rs   # OcelBuilder（fallible な組み立て）
├── io/
│   ├── mod.rs       # Format 判定・共通トレイト（Reader/Writer）
│   ├── json.rs      # OCEL 2.0 JSON
│   ├── sqlite.rs    # OCEL 2.0 SQLite（差別化の核）
│   └── xml.rs       # OCEL 2.0 XML
└── validate/        # 仕様準拠バリデーション（JSON Schema / SQLite 22 項目）
```

## データモデル（OCEL 2.0 ネイティブ）

仕様 `L = (E, O, EA, OA, evtype, time, ..., E2O, O2O)`（arXiv:2403.01975）を Rust の型に写す。

```rust
pub struct Ocel {
    pub event_types:  Vec<EventType>,
    pub object_types: Vec<ObjectType>,
    pub events:       Vec<Event>,   // 各 Event が E2O relationships を内包
    pub objects:      Vec<Object>,  // 各 Object が動的属性 + O2O relationships を内包
}
```

関係（E2O / O2O）は OCEL 2.0 JSON と同様に `Event` / `Object` に埋め込む。フラットな関係ビューは `Ocel::e2o()` / `Ocel::o2o()` アクセサが提供し、SQLite の `event_object` / `object_object` テーブルや列出力（Arrow / PyO3）はこのビューを使う。

**設計上のキモ（ADR 0001）:**

- **属性値の型:** `enum AttrValue { String, Integer, Float, Boolean, Time }`。JSON は全て文字列で来るため read 時に型変換、SQLite は型付き。詳細は [spec-v0.1.md](spec-v0.1.md)。
- **動的オブジェクト属性:** タイムスタンプ付きの値リスト（`Vec<ObjectAttribute>`）として保持し、`Object::attribute_at(name, t)` が forward-fill で任意時点の値を復元する。初期値はエポック `1970-01-01T00:00:00Z`。SQLite の `ocel_changed_field` は I/O 層で扱う。
- **provenance:** 出自（`rule` / `llm` 等）は通常の属性（name/value）として保持できるため、モデルに追加フィールドは不要。

## 「塞がない」ための API 制約

将来（v0.2 の PyO3 / Arrow、大規模ストリーミング）を阻害しないため、core は以下を必ず提供する:

- **`OcelBuilder`（fallible）:** 完成グラフを一括で要求せず、部品から積み上げて最後に `build() -> Result<Ocel, _>` で検証。ETL が `StagingLog` からゲートを通す経路にも使える。
- **イテレータ API:** 全件を `Vec` で抱える以外に、関係を逐次走査できる（`e2o()` / `o2o()` がフラットな関係を返す）。
- **列アクセサ:** `event_columns()` 等、列指向で取り出せる。これがあれば Arrow `RecordBatch` / Polars `DataFrame` / PyO3 が薄いラッパで載る。

## I/O 設計

- 共通トレイト `Reader` / `Writer` を定義し、フォーマットごとに実装。
- **round-trip 一致**を最上位の正しさ基準にする（JSON→メモリ→XML→SQLite→メモリ が公式データと一致）。
- SQLite は型別テーブル `event_{Type}` / `object_{Type}` の動的テーブル名を扱う。テーブル名のサニタイズを徹底。データ投入時は `PRAGMA foreign_keys = OFF`、整合性は validate で別途チェック。

## 依存クレート方針

- シリアライズ: `serde` + `serde_json`
- XML: `quick-xml`
- SQLite: `rusqlite` + `bundled`
- エラー: `thiserror`
- 依存は最小限に。features は必要なものだけ。`cargo deny` を通す。

## テスト戦略

`docs/development-guide.md` と [spec-v0.1.md](spec-v0.1.md) を参照。PM4Py の `ocel20_example`（3 形式）を主フィクスチャに、round-trip とスキーマバリデーションで仕様準拠を担保する。
