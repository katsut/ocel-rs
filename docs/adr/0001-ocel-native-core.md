# ADR 0001: コアモデルは OCEL 2.0 ネイティブ、拡張ポイントを塞がない

- **ステータス:** Accepted
- **日付:** 2026-07-01
- **スコープ:** ocel-rs（このリポジトリ内の決定）
- **関連:** ETL 側の境界は ocel-etl の [ADR 0001: StagingLog と Ocel 境界](https://github.com/katsut/ocel-etl/blob/main/docs/adr/0001-staging-log-and-ocel-boundary.md)

---

## コンテキスト

ocel-rs は OCEL 2.0 のコアライブラリに徹する（ETL・コネクタは別リポジトリ）。in-memory モデルを「OCEL 2.0 ネイティブ」にするか、「ETL に適した柔軟な中間表現」にするかを検討した。North Star は OSS 実用性（crates.io で使われるライブラリ）。

## 決定

1. **`ocel-core` のモデルは OCEL 2.0 ネイティブ**とする。中間表現は core に持ち込まない。利用者が期待する「OCEL 2.0 の構造がそのまま Rust の型で見える」状態を保つ。
2. **完成グラフを前提にしない。** 以下の拡張ポイントを必ず提供し、下流（ETL・将来の PyO3 / Arrow バインディング・大規模ストリーミング）を塞がない:
   - `OcelBuilder`（fallible。部品から積み上げて最後に検証）
   - イテレータ走査 API（`e2o()` / `o2o()` のフラット化）
   - 列アクセサ（`event_columns()` 等）
3. **provenance（出自）は通常の属性（name/value）として表現できる**ため、モデルに専用フィールドを追加しない。

## 根拠

中間表現を core に入れると利用者の学習コストが上がり adoption が落ちる。公開後の semver リスクも増える。中間表現は ETL の関心事であり core の責務ではない。

ETL 側が core にどう変換するか（緩い作業表現 → 厳格な `Ocel` の検証ゲート）と、その技術的根拠（なぜ厳格な OCEL を作業バッファに使うと破綻するか）は **ocel-etl の ADR** に委譲する。core はターゲット型 `ocel_core::Ocel` を「妥当なものしか作れない」形で提供することに集中する。

## 結果

- core の公開 API が素直で学習コストが低い（OSS adoption に有利）。
- 下流が `StagingLog → Ocel` のような変換を被せても core は無傷。
- v0.1 では列アクセサ・イテレータ・Builder の「口」を用意するだけでよく、スコープは増えない。
- round-trip 検証基盤が、将来のマッピング妥当性検証にもそのまま使える。

## 却下した案

- **core に柔軟な中間表現を内蔵** → 利用者の学習コスト増・semver リスク増。中間表現は ETL 層に置く。
