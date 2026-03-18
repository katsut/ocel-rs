# OCEL 2.0 包括リサーチ

> 本ドキュメントはプロセスマイニングの事前知識がない読者を対象に、OCEL 2.0 (Object-Centric Event Log) を体系的に解説する。
> 調査日: 2026-03-18

---

## 目次

1. [背景: プロセスマイニングとは](#1-背景-プロセスマイニングとは)
2. [従来のイベントログ (XES) の問題点](#2-従来のイベントログ-xes-の問題点)
3. [オブジェクト中心プロセスマイニング (OCPM)](#3-オブジェクト中心プロセスマイニング-ocpm)
4. [OCEL 2.0 仕様の詳細](#4-ocel-20-仕様の詳細)
5. [業界採用とエコシステム](#5-業界採用とエコシステム)
6. [メリットと価値提案](#6-メリットと価値提案)
7. [最新トレンド (2024-2026)](#7-最新トレンド-2024-2026)

---

## 1. 背景: プロセスマイニングとは

### 1.1 定義

プロセスマイニングとは、企業の IT システムに記録された「イベントログ」（誰が、いつ、何をしたかの記録）を分析し、業務プロセスの実態を可視化・改善する技術のこと。

**たとえ話**: 病院で患者の動線を追うことを考える。受付 → 待合室 → 診察 → 検査 → 会計 という流れが理想だが、実際にはどうなっているか？ プロセスマイニングは IT システムのログデータから「実際に起きたこと」を自動的にプロセスとして再構築する。人間の記憶やインタビューに頼るのではなく、データに基づいて事実を把握できる。

**ソース**: [Process mining - Wikipedia](https://en.wikipedia.org/wiki/Process_mining), [What is Process Mining? | IBM](https://www.ibm.com/think/topics/process-mining)

### 1.2 3 つのタイプ

プロセスマイニングは大きく 3 つに分類される:

#### (1) プロセスディスカバリ (Process Discovery)

イベントログからプロセスモデルを「発見」する。設計図がない状態から、実際のデータだけでプロセスの全体像を構築する。

**具体例**: ある保険会社が「保険金請求の処理フロー」を可視化したいとする。システムのログには「申請受付」「書類確認」「査定」「支払承認」「振込」といったイベントが記録されている。プロセスディスカバリはこれらのイベントの順序関係を分析し、フローチャートのようなプロセスモデルを自動生成する。結果として「書類確認の前に査定が始まっているケースが 20% ある」といった発見につながる。

#### (2) 適合性検査 (Conformance Checking)

「あるべきプロセス」（設計モデル）と「実際のプロセス」（ログから再構築）を比較し、乖離を検出する。

**具体例**: 銀行の融資審査プロセスでは「信用調査 → 審査委員会 → 承認」の順で進むルールがある。適合性検査により「審査委員会をスキップして承認されたケースが 5 件ある」といった逸脱を自動検出できる。

#### (3) プロセス拡張 (Enhancement)

既存のプロセスモデルにパフォーマンス情報（所要時間、ボトルネック等）を付加し、改善ポイントを特定する。

**具体例**: 注文処理プロセスにおいて「出荷承認」ステップの平均待ち時間が 3 日であることを発見し、ここがボトルネックだと特定する。

**ソース**: [Process Mining: Discovery, Conformance and Enhancement of Business Processes (Springer)](https://link.springer.com/book/10.1007/978-3-642-19345-3)

### 1.3 実世界のユースケース

| 領域 | プロセス | 効果の実例 |
|------|---------|-----------|
| **Order-to-Cash (O2C)** | 受注 → 出荷 → 請求 → 入金 | Kraft Heinz: 延滞支払い 30% 削減 |
| **Purchase-to-Pay (P2P)** | 購買依頼 → 発注 → 検収 → 支払 | Tech Data: P2P サイクルタイム 57% 短縮、請求書自動処理率 95% |
| **ヘルスケア** | 患者受付 → 診察 → 検査 → 退院 | GE Healthcare: フリーキャッシュフロー $1.3B 改善 |
| **IT サービス管理** | インシデント発生 → 対応 → 解決 → クローズ | 解決時間の短縮、SLA 違反の削減 |
| **コンプライアンス** | 各種承認プロセス | 規制違反の自動検出、監査証跡の自動生成 |

**ソース**: [Process Mining Order to Cash: 8 Real-Life Use Cases](https://research.aimultiple.com/process-mining-order-to-cash/), [12+ Real World Process Mining Case Studies | Celonis](https://www.celonis.com/blog/12-case-studies-that-drive-home-the-power-of-process-mining), [The Power of Process Mining in Purchase-to-Pay | Celonis](https://www.celonis.com/blog/the-power-of-process-mining-in-purchase-to-pay)

### 1.4 市場規模と成長

プロセスマイニング市場は急速に成長している。調査機関によって数値にばらつきがあるが、高成長であることは一貫している:

| 調査機関 | 2024年 | 2025年 | 2026年 | CAGR | 将来予測 |
|---------|--------|--------|--------|------|---------|
| Fortune Business Insights | - | $3.66B | $5.45B | 34.4% | $58.18B (2034) |
| Grand View Research | $1.4B | - | - | 59.4% | $21.92B (2030) |
| Research and Markets | - | $2.34B | $3.4B | 45.5% | - |
| Straits Research | $1.25B | $1.96B | - | 56.75% | $71.41B (2033) |

**地域別**: 北米が市場をリード（2025年 $1.23B → 2026年 $1.88B）。アジア太平洋地域が最速成長。

**成長の要因**:
- デジタルトランスフォーメーション (DX) の推進
- AI/自動化との統合
- クラウドベースソリューションの普及
- データドリブンな意思決定への需要増加

**ソース**: [Process Mining Software Market Size | Fortune Business Insights](https://www.fortunebusinessinsights.com/process-mining-software-market-104792), [Process Mining Software Market Size | Grand View Research](https://www.grandviewresearch.com/industry-analysis/process-mining-software-market-report), [6 Process Mining Trends & 20 Stats to Watch for in 2026](https://research.aimultiple.com/process-mining-trends/)

---

## 2. 従来のイベントログ (XES) の問題点

### 2.1 XES とは

XES (eXtensible Event Stream) は、プロセスマイニング用のイベントログ標準フォーマット。2016 年に IEEE 標準 (IEEE 1849-2016) として承認され、2023 年に改訂版 (IEEE 1849-2023) が公開された。

XES の基本構造は XML ベースで、以下の 3 層で構成される:

```
Log（ログ全体）
  └── Trace（1 つのケース / プロセスインスタンス）
        └── Event（1 つのイベント / アクティビティの実行）
```

**核心的な制約**: XES では、すべてのイベントは必ず 1 つの「ケース」（プロセスインスタンス）に属する。これを「1 イベント = 1 ケース」の制約と呼ぶ。

**ソース**: [IEEE SA - IEEE Standard for XES](https://standards.ieee.org/standard/1849-2016.html), [IEEE 1849 - Wikipedia](https://en.wikipedia.org/wiki/IEEE_1849)

### 2.2 「1 イベント = 1 ケース」の限界

#### 具体例: EC サイトの注文管理

あるオンラインストアで、顧客 Alice が 2 つの注文を出し、各注文に複数の商品が含まれているとする:

```
顧客: Alice
  ├── 注文 O1
  │     ├── 商品 I1 (本)
  │     └── 商品 I2 (ペン)
  └── 注文 O2
        └── 商品 I3 (ノート)

配送: 商品 I1, I2, I3 をまとめて 1 つの荷物 S1 で配送
請求: 注文 O1 と O2 をまとめて 1 つの請求書 INV1 で請求
```

ここで XES のイベントログを作ろうとすると、「ケース」を何にするかという問題が発生する:

| ケースの選択 | 問題 |
|-------------|------|
| **注文をケースにする** | 配送イベント「荷物 S1 を出荷」は O1 と O2 のどちらに属する？ → 複製が必要 |
| **商品をケースにする** | 注文レベルのイベント「注文確定」は I1, I2, I3 の 3 つに複製される |
| **配送をケースにする** | 注文や商品ごとのイベントが見えなくなる |
| **顧客をケースにする** | すべての注文が 1 つのケースに入り、プロセスが混ざる |

**どのケース概念を選んでも、現実を正しく表現できない。**

### 2.3 収束 (Convergence) と発散 (Divergence) 問題

XES の単一ケース制約から生じる 2 つの根本的な問題:

#### 収束問題 (Convergence)

**1 つのイベントが複数のケースに関連する場合**、そのイベントが各ケースに複製される。

```
例: 「注文」をケースとした場合

実際: 「荷物 S1 を出荷」は 1 回だけ発生
XES:  注文 O1 にも「荷物 S1 を出荷」、注文 O2 にも「荷物 S1 を出荷」→ 2 回に見える

→ 頻度統計が水増しされる（実際は 1 回なのに 2 回とカウント）
→ 所要時間の分析も歪む
```

#### 発散問題 (Divergence)

**1 つのケースに同じアクティビティの複数インスタンスが含まれる場合**、それらが区別できなくなる。

```
例: 「注文」をケースとした場合

注文 O1 に商品 I1 と I2 がある
「商品をピッキング」が 2 回発生するが、どの商品に対するものか区別できない

→ 同一ケース内のアクティビティが混在し、因果関係が失われる
```

**ソース**: [Object-Centric Process Mining: Dealing with Divergence and Convergence in Event Data (Springer)](https://link.springer.com/chapter/10.1007/978-3-030-30446-1_1), [What is object-centric process mining? | Celonis](https://www.celonis.com/blog/what-is-object-centric-process-mining-ocpm)

### 2.4 フラット化 (Flattening) の弊害

XES で多対多の関係を持つプロセスを扱うには、データを「フラット化」（1 つのケース概念に合わせてデータを変形）する必要がある。これにより:

1. **イベントの複製**: 1 回のイベントが複数ケースにコピーされ、頻度・コスト統計が歪む
2. **情報の欠落**: 選んだケース概念に属さないイベントは分析から漏れる（deficiency）
3. **スパゲッティモデル**: 複製されたイベントにより、プロセスモデルが複雑で読めなくなる
4. **因果関係の喪失**: 異なるオブジェクト間の依存関係が見えなくなる

---

## 3. オブジェクト中心プロセスマイニング (OCPM)

### 3.1 OCPM とは

オブジェクト中心プロセスマイニング (Object-Centric Process Mining; OCPM) は、XES の「1 イベント = 1 ケース」制約を撤廃し、**1 つのイベントが複数のオブジェクトに関連できる**ようにしたプロセスマイニングの進化形。

RWTH Aachen 大学の Wil van der Aalst 教授（「プロセスマイニングの父」と呼ばれる）を中心に開発された。

**ソース**: [Homepage Wil van der Aalst](https://www.vdaalst.com/), [Object-Centric Process Mining: Unraveling the Fabric of Real Processes](https://www.mdpi.com/2227-7390/11/12/2691)

### 3.2 従来手法との違い

| 特徴 | 従来 (XES ベース) | OCPM (OCEL ベース) |
|------|------------------|-------------------|
| **ケース概念** | 1 イベント = 1 ケース | ケース概念なし。イベントは複数オブジェクトに関連 |
| **データモデル** | フラットなトレース | オブジェクトのグラフ構造 |
| **複数視点** | ケースごとに別のログが必要 | 1 つのログで全視点をカバー |
| **オブジェクト間関係** | 表現不可 | 明示的に定義可能 |
| **収束/発散** | 発生する | 発生しない |
| **データ抽出** | ケースごとに別途 ETL | 1 回の ETL で完了 |
| **プロセスモデル** | スパゲッティになりやすい | クリーンで直感的 |

### 3.3 OCPM の基本概念

#### オブジェクト (Object)

プロセスに関与する「もの」。それぞれが一意の識別子と型を持つ。

例: 注文 O1、商品 I1、配送 S1、請求書 INV1、顧客 C1

#### オブジェクトタイプ (Object Type)

オブジェクトの分類。

例: Order, Item, Shipment, Invoice, Customer

#### イベント (Event)

特定の時刻に発生したアクティビティの実行。**複数のオブジェクトに関連できる**のが最大のポイント。

```
イベント: 「荷物を出荷」
  時刻: 2026-03-15 10:00
  関連オブジェクト:
    - 配送 S1 (ロール: 出荷物)
    - 商品 I1 (ロール: 出荷対象)
    - 商品 I2 (ロール: 出荷対象)
    - 商品 I3 (ロール: 出荷対象)
```

#### オブジェクト間関係 (Object-to-Object Relationship)

オブジェクト同士の関連を明示的に定義できる。

```
注文 O1 ──("contains")──→ 商品 I1
注文 O1 ──("contains")──→ 商品 I2
請求書 INV1 ──("covers")──→ 注文 O1
請求書 INV1 ──("covers")──→ 注文 O2
```

**ソース**: [Overview of object-centric process mining | Microsoft Learn](https://learn.microsoft.com/en-us/power-automate/object-centric-overview), [Difference between OCPM and Process Mining | mpmX](https://mpmx.com/blog/difference-ocpm-process-mining)

### 3.4 進化の歴史: XES → OCEL 1.0 → OCEL 2.0

| 年 | マイルストーン | 内容 |
|----|-------------|------|
| 2016 | XES (IEEE 1849-2016) | プロセスマイニング初の IEEE 標準。1 イベント = 1 ケースのフラット構造 |
| 2020 | OCEL 1.0 リリース | オブジェクト中心イベントログの初版。1 イベント → 複数オブジェクト対応 |
| 2020-2022 | 研究の活発化 | OCEL 1.0 ベースの OCPM 技術が多数開発される |
| 2023 | OCEL 2.0 リリース | オブジェクト間関係、修飾子、動的属性を追加した拡張版 |
| 2023 | XES 改訂 (IEEE 1849-2023) | XES の改訂版公開。ただし根本的な制約は変わらず |
| 2024 | OCEL 2.0 仕様論文 (arXiv) | 正式な仕様ドキュメントが公開 (arXiv:2403.01975) |
| 2025 | 産業界での本格採用 | Celonis が OCPM を中核機能化、Microsoft がプレビューリリース |

### 3.5 OCEL 1.0 → OCEL 2.0 の変更点

| 機能 | OCEL 1.0 | OCEL 2.0 |
|------|---------|---------|
| イベント → 複数オブジェクト | あり | あり |
| オブジェクト間関係 (O2O) | **なし** | あり（修飾子付き） |
| E2O 修飾子 (Qualifier) | **なし** | あり |
| 動的オブジェクト属性 | **なし**（静的のみ） | あり（タイムスタンプ付きで変化を追跡） |
| SQLite 形式 | **なし** | あり |
| 交換フォーマット | JSON, XML | JSON, XML, **SQLite** |

OCEL 1.0 は「1 イベントが複数オブジェクトに関連できる」という最低限の機能のみだった。OCEL 2.0 はそこに「なぜ関連するのか（修飾子）」「オブジェクト同士の関係」「属性の時間変化」を加え、より表現力の高い標準となった。

**ソース**: [OCEL (Object-Centric Event Log) 2.0 Specification](https://arxiv.org/html/2403.01975v1), [OCEL 2.0: Enabling Object-Centric Process Mining | LinkedIn](https://www.linkedin.com/pulse/ocel-20-enabling-object-centric-process-mining-wil-van-der-aalst-yafie)

---

## 4. OCEL 2.0 仕様の詳細

### 4.1 形式モデル（13 タプル）

OCEL 2.0 のイベントログ L は、以下の 13 要素のタプルとして数学的に定義される:

```
L = (E, O, EA, OA, evtype, time, objtype, eatype, oatype, eaval, oaval, E2O, O2O)
```

| 要素 | 型 | 説明 | 例 |
|------|---|------|-----|
| **E** | 集合 | イベントの集合 | {e1, e2, e3, ...} |
| **O** | 集合 | オブジェクトの集合 | {PO1, INV1, ...} |
| **EA** | 集合 | イベント属性名の集合 | {pr_creator, approval_status} |
| **OA** | 集合 | オブジェクト属性名の集合 | {po_product, po_quantity} |
| **evtype** | 関数 E → Type | イベント → イベントタイプ | evtype(e1) = "Create PR" |
| **time** | 関数 E → Time | イベント → タイムスタンプ | time(e1) = 2022-01-09T15:00 |
| **objtype** | 関数 O → Type | オブジェクト → オブジェクトタイプ | objtype(PO1) = "PurchaseOrder" |
| **eatype** | 関数 EA → Type | イベント属性 → 属するイベントタイプ | eatype(pr_creator) = "Create PR" |
| **oatype** | 関数 OA → Type | オブジェクト属性 → 属するオブジェクトタイプ | oatype(po_quantity) = "PurchaseOrder" |
| **eaval** | 部分関数 | (イベント, 属性) → 値 | eaval(e1, pr_creator) = "Alice" |
| **oaval** | 部分関数 | (オブジェクト, 属性, 時刻) → 値 | oaval(PO1, po_quantity, t1) = 500 |
| **E2O** | 関係 | 修飾子付きイベント-オブジェクト関係 | (e1, "created", PO1) |
| **O2O** | 関係 | 修飾子付きオブジェクト-オブジェクト関係 | (PO1, "from PR", PR1) |

**ポイント**:
- `eaval` は通常の部分関数（時間の概念なし）。イベント属性はイベント発生時の値のみ
- `oaval` は時間パラメータを持つ部分関数。オブジェクトの属性は時間とともに変化しうる
- `E2O` と `O2O` は修飾子 (qualifier) という文字列で関係の意味を記述する

**ソース**: [OCEL 2.0 Specification (arXiv:2403.01975)](https://arxiv.org/html/2403.01975v1), [Specification - OCEL 2.0](https://www.ocel-standard.org/specification/overview/)

### 4.2 6 つのコアエンティティとその関係

```
Event Types ←── defines attributes ──→ Event Attributes
    ↑ type of
  Events ──── E2O (qualifier) ────→ Objects
                                       ↑ type of
                                   Object Types ←── defines attributes ──→ Object Attributes
                                       ↑
                                   Objects ──── O2O (qualifier) ────→ Objects
```

#### (1) Event Types（イベントタイプ）

アクティビティの種類を定義。各イベントタイプは独自の属性セットを持つ。

例:
```
イベントタイプ: "Create Purchase Requisition"
  属性: pr_creator (string), pr_type (string)

イベントタイプ: "Approve Purchase Order"
  属性: approval_user (string), approval_result (string)
```

#### (2) Object Types（オブジェクトタイプ）

オブジェクトの種類を定義。各オブジェクトタイプは独自の属性セットを持つ。

例:
```
オブジェクトタイプ: "PurchaseOrder"
  属性: po_product (string), po_quantity (integer), po_price (float)

オブジェクトタイプ: "Invoice"
  属性: inv_amount (float), is_blocked (string)
```

#### (3) Events（イベント）

タイムスタンプ付きの具体的なアクティビティ実行。

例:
```
イベント e1: type="Create PR", time=2022-01-09T15:00, pr_creator="Alice"
イベント e2: type="Approve PO", time=2022-01-10T09:00, approval_user="Bob"
```

#### (4) Objects（オブジェクト）

プロセスに関与するエンティティのインスタンス。動的属性を持つ。

例:
```
オブジェクト PO1: type="PurchaseOrder"
  po_quantity = 500 (時刻: 初期値)
  po_quantity = 600 (時刻: 2022-01-13T12:00)  ← 数量が変更された
```

#### (5) E2O（イベント-オブジェクト関係）

イベントとオブジェクトの関係。修飾子 (qualifier) でその関係の意味を記述する。

例:
```
(e1, "Regular placement of PR", PR1)
(e1, "Created order from PR", PO1)
(e2, "Approved", PO1)
(e2, "Approver", User_Bob)
```

#### (6) O2O（オブジェクト-オブジェクト関係）

オブジェクト間の関係。修飾子で関係の種類を記述する。

例:
```
(PO1, "PO from PR", PR1)         ← 購買発注は購買依頼から作成された
(INV1, "Invoice from PO", PO1)   ← 請求書は購買発注に基づく
(PO1, "Maverick buying", null)    ← マーベリック購買（特殊な関係）
```

### 4.3 Qualifier（修飾子）の詳細

修飾子はOCEL 2.0 の重要な概念で、**関係の「なぜ」や「どのような役割で」を説明する文字列**。

#### E2O の修飾子

イベントに対してオブジェクトがどのような役割を果たすかを記述する。

```
イベント「商品を出荷」に対して:
  - 配送 S1 → qualifier: "shipped as"      (出荷物として)
  - 商品 I1 → qualifier: "item shipped"     (出荷された商品)
  - 商品 I2 → qualifier: "item shipped"     (出荷された商品)
  - 倉庫 W1 → qualifier: "shipped from"     (出荷元として)
```

#### O2O の修飾子

オブジェクト間の関係の性質を記述する。

```
注文 O1 → 商品 I1: qualifier: "contains"
注文 O1 → 商品 I2: qualifier: "contains"
請求書 INV1 → 注文 O1: qualifier: "covers"
```

#### なぜ修飾子が重要か

修飾子なしでは「イベント e1 はオブジェクト PO1 に関連する」としか分からない。修飾子があることで「イベント e1 は PO1 を"作成した"」のか「PO1 を"承認した"」のか区別できる。同じイベント-オブジェクトペアが異なる修飾子で複数回関連することも可能。

### 4.4 動的オブジェクト属性

OCEL 2.0 の大きな特徴の一つが**タイムスタンプ付きのオブジェクト属性**。オブジェクトの状態が時間とともにどう変化したかを追跡できる。

#### 仕組み

```
オブジェクト PO1 (PurchaseOrder) の属性履歴:

時刻                    | po_product | po_quantity | ocel_changed_field
------------------------|-----------|-------------|-------------------
1970-01-01T00:00:00     | "Cows"    | 500         | NULL (初期値)
2022-01-13T12:00:00     | "Cows"    | 600         | "po_quantity"
2022-01-15T09:00:00     | "Sheep"   | 600         | "po_product"
```

- 初期値は `1970-01-01T00:00:00 UTC` のタイムスタンプで記録
- 変更時は新しいタイムスタンプ + 新しい値 + `ocel_changed_field`（変更されたフィールド名）
- 任意の時点の値は forward-fill で復元可能（最も近い過去の記録を適用）

#### なぜ動的属性が重要か

現実のプロセスでは、オブジェクトの属性は頻繁に変化する:
- 注文のステータス: 「未確認」→「確認済」→「出荷済」→「完了」
- 請求書の金額: 値引き適用で変更
- タスクの担当者: 再割り当て

従来の OCEL 1.0 では属性は静的で、こうした変化を追跡できなかった。

### 4.5 属性のデータ型

OCEL 2.0 で定義されるデータ型:

| 型 | 説明 | 例 |
|----|------|-----|
| `string` | 文字列 | "Alice", "Approved" |
| `integer` | 整数 | 500, -1 |
| `float` | 浮動小数点数 | 99.99 |
| `boolean` | 真偽値 | true, false |
| `time` | 日時 (ISO 8601) | "2022-01-09T15:00:00Z" |

### 4.6 ストレージフォーマット: SQLite, JSON, XML

OCEL 2.0 は 3 つの交換フォーマットを定義している。

#### SQLite（リレーショナルデータベース）

**構造**: 6 つの型非依存テーブル + 型ごとの個別テーブル

型非依存テーブル:

| テーブル | 説明 |
|---------|------|
| `event_corr_type` | イベントタイプ定義 (PK: ocel_type) |
| `object_corr_type` | オブジェクトタイプ定義 (PK: ocel_type) |
| `event` | 全イベント (PK: ocel_id, FK: ocel_type → event_corr_type) |
| `object` | 全オブジェクト (PK: ocel_id, FK: ocel_type → object_corr_type) |
| `event_object` | E2O 関係 (FK: event, object; qualifier 付き) |
| `object_object` | O2O 関係 (FK: source, target; qualifier 付き) |

型別テーブル:

| パターン | 説明 |
|---------|------|
| `event_{TypeName}` | イベントタイプごとの属性テーブル (ocel_id, ocel_time, 型固有属性) |
| `object_{TypeName}` | オブジェクトタイプごとの属性テーブル (ocel_id, ocel_time, 型固有属性, ocel_changed_field) |

**メリット**: SQL クエリで直接分析可能、大規模データに向く、外部キー制約で整合性を保証
**デメリット**: ファイル共有時にバイナリ形式、型追加にテーブル作成が必要

**ソース**: [SQLite - OCEL 2.0](https://ocel-standard.org/specification/formats/sqlite/)

#### JSON

```json
{
  "objectTypes": [{"name": "Order", "attributes": [{"name": "total", "type": "float"}]}],
  "eventTypes": [{"name": "Place Order", "attributes": [{"name": "channel", "type": "string"}]}],
  "objects": [{
    "id": "O1", "type": "Order",
    "attributes": [{"name": "total", "value": "99.99", "time": "1970-01-01T00:00:00Z"}],
    "relationships": [{"objectId": "I1", "qualifier": "contains"}]
  }],
  "events": [{
    "id": "e1", "type": "Place Order", "time": "2022-01-09T15:00:00Z",
    "attributes": [{"name": "channel", "value": "web"}],
    "relationships": [{"objectId": "O1", "qualifier": "created"}]
  }]
}
```

**メリット**: 人間が読みやすい、Web API との親和性、多くのツールで扱いやすい
**デメリット**: 大規模データではファイルサイズが大きくなる、パース時にメモリを大量消費

**ソース**: [JSON - OCEL 2.0](https://www.ocel-standard.org/specification/formats/json/)

#### XML

```xml
<log>
  <object-types>
    <object-type name="Order">
      <attributes><attribute name="total" type="float"/></attributes>
    </object-type>
  </object-types>
  <event-types>
    <event-type name="Place Order">
      <attributes><attribute name="channel" type="string"/></attributes>
    </event-type>
  </event-types>
  <objects>
    <object id="O1" type="Order">
      <attributes><attribute name="total" time="1970-01-01T00:00:00Z">99.99</attribute></attributes>
      <objects><relationship object-id="I1" qualifier="contains"/></objects>
    </object>
  </objects>
  <events>
    <event id="e1" type="Place Order" time="2022-01-09T15:00:00Z">
      <attributes><attribute name="channel">web</attribute></attributes>
      <objects><relationship object-id="O1" qualifier="created"/></objects>
    </event>
  </events>
</log>
```

**メリット**: XSD によるスキーマ検証、既存の XML ツールチェーンとの互換性
**デメリット**: 冗長、JSON より読みにくい、大規模データではパフォーマンス劣化

**ソース**: [XML - OCEL 2.0](https://ocel-standard.org/specification/formats/xml/)

#### ファイル拡張子

| フォーマット | 標準拡張子 | 推奨拡張子 |
|------------|----------|----------|
| JSON | .json | .jsonocel |
| XML | .xml | .xmlocel |
| SQLite | .sqlite / .sqlite3 | .sqlite |

#### フォーマット選択の指針

| 用途 | 推奨フォーマット | 理由 |
|------|----------------|------|
| 大規模データの分析 | SQLite | SQL クエリ、インデックス、メモリ効率 |
| Web アプリケーション / API | JSON | パース容易、軽量 |
| 既存システムとの統合 | XML | スキーマ検証、エンタープライズ対応 |
| 小規模データの共有 | JSON or XML | テキストベースで差分確認可能 |
| 学術研究 / ベンチマーク | SQLite | 再現性、クエリの柔軟性 |

---

## 5. 業界採用とエコシステム

### 5.1 主要企業の採用状況

#### Celonis

- プロセスマイニング市場のリーダー（Gartner Magic Quadrant 2025 でリーダーに選出）
- 2022 年から OCPM を本格サポート
- 2025 年の Celosphere で新 OCPM 機能を発表: Performance Spectrum、Instance Explorer、Object-Centric Performance アプリ
- 「OCPM はもはや新しいものではない。全員が受け入れ、これが進むべき道だと認識している」(2025)
- MCP (Model Context Protocol) サーバーを世界初のプロセスインテリジェンス向けに構築

**ソース**: [Exploring Celonis' object-centric process mining approach | SiliconANGLE](https://siliconangle.com/2025/11/05/exploring-object-centric-process-mining-celosphere/), [2025 Gartner Magic Quadrant for Process Mining Platforms | Celonis](https://www.celonis.com/insights/reports/gartner-magic-quadrant)

#### Microsoft

- Power Automate にオブジェクト中心プロセスマイニングをプレビュー機能として搭載
- OCEL フォーマットからのインポートをサポート
- まだ正式リリース (GA) ではなくプレビュー段階

**ソース**: [Overview of object-centric process mining | Microsoft Learn](https://learn.microsoft.com/en-us/power-automate/object-centric-overview)

#### SAP Signavio

- Gartner Magic Quadrant 2025 で 3 年連続リーダー
- SAP ERP との深い統合

**ソース**: [SAP Signavio: A Leader in 2025 Gartner Magic Quadrant](https://www.signavio.com/downloads/analyst-reports/2025-gartner-magic-quadrant-process-mining/)

#### その他の主要ベンダー (Gartner 2025)

| ベンダー | Gartner 2025 の位置付け |
|---------|----------------------|
| Celonis | リーダー |
| SAP Signavio | リーダー |
| UiPath | リーダー |
| ARIS | リーダー |
| ServiceNow | チャレンジャー |
| QPR Software | ビジョナリー |

### 5.2 学術界の採用

#### 主要研究グループ

| 機関 | リーダー | 貢献 |
|------|---------|------|
| **RWTH Aachen 大学 (PADS グループ)** | Wil van der Aalst 教授 | プロセスマイニングの創始。OCEL 標準の策定 |
| **Fraunhofer FIT** | Wil van der Aalst | プロセスマイニンググループ（産業応用） |
| **Eindhoven 工科大学** | (元 van der Aalst) | プロセスマイニング研究の発祥地 (1999-) |

#### 主要学会・カンファレンス

| カンファレンス | 説明 |
|-------------|------|
| **ICPM** (International Conference on Process Mining) | プロセスマイニング専門の国際会議。2019 年に Aachen で初回開催 |
| **BPM** (Business Process Management) | ビジネスプロセス管理の国際会議。プロセスマイニングのトラックあり |
| **BPI Challenge** | 実データを使ったプロセスマイニングコンペティション（2011年-） |

**ソース**: [Wil van der Aalst - Wikipedia](https://en.wikipedia.org/wiki/Wil_van_der_Aalst)

### 5.3 ツールとライブラリ

#### Python

| ライブラリ | 説明 | OCEL 2.0 対応 |
|-----------|------|--------------|
| **PM4Py** | デファクトスタンダードのプロセスマイニングライブラリ | 完全対応 (JSON, XML, SQLite) |
| **ocpa** | オブジェクト中心プロセス分析専門ライブラリ | 対応 |
| **ocel-support** | OCEL 公式のインポート/エクスポートライブラリ | 基本対応 |

**ソース**: [pm4py - OCEL 2.0](https://www.ocel-standard.org/tool-support/libraries/pm4py/), [GitHub - ocpm/ocpa](https://github.com/ocpm/ocpa)

#### Rust

| ライブラリ | 説明 | OCEL 2.0 対応 |
|-----------|------|--------------|
| **Rust4PM** (process_mining crate) | パフォーマンス重視のプロセスマイニングライブラリ。XES / OCEL 2.0 パーシングが他ツールより大幅に高速 | 対応 |
| **pmrs** | OCEL 標準サポートの Rust ライブラリ + CLI | 部分対応 |

**ソース**: [Rust4PM: A Versatile Process Mining Library](https://ceur-ws.org/Vol-3758/paper-16.pdf), [GitHub - DerAndereJohannes/pmrs](https://github.com/DerAndereJohannes/pmrs)

#### JavaScript

| ライブラリ | 説明 |
|-----------|------|
| **OCPM (JS)** | ブラウザベースの OCPM ツール。PM4JS ライブラリ上に構築 |

**ソース**: [OCPM (JS) - OCEL 2.0](https://ocel-standard.org/tool-support/software/ocpm/)

### 5.4 利用可能なデータセット

#### OCEL 2.0 公式 (Zenodo)

| データセット | 内容 | フォーマット |
|------------|------|------------|
| Order Management | CPN-Tools で生成した注文管理プロセス | SQLite |
| Procure-to-Payment (P2P) | SAP トランザクションベースの購買プロセス | SQLite |
| Container Logistics | コンテナ物流プロセス | SQLite |
| Simulated Logs (4 種) | O2C, P2P, Hiring, Hospital のシミュレーション | OCEL 2.0 |

**ソース**: [Simulated OCEL 2.0 Logs | Zenodo](https://zenodo.org/records/13879980), [P2P OCEL 2.0 Log | Zenodo](https://zenodo.org/records/8412920), [Order Management OCEL 2.0 | Zenodo](https://zenodo.org/records/8428112)

#### BPI Challenge（XES → OCEL 変換済みあり）

| データセット | 内容 |
|------------|------|
| BPI Challenge 2015 | オランダの自治体の許可申請プロセス（OCEL 変換済み） |
| BPI Challenge 2017 | 金融機関のローン申請プロセス（Application と Offer の 2 オブジェクトタイプ） |

**ソース**: [BPI Challenge 2015 (OCEL)](https://data.4tu.nl/datasets/110d2fcf-b5e1-494a-a588-896a0a21e60a)

### 5.5 エコシステムの現状と課題

#### 現状の制限

1. **プロセススコープの定義が不明確**: OCEL フォーマットにはプロセスの境界を明示的に定義する仕組みがない。分析範囲は組織やアナリストの主観に依存する
2. **複雑性のトレードオフ**: OCEL の表現力が増すほど、分析の複雑性も増す
3. **属性変更の曖昧性**: タイムスタンプベースの動的属性では、どのイベントがどの変更を引き起こしたかが曖昧になりうる
4. **ツールの成熟度**: PM4Py 以外のツールはまだ発展途上。特に Rust / JavaScript のエコシステムは薄い
5. **ETL が最大のボトルネック**: OCEL 2.0 形式へのデータ変換は「プロセスマイニングの最初かつ最もコストの高いステップ」

**ソース**: [Object-Centric Process Mining: Unraveling the Fabric of Real Processes](https://www.mdpi.com/2227-7390/11/12/2691), [Enriching Object-Centric Event Data](https://arxiv.org/pdf/2508.18830)

---

## 6. メリットと価値提案

### 6.1 OCEL 2.0 を XES の代わりに使うべき理由

#### (1) 現実のプロセスをそのまま表現できる

XES ではフラット化が必要だが、OCEL 2.0 では注文、商品、配送、請求書といった複数のオブジェクトの関係をそのまま記録できる。

**例**: 購買プロセスで「購買依頼 → 発注 → 請求書 → 支払い」の関係を、無理なデータ変形なしに表現可能。

#### (2) データ抽出が 1 回で済む

XES: 「注文視点のログ」「商品視点のログ」「配送視点のログ」を別々に抽出する必要がある。
OCEL 2.0: 1 つのログに全視点が含まれる。分析時にフィルタリングで視点を切り替えられる。

#### (3) 収束/発散問題が解消される

統計データ（頻度、所要時間）が歪まない。正確なパフォーマンス分析が可能になる。

#### (4) プロセス間のインタラクションが見える

XES ではプロセスごとに個別に分析するしかなかったが、OCEL 2.0 ではプロセスの「接続点」（あるプロセスの出力が別のプロセスの入力になるポイント）を分析できる。

#### (5) よりクリーンな可視化

XES ベースのプロセスモデルは「スパゲッティ図」になりやすい。OCEL 2.0 ベースのオブジェクト中心 DFG (Directly-Follows Graph) はオブジェクトタイプごとに整理された読みやすい図になる。

### 6.2 パフォーマンスへの影響

- **データサイズ**: OCEL 2.0 はフラット化によるイベント複製がないため、多くの場合 XES より小さいデータサイズになる
- **クエリ効率**: SQLite フォーマットにより、SQL クエリでの高速フィルタリング・集計が可能
- **Rust4PM のベンチマーク**: Rust 実装の OCEL 2.0 パーサーは Python (PM4Py) より大幅に高速

### 6.3 OCEL 2.0 を使うべきでない場合

| 状況 | 理由 |
|------|------|
| **単一オブジェクトタイプの単純なプロセス** | 従来の XES / CSV で十分。OCEL のオーバーヘッドが不要 |
| **ツール互換性が最優先** | 多くの商用ツールがまだ XES を主要サポート |
| **チームの学習コストが許容できない** | OCPM の概念理解に一定の教育コストが必要 |
| **リアルタイムストリーミングが主目的** | OCEL 2.0 はバッチ処理向けに設計されている（ストリーミング拡張は研究段階） |

---

## 7. 最新トレンド (2024-2026)

### 7.1 プロセスマイニングの産業トレンド

#### (1) プロセスマイニング → プロセスインテリジェンスへの進化

2025 年以降、プロセスマイニングは「プロセスインテリジェンス」へと進化している。単なる分析ツールから、AI のための戦略的コアテクノロジーへ:
- データプラットフォームとのネイティブ統合
- GenAI / ML による分析の自動化
- ビジネスコンテキストを AI に提供する基盤

**ソース**: [2026 process mining trends](https://www.processexcellencenetwork.com/process-mining/articles/6-trends-shaping-process-mining-in-2026)

#### (2) OCPM の主流化

OCPM は 2025-2026 年にかけて急速に主流化:
- Celonis が OCPM をプラットフォームの中核に据える
- Microsoft Power Automate がプレビュー提供
- 企業のセンター・オブ・エクセレンスがレガシーモデルを OCPM に移行中

#### (3) リアルタイム監視への移行

定期的な監査 / 分析から、継続的なリアルタイム監視へ:
- 予測分析、異常検知、プロアクティブな介入
- ストリーミングプロセスマイニング: リアルタイムのイベントストリームをインクリメンタルに処理

**ソース**: [6 Process Mining Trends & 20 Stats to Watch for in 2026](https://research.aimultiple.com/process-mining-trends/)

### 7.2 AI / LLM との融合

#### エージェンティック AI とプロセスマイニング

2025-2026 年の最大トレンドの一つが「エージェンティックプロセスマイニング」:

- **85%** の企業が 3 年以内に「エージェンティック企業」になることを目指している
- **90%** がマルチエージェントシステムを使用中または検討中
- **89%** が「AI は業務コンテキストがなければ ROI を出せない」と回答

プロセスマイニングは AI エージェントに「業務のルール、KPI、ベンチマーク」というコンテキストを提供する基盤となる。

**具体例**: Celonis は MCP (Model Context Protocol) サーバーを構築し、AI エージェントにリアルタイムの業務コンテキストを供給。タスクの自動化から「アウトカム・オーケストレーション」（人・AI・システムを横断してワークフロー全体を調整）へとシフトが進んでいる。

**ソース**: [AI agents, automation, process mining starting to converge | Constellation Research](https://www.constellationr.com/blog-news/insights/ai-agents-automation-process-mining-starting-converge), [Celonis wants to be the roadmap for agentic AI](https://fastforward.boldstart.vc/celonis-wants-to-be-the-roadmap-to-rebuild-workflows-for-agentic-ai/)

### 7.3 ストリーミング / リアルタイムプロセスマイニング

- ストリーミングプロセスマイニングは新興分野で、ストリームデータマイニング、プロセス発見、適合性検査、予測分析を横断する
- イベントストリーム処理ソフトウェア市場は 2026 年に $3.25B、2035 年に $17.5B に成長見込み
- 主なトレンド: イベント駆動アーキテクチャ (64%)、クラウドネイティブストリーム処理 (61%)、統合データパイプライン (56%)、リアルタイム異常検知 (48%)、AI アシスト分析 (42%)

**ソース**: [Streaming Management and Analytics for Process Mining](https://sma4pm.github.io/2025/), [AVOCADO: The Streaming Process Mining Challenge](https://arxiv.org/html/2510.17089)

### 7.4 デジタルツインとプロセスマイニング

- Gartner 予測: 2026 年までにグローバル企業の **25%** がプロセスマイニングプラットフォームを導入し、業務のデジタルツイン (DTO: Digital Twin of Organization) を構築
- DTO はプロセスの発見・最適化・シミュレーションを行い、what-if 分析で最適な意思決定を支援
- LLM / 基盤モデルの登場により、デジタルツインの構築とシミュレーション自動化が加速

**ソース**: [The hype surrounding Digital Twins | mpmX](https://mpmx.com/blog/hype-digital-twin), [Apromore - Build Digital Twins with Process Mining](https://apromore.com/build-a-digital-twin-model-with-process-mining)

### 7.5 規制 / コンプライアンスドライバー

プロセスマイニングは監査・コンプライアンスの強力なツールとして認知されている:

- **自動監査**: 手動監査からデータ駆動の自動監査へ移行。プロセス全体を網羅的に検証可能
- **リアルタイム逸脱検知**: 標準プロセスからの逸脱をリアルタイムで検出・アラート
- **GDPR 対応**: プロセスマイニングを活用した GDPR コンプライアンスの継続的監視
  - 2026 年: プロアクティブなプライバシーエンジニアリングへの進化が求められる
  - ダークパターン、AI 処理、同意操作に対する規制強化
- **監査証跡の自動維持**: 詳細なログと監査証跡の自動生成・保持

**ソース**: [How to Use Process Mining for Auditing and Compliance | ProcessMaker](https://www.processmaker.com/blog/how-to-use-process-mining-for-auditing-and-compliance/), [Process Mining for Compliance and Audit | mindzie](https://mindzie.com/process-mining-compliance-and-audit/)

---

## 付録: 用語集

| 用語 | 説明 |
|------|------|
| **イベントログ (Event Log)** | システムに記録された「誰が、いつ、何をしたか」の履歴データ |
| **ケース (Case)** | 従来のプロセスマイニングにおけるプロセスインスタンス。1 つの注文の処理など |
| **トレース (Trace)** | 1 つのケースに属するイベントの時系列 |
| **XES** | eXtensible Event Stream。従来のイベントログ標準 (IEEE 1849) |
| **OCEL** | Object-Centric Event Log。オブジェクト中心イベントログ |
| **OCPM** | Object-Centric Process Mining。オブジェクト中心プロセスマイニング |
| **DFG** | Directly-Follows Graph。イベント間の直接的な順序関係を表すグラフ |
| **フラット化 (Flattening)** | 多対多の関係を持つデータを単一ケースの XES 形式に変形すること |
| **収束 (Convergence)** | 1 つのイベントが複数ケースに複製される問題 |
| **発散 (Divergence)** | 1 つのケース内で同じアクティビティが複数回出現し区別できない問題 |
| **修飾子 (Qualifier)** | OCEL 2.0 における関係の意味を記述する文字列ラベル |
| **E2O** | Event-to-Object。イベントとオブジェクトの関係 |
| **O2O** | Object-to-Object。オブジェクト間の関係 |
| **動的属性** | 時間とともに値が変化するオブジェクトの属性 |
| **DTO** | Digital Twin of Organization。組織のデジタルツイン |

---

## 参考文献・ソース一覧

### 仕様・標準
- [OCEL 2.0 公式サイト](https://www.ocel-standard.org/)
- [OCEL 2.0 仕様論文 (arXiv:2403.01975)](https://arxiv.org/html/2403.01975v1)
- [OCEL 2.0 SQLite フォーマット](https://ocel-standard.org/specification/formats/sqlite/)
- [OCEL 2.0 JSON フォーマット](https://www.ocel-standard.org/specification/formats/json/)
- [OCEL 2.0 XML フォーマット](https://ocel-standard.org/specification/formats/xml/)
- [IEEE 1849-2016 (XES 標準)](https://standards.ieee.org/standard/1849-2016.html)

### 学術・概念
- [Process mining - Wikipedia](https://en.wikipedia.org/wiki/Process_mining)
- [What is Process Mining? | IBM](https://www.ibm.com/think/topics/process-mining)
- [Object-Centric Process Mining: Dealing with Divergence and Convergence (Springer)](https://link.springer.com/chapter/10.1007/978-3-030-30446-1_1)
- [Object-Centric Process Mining: Unraveling the Fabric of Real Processes (MDPI)](https://www.mdpi.com/2227-7390/11/12/2691)
- [Wil van der Aalst Homepage](https://www.vdaalst.com/)

### 産業・市場
- [Celonis OCPM Overview](https://www.celonis.com/blog/what-is-object-centric-process-mining-ocpm)
- [Microsoft Power Automate OCPM](https://learn.microsoft.com/en-us/power-automate/object-centric-overview)
- [SAP Signavio Gartner Leader 2025](https://www.signavio.com/downloads/analyst-reports/2025-gartner-magic-quadrant-process-mining/)
- [Process Mining Market Size | Fortune Business Insights](https://www.fortunebusinessinsights.com/process-mining-software-market-104792)
- [Process Mining Market | Grand View Research](https://www.grandviewresearch.com/industry-analysis/process-mining-software-market-report)

### トレンド
- [6 Process Mining Trends & 20 Stats to Watch for in 2026](https://research.aimultiple.com/process-mining-trends/)
- [2026 process mining trends | PEX Network](https://www.processexcellencenetwork.com/process-mining/articles/6-trends-shaping-process-mining-in-2026)
- [AI agents, automation, process mining converging | Constellation Research](https://www.constellationr.com/blog-news/insights/ai-agents-automation-process-mining-starting-converge)
- [Celonis agentic AI roadmap](https://fastforward.boldstart.vc/celonis-wants-to-be-the-roadmap-to-rebuild-workflows-for-agentic-ai/)

### ツール・ライブラリ
- [PM4Py - OCEL 2.0 Support](https://www.ocel-standard.org/tool-support/libraries/pm4py/)
- [ocpa - GitHub](https://github.com/ocpm/ocpa)
- [Rust4PM (CEUR-WS)](https://ceur-ws.org/Vol-3758/paper-16.pdf)
- [pmrs - GitHub](https://github.com/DerAndereJohannes/pmrs)

### データセット
- [Simulated OCEL 2.0 Logs | Zenodo](https://zenodo.org/records/13879980)
- [P2P OCEL 2.0 | Zenodo](https://zenodo.org/records/8412920)
- [Order Management OCEL 2.0 | Zenodo](https://zenodo.org/records/8428112)
- [Container Logistics OCEL | Zenodo](https://zenodo.org/records/8428084)
- [BPI Challenge 2015 (OCEL)](https://data.4tu.nl/datasets/110d2fcf-b5e1-494a-a588-896a0a21e60a)

### ユースケース
- [Process Mining Order to Cash | AIMultiple](https://research.aimultiple.com/process-mining-order-to-cash/)
- [Top 50 Process Mining Use Cases | AIMultiple](https://research.aimultiple.com/process-mining-use-cases/)
- [12+ Real World Case Studies | Celonis](https://www.celonis.com/blog/12-case-studies-that-drive-home-the-power-of-process-mining)
- [Process Mining for Compliance | mindzie](https://mindzie.com/process-mining-compliance-and-audit/)
