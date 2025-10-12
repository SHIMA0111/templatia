[![CI](https://github.com/SHIMA0111/templatia/actions/workflows/ci.yml/badge.svg)](https://github.com/SHIMA0111/templatia/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/templatia.svg)](https://crates.io/crates/templatia)
[![Docs.rs](https://docs.rs/templatia/badge.svg)](https://docs.rs/templatia)
[![Crates.io MSRV (version)](https://img.shields.io/crates/msrv/templatia/0.0.3)](https://crates.io/crates/templatia)
[![Downloads](https://img.shields.io/crates/d/templatia.svg)](https://crates.io/crates/templatia)

# templatia

Rustの構造体とテキストのシームレスな相互変換をユーザが定義するテンプレートに従って実現するテンプレートベースのシリアライズ/デシリアライズライブラリです。
このライブラリは以下の二つのクレートから実現しています。

- templatia: コアのトレイトとエラーを提供するライブラリ
- templatia-derive: ユーザのテンプレートに従って構造体とテキストの相互互換を実現するための処理を自動生成するマクロライブラリ

通常はどちらかを単体で利用するのではなく、`templatia`の`derive`feature経由で組み合わせて利用することが想定されたライブラリです。
(ただし、templatia-deriveは現時点では`named_struct`のみをサポートしているため、特殊な型には独自実装することも可能です。)

## 特徴
- Rustの構造体とテキストのシームレスな相互変換
- デフォルトテンプレートとして全てのフィールドをkey-value形式: `{field_name} = {field_name}`
- `templatia`属性を利用したカスタムテンプレートの定義: `#[templatia(template = "...")]`
- 明確な実行時エラーと分かりやすいコンパイルエラーの出力
  - コンパイルエラーの例
    - 隣り合うことで曖昧さを生む組み合わせの接触コンパイルエラー
      - StructName: Placeholder "field1" and "field2" are consecutive. These are ambiguous to parsing.
        "field1" is `String` type data. Consecutive allows only: [char, bool]
    - 構造体フィールド全てがテンプレート内のプレースホルダーに含まれない場合のコンパイルエラー
      - StructName has more field specified than the template's placeholders: field1, field2, field3
        If you want to allow missing placeholders, use `#[templatia(allow_missing_placeholders)]` attribute.


## 対応Rustバージョン（MSRV）
- Rust 1.85.0
- Edition 2024

## インストール
### cargo addコマンドを利用する
```shell
cargo add templatia --features derive
```

### Cargo.tomlで直接記述する
1) templatiaをインポートする。featuresには`derive`を追加します。
```toml
[dependencies]
templatia = { version = "0.0.3", features = ["derive"] }
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
    let s = cfg.render_string();
    assert!(s.contains("host = localhost"));
    assert!(s.contains("port = 5432"));
}
```

## クイックスタートガイド
### デフォルトのテンプレート
テンプレートを指定しない場合、各フィールドが1行ずつ `field_name = {field_name}` という形式で合成されます。
例えば
```rust
#[derive(Template)]
struct AwesomeStruct {
  data1: String,
  data2: u32,
}

fn main() {
  let data = AwesomeStruct { data1: "data1".into(), data2: 100 };
}
```
のようにした場合は
```text
data1 = {data1}
data2 = {data2}
```
という形式でテンプレートが生成され、render_string()を実行した場合には
```text
data1 = data1
data2 = 100
```
という出力を得ることができます。

### カスタムテンプレート
`templatia`属性内の`template`に構造体のフィールド名で`{}`で囲ったプレースホルダーを使用すると、カスタムのテンプレートを定義できます。  
以下のケースでは`"{host}:{port}"`を定義しているため、`cfg`からは`db.example.com:3306`を得ることができます。
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
    assert_eq!(cfg.render_string(), "db.example.com:3306");

    let parsed = DbCfg::from_str("db.example.com:3306").unwrap();
    assert_eq!(parsed.host, "db.example.com");
    assert_eq!(parsed.port, 3306);
}
```

### Option<T>のサポート
`Option<T>` 型のフィールドは、プレースホルダがテンプレートに存在しない場合、自動的に `None` になります:

```rust
use templatia::Template;

#[derive(Template)]
#[templatia(template = "host={host}:{port}", allow_missing_placeholders)]
struct ServerConfig {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
}

fn main() {
    let config = ServerConfig::from_str("host=localhost:8080").unwrap();
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);
    assert_eq!(config.username, None); // テンプレートにないため、Noneになる
    assert_eq!(config.password, None); // テンプレートにないため、Noneになる
}
```

デフォルトでは、`Option<String>` の空文字列は `None` としてパースされます。空文字列を `Some("")` として扱うには、`empty_str_option_not_none` 属性を使用します:

```rust
use templatia::Template;

#[derive(Template)]
#[templatia(template = "value={value}", empty_str_option_not_none)]
struct OptionalValue {
    value: Option<String>,
}

fn main() {
    let parsed = OptionalValue::from_str("value=").unwrap();
    assert_eq!(parsed.value, Some("".to_string())); // 空文字列がSome("")になる
}
```

### プレースホルダの欠損を許可
`allow_missing_placeholders` 属性を使用すると、テンプレートに含まれないフィールドを許可できます:

```rust
use templatia::Template;

#[derive(Template)]
#[templatia(template = "id={id}", allow_missing_placeholders)]
struct Config {
    id: u32,
    name: String,           // テンプレートにないため、Default::default()を使用
    optional: Option<u32>,  // テンプレートにないため、Noneになる
}

fn main() {
    let config = Config::from_str("id=42").unwrap();
    assert_eq!(config.id, 42);
    assert_eq!(config.name, "");          // Stringのデフォルト値
    assert_eq!(config.optional, None);     // Option<T>はNone
}
```

### プレースホルダと型
- テンプレート内の `{name}` は、該当する名前付きフィールドと一致している必要があります
- テンプレートで使用されるフィールド型は Display と FromStr を実装している必要があります
  - `allow_missing_placeholders`を有効にしている場合にはDefaultの実装も必要になります。
- 同じフィールドのプレースホルダーを複数回テンプレート内で利用することが可能ですが、from_str()時には同じフィールドのプレースホルダーは同じ値である必要があります。
  - `"{first_name} (Full: {first_name} {family_name})"`となっていた場合に`Taro (Full: Jiro Yamada)`を構造体にデシリアライズすることはできません。

## 実行時エラー
templatia は解析や検証に関するシンプルなエラー型を提供します。

- TemplateError::InconsistentValues { placeholder, first_value, second_value }
  - 同一プレースホルダが複数回現れた際に、解析された値が矛盾している場合に発生します
- TemplateError::Parse(String)
  - 一般的な解析エラーを表すメッセージ

## クレート概要
- templatia
  - Template トレイト
    - `templatia`の振る舞いを定義したトレイトです。  
      `render_string()`と`from_str()`という二つのメソッドと一つの関連型`Error`を定義しています。
  - TemplateError
    - templatia-deriveのデフォルトのエラーです。
- templatia-derive
  - #[derive(Template)] マクロ
  - オプション属性:
    - `#[templatia(template = "...")]` カスタムテンプレート用
    - `#[templatia(allow_missing_placeholders)]` テンプレートにないフィールドを許可
    - `#[templatia(empty_str_option_not_none)]` `Option<String>`の空文字列を`Some("")`として扱う

## フィーチャフラグ
- derive
  - templatia-deriveを有効にするフラグです。これを有効にすることで`templatia::Template`をderiveできるようになります。

## Road Map（0.0.x → 0.1.0）
- 0.0.2
  - [x] 欠損データのデフォルト挙動を定義: `#[templatia(allow_missing_placeholders)]` 属性により、テンプレートに含まれないフィールドを `Default::default()` で初期化可能
  - [x] Option<T>: プレースホルダが無い場合は既定で None（`allow_missing_placeholders` 不要で自動対応）
  - [x] `Template`トレイトから関連型の`type Struct`を削除
- 0.0.3
  - [x] エラーハンドリングの充実化（compile-failテストによる診断の明確化とカバレッジ拡大）
  - [x] 将来の機能実装に備えた内部リファクタリング
- 0.0.4
  - [ ] Vec, HashMap, HashSet などコレクション向けの宣言的テンプレート対応
  - [ ] 親構造体でテンプレートの柔軟性を高めるための container 属性の追加
- 0.0.5
  - [ ] 非名前付き構造体（タプル構造体、ユニット構造体）、union 構造体、enum への対応
- 0.0.6以降（将来バージョン）
  - [ ] オプショナルプレースホルダー構文: `{name?}` 形式でプレースホルダー自体をオプショナルにする
    - フィールドが `Option<T>` の場合、値が `None` のときにプレースホルダー部分を空文字列として扱う
    - パース時にはプレースホルダーが存在しない場合に `None` を返す
  - [ ] 範囲オプショナル構文: `[literal{placeholder}literal]?` 形式でテンプレートの一部をオプショナルにする
    - 例: `#[templatia(template = "[name={name}]?")]` とすると、`name` が `None` の場合に `name=` ごと出力から除外される
    - パース時の一貫性を保ちつつ、オプショナルなセクション全体の存在/非存在を表現可能

## テスト方針とドキュメント規約
本リポジトリはドキュメントおよびテストの規約として AGENTS.md に従います。概要:
- ドキュメントコメントは英語で記述します
- 例は最小限で正確なものとし、可能であれば doctest としてコンパイル可能にします
- すでに意図を正しく反映しているテストを、実装に合わせるためだけに変更しないでください

## ライセンス
次のいずれかのライセンスでデュアルライセンスされています:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-ap.md) または http://www.apache.org/licenses/LICENSE-2.0)
- MIT ライセンス ([LICENSE-MIT](LICENSE-mit.md) または http://opensource.org/licenses/MIT)

いずれかの条件に従って本ソフトウェアを利用できます。

## 貢献
あなたが本プロジェクトに意図的に提供した貢献は、明示的な記載がない限り、Apache-2.0 ライセンスで定義されるところの本作業への包含を目的として提出されたものと見なし、上記のデュアルライセンスで提供されるものとします。追加の条件や制限はありません。