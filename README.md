# WASIとWITを理解するぞ！

## [WASIとは](https://wasmbyexample.dev/examples/wasi-introduction/wasi-introduction.all.en-us.html)

WebAssembly System Interfaceのこと。
WebAssemblyでのモジュール式のシステムインタフェース

### どんなメリットがあるんだよ
クロスプラットフォーム化が可能がWASMによって可能になり、WASMをコンパイルして出力する際、作成したWASMアプリとそれと依存関係を持つクレートを単一のものとしてコンテナ化できる。
さらに、ブラウザ外でWASMが使用できるというこれまでの利点を持ちつつ、Webで運用するWASMではできなかったシステムリソースへのアクセスが可能に。
ファイルシステムへのアクセス、ネットワークに関するシステムへのアクセス、システム内の時間にアクセスが可能になる。

## [WITとは](https://zenn.dev/chikoski/articles/webassembly-interface-type-101)
簡単に書くと、WASMで作成されたコンポーネントのインタフェースを定義する言語
パッケージという名称で名前空間を提供し、このimportとexportの定義のことをworldと呼称する。

#️## 背景事情
そもそもWASMモジュールには、整数型の32ビットと64ビット、浮動小数点型の32ビットと64ビットの型しか存在せず
文字列やユーザ定義型のような構造を持つ型の表現に標準的なものが存在せず、データをどのようにメモリに配置するのかは
その言語の処理系次第か、コードを書く人間がなんとかするものとされていたものを解決したいという事情があった。

つまり、構造体を使用した型の値のメモリ配置は、言語やそのコンパイラや、コンパイル時の設定などによって大きく左右される。
利用する側は必死こいてどういうメモリ配置で動かしているのかを必死に調べる必要に追われる。これは非常に使いづらいぞ！

メモリ上にどうやって配置しているのか記述するためのセクションとなる部分がWASMの仕様上に存在しないこともあり、
相互運用性も開発体験もクソッタレになってしまう。
なんとかこの2つを高めたい！という思いのもと、コンポーネントモデルが生まれた。
これがコンポーネントとしての構造に加えて、以下の2つの仕様で構成されている

- [Canonical ABI](https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md)
- [WebAssembly Interface Type](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)

Canonical ABIはデータ構造の線型メモリ上の配置と、メモリからの読み取り方法を規定し、相互運用性の問題を解決
WITは開発体験を高めるための仕様として、WASMコンポーネント間のインターフェースを定義するための言語を定めている

そのインターフェース定義言語、いわゆるIDLを使用してWASMコンポーネントのインタフェースを定義する
そのインターフェースを開発ツールがよしなに処理してグルーコードを生成してくれるようになり、
提供側はメモリ配置が使いやすくなるかに気を使わなくて良くなり、利用側はメモリ上の配置をいちいち気にせずWASMコンポーネントが提供する
機能を扱える。嬉しい。

##　wit-bindgenを用いてWITに入門する

###　cargo-componentを使ってみるパターン
WASMコンポーネントを1から作成するならこれを使うのが良い
WASMコンポーネントを作成するために便利なCargoのサブコマンドを追加する

### サブコマンド
| サブコマンド名 | 説明 | 利用例 |
| --- | --- | --- |
| new | Component開発用パッケージの作成 | cargo component new hello-world |
| build | Wasmコンポーネントをビルド | cargo component build -r |

パッケージ作成時に、reactorというオプションを指定すると、他プログラムから利用されるWASMコンポーネントを作成するためのパッケージを作成する
つけない場合はCLI向けのWASMコンポーネントを作成する

## WITファイルを見てみよう
```wit
package component:hello-world;

world example {
    export hello-world: func() -> string;
}
```

WITファイルには2つの要素があり、一つはパッケージ宣言、もう一つはワールドの宣言
| 要素 | 説明 | witファイル中の記述 |
| --- | --- | --- |
| パッケージ宣言 | 名前空間の宣言 | package component:hello-world |
| ワールド定義 | コンポーネントのインターフェースの定義 | world example {}|

