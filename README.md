# xml_deck_gen

`xml_deck_gen`は、スライドのPDFファイルを迅速かつ、簡単に生成するRustアプリケーションです。

プロジェクト内で書かれたXMLファイルを読み込み、PDFファイルを生成します。

![thumbnail](https://i.gyazo.com/fbbe2852de1a255eae7a9d318f1f7ab8.png) 


## 実行

フォントを指定する場合、`fonts`ディレクトリにフォントファイルを導入してください。

また、以下のフォントファイルを指定する箇所を適宜書き換えてください。

```rust
let font_family = fonts::from_files("./fonts/Noto_Sans/static/", "NotoSans", None)
    .expect("Failed to load font family");
```

プロジェクト内のルートでXMLファイルを作成し、下記のコマンドのように作成したXMLファイルを引数に追加してください。

```bash
cargo run -- sample_deck.xml
```

PDFファイルはプロジェクト内のルートに生成されます。

## 使用可能なXMLタグ

```xml
<deck>
  <page>
    <!-- snip -->
  </page>
</deck>
```

- `deck`はタグで囲まれた範囲をスライドとして生成します。
- `page`は一枚のページとして生成します。

```xml
<title>
  Sample Title
</title>
```

- `title`は生成するファイル名への使用と、スライドのタイトルとして生成します。

```xml
<head>
  Heading Text
</head>
```

- `head`は見出しテキストとして生成します。

```xml
<br></br>
```

- `br`は改行として生成します。

```xml
<text>Sample Text</text>
```

- `text`は通常のテキストとして生成します。

```xml
<image>
  <path>./path/to/image.jpg</path>
  <scale>0.9</scale>
</image>
```

- `image`は画像を生成します。
  - `path`は画像のパスを指定します。
  - `scale`は画像のリサイズを行います。
    - ページを超える大きさの画像を使用した場合、スライドに空白のページが生成されてしまうことを防ぎます。

```xml
<ul>
  <li>Apple</li>
  <li>Banana</li>
  <li>Cherry</li>
</ul>

<ol>
  <li>Red</li>
  <li>Green</li>
  <li>Yellow</li>
</ol>
```

- `ul`は順不同リストとして生成します。
- `ol`は順序付きリストとして生成します。

### サンプル

サンプルとして`sample_deck.xml`を作っています。 [sample_deck.xml](./sample_deck.xml)

`sample_deck.xml`から生成されたPDFファイルは`Sample_Deck.pdf`です。 [Sample_Deck.pdf](./Sample_Deck.pdf)
