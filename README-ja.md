# templatia

構造体をシンプルなテキストテンプレートへ変換し、文字列から再構築するためのRustライブラリです。構成は以下の2クレートです。

- templatia: コアのトレイトとエラー型
- templatia-derive: 構造体用の実装を自動生成するプロシージャルマクロ

どちらのクレートも本リポジトリに含まれます。多くの利用者は、templatia を derive 機能とともに依存に追加するだけで利用できます。

## 特徴
- 名前付き構造体に対する Template の導出
- デフォルトのテンプレートを自動生成（1フィールド1行: `name = {name}`）
- 属性でカスタムテンプレートを指定: `#[templatia(template = "...")]`
- ラウンドトリップ対応: `to_string` と `from_string`
- 分かりやすいエラー（`TemplateError`）

## 対応Rustバージョン（MSRV）
- Rust 1.85.0
- Edition 2024

## インストール
Cargo.toml に templatia を追加します。

1) templatiaをインポートする。通常featuresに"derive"を追加して利用することを想定しています。
```toml
[dependencies]
templatia = { version = "0.0.1", features = ["derive"] }
```

```rust
use templatia::Template; 

#[derive(Template)]
struct Config {
    host: String,
    port: u16,
}

fn main() {
    let cfg = Config { host: "localhost".into(), port: 5432 };
    let s = cfg.to_string();
    assert!(s.contains("host = localhost"));
    assert!(s.contains("port = 5432"));
}
```

## 使い方
### デフォルトテンプレート
テンプレートを指定しない場合、各フィールドが1行ずつ `name = {name}` という形式で合成されます。

```text
name = {name}
port = {port}
```

### カスタムテンプレート
構造体のフィールド名のプレースホルダを使い、templatia 属性で書式を指定できます。
```rust
use templatia::Template;

#[derive(Template)]
#[templatia(template = "{host}:{port}")]
struct DbCfg {
    host: String,
    port: u16,
}

fn main() {
    let cfg = DbCfg { host: "db.example.com".into(), port: 3306 };
    assert_eq!(cfg.to_string(), "db.example.com:3306");

    let parsed = DbCfg::from_string("db.example.com:3306").unwrap();
    assert_eq!(parsed.host, "db.example.com");
    assert_eq!(parsed.port, 3306);
}
```

### プレースホルダと型
- テンプレート内の `{name}` は、該当する名前付きフィールドと一致している必要があります
- テンプレートで使用されるフィールド型は Display と FromStr を実装している必要があります
- 同じプレースホルダを複数回登場させることは可能ですが、解析結果は互いに矛盾しない必要があります

## エラー
templatia は解析や検証に関するシンプルなエラー型を提供します。

- TemplateError::InconsistentValues { placeholder, first_value, second_value }
  - 同一プレースホルダが複数回現れた際に、解析された値が矛盾している場合に発生します
- TemplateError::Parse(String)
  - 一般的な解析エラーを表すメッセージ

## クレート概要
- templatia
  - Template トレイト（`fn to_string(&self) -> String` と `fn from_string(s: &str) -> Result<Self::Struct, Self::Error>`）
  - エラー報告のための TemplateError 列挙型
- templatia-derive
  - 名前付き構造体用の #[derive(Template)] マクロ
  - オプション属性: `#[templatia(template = "...")]`
  - プレースホルダが実在するフィールドに対応しているか検証

## フィーチャフラグ
- derive
  - 

## Road Map（0.0.x → 0.1.0）
- 0.0.2
  - [x] 欠損データのデフォルト挙動を定義: `#[templatia(allow_missing_placeholders)]` 属性により、テンプレートに含まれないフィールドを `Default::default()` で初期化可能
  - [ ] Option<T>: プレースホルダが無い場合は既定で None（`allow_missing_placeholders` 不要で自動対応）
- 0.0.3
  - [ ] エラーハンドリングと警告の充実化（診断の明確化とカバレッジ拡大）
- 0.0.4
  - [ ] Vec, HashMap, HashSet などコレクション向けの宣言的テンプレート対応
  - [ ] 親構造体でテンプレートの柔軟性を高めるための container 属性の追加
- 0.0.5
  - [ ] 非名前付き構造体（タプル構造体、ユニット構造体）、union 構造体、enum への対応

## テスト方針とドキュメント規約
本リポジトリはドキュメントおよびテストの規約として AGENTS.md に従います。概要:
- ドキュメントコメントは英語で記述します
- 例は最小限で正確なものとし、可能であれば doctest としてコンパイル可能にします
- すでに意図を正しく反映しているテストを、実装に合わせるためだけに変更しないでください

## ライセンス
次のいずれかのライセンスでデュアルライセンスされています:
- Apache License, Version 2.0 (LICENSE-APACHE または http://www.apache.org/licenses/LICENSE-2.0)
- MIT ライセンス (LICENSE-MIT または http://opensource.org/licenses/MIT)

いずれかの条件に従って本ソフトウェアを利用できます。

## 貢献
あなたが本プロジェクトに意図的に提供した貢献は、明示的な記載がない限り、Apache-2.0 ライセンスで定義されるところの本作業への包含を目的として提出されたものと見なし、上記のデュアルライセンスで提供されるものとします。追加の条件や制限はありません。