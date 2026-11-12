# Portfolio

Dioxus 0.7 (Rust) で作成した個人ポートフォリオサイト。GitHub Pages で公開している。

公開URL: https://okonomiyak.github.io/portfolio

## 技術スタック

- [Dioxus 0.7](https://dioxuslabs.com/learn/0.7)(`web` フィーチャー、WASM としてビルド)
- SCSS(`dx` のアセットパイプラインが内蔵の [grass](https://github.com/connorskees/grass) で自動コンパイル。別途 Node.js や `sass` コマンドは不要)
- GitHub Actions + GitHub Pages でのホスティング

## 必要なツール

- Rust ツールチェーン + `wasm32-unknown-unknown` target
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- [dioxus-cli](https://dioxuslabs.com/learn/0.7/CLI/) — `Cargo.toml` の `dioxus` クレートとバージョンを揃えること(ずれると `dx` がバージョン不一致の警告を出し、ファイル変更時の自動リビルドが効かなくなることがある)
  ```sh
  cargo binstall dioxus-cli@0.7.10
  ```

Node.js や npm は不要。SCSS は `asset!("/styles/main.scss")` 経由で `dx` に直接渡しており、`dx serve`/`dx bundle` が内部の `grass` でコンパイルする。

## 開発用ビルド
SCSSを一定間隔保存させる場合(`dx` が SCSS の変更を拾い損ねることがあるため。間隔は `INTERVAL` で秒指定、デフォルト20秒)
```sh
./scripts/dev.sh
```

```bat
scripts\dev.bat
```

手動実行
```sh
dx serve
```

これだけで Rust コードと SCSS の両方がビルド・監視される。`styles/` 以下を編集すると自動でリコンパイル・ホットリロードされる。

起動後は以下にアクセスする(`Dioxus.toml` の `base_path = "portfolio"` のため、パスに `/portfolio/` が付く)。

```
http://127.0.0.1:8080/portfolio/
```

## ディレクトリ構成

```
src/
  main.rs          ルーティング(Route enum)
  data.rs          作品・言語(Stack)・ニュース(NEWS)のデータ
  views/           ページ単位のコンポーネント(home, about, workdetail など)
  components/      部品(navbar, footer, carousel, cworks, newsboxs など)
styles/
  main.scss        エントリーポイント。各パーシャルを @use でまとめる
  abstracts/       変数(_variables)と関数(_functions)。CSS は出力しない
  base/            テーマ(ライト/ダークの CSS 変数)、リセット、全体レイアウト
  components/      部品ごとのスタイル
  layout/          ナビ・フッター
  pages/           ページごとのスタイル
scripts/           開発用スクリプト(dev.sh / dev.bat)
```

コンテンツ(作品やニュース)の追加は、基本的に `src/data.rs` を編集するだけで済む。

## スタイルの書き方

### 色は `_variables.scss` の変数を使う

色はすべて `styles/abstracts/_variables.scss` に定義し、各ファイルでは変数で指定する。カラーコードを直接書かない。

色は2種類ある。

- **テーマで切り替わる色**: `$light` / `$dark` のマップに値を書く。`base/_theme.scss` がそれを CSS 変数(`--text` など)として出力し、OS がダークモードのときは `$dark` の値に切り替わる。`$text` などの変数の中身は `var(--text)` なので、`rgba($text, .5)` のような Sass の色関数は使えない。透明度が要るときは `color-mix()` を使う。
- **テーマによらない色**: 普通の Sass 変数。暗い帯・暗いカード・青いボタンの上など、どちらのテーマでも同じ見た目にしたい場所に使う。

```scss
@use "../abstracts/variables" as *;

.card {
  background-color: $Sbg;                                      // テーマで切り替わる
  color: $text;
  box-shadow: 0 12px 32px rgba($shadow, .10);                  // 固定色は rgba(変数, α) でOK
  border: 1px solid color-mix(in srgb, #{$bg} 72%, transparent); // テーマ色に透明度をつけるとき
}
```

| 変数 | テーマ | 用途 |
|---|---|---|
| `$text` / `$subtext` | 切り替わる | 本文 / 補足テキスト |
| `$bg` / `$Sbg` / `$navbarbg` | 切り替わる | 背景 / カードなどの薄い背景 / やや濃い背景 |
| `$link` | 切り替わる | 本文中のリンク |
| `$border` / `$divider` | 切り替わる | 境界線 / 表の区切り線 |
| `$icon-filter` | 切り替わる | 単色アイコンの `filter`(ライトで黒、ダークで白) |
| `$band` / `$textbg` | 固定 | トップの暗い帯 / Cworks のスライド |
| `$on-dark` / `$on-dark-sub` | 固定 | 暗い面・青い面の上の文字 / 補足テキスト |
| `$accent` / `$accent-hover` | 固定 | 青ボタン・フォーカス枠 / ボタンのホバー |
| `$shadow` | 固定 | 影・薄い線の元の色(必ず透明度をつけて使う) |
| `$glow-start` / `$glow-mid` / `$glow-end` | 固定 | トップの見出しのグラデーション |

新しい色が必要になったら、まず `_variables.scss` に変数を追加してから使う。テーマで切り替えたい色なら `$light` と `$dark` の両方に値を書き、`$名前: var(--名前);` を足す。

### フォント

- 本文: `$font-main`(OS の標準フォント。Web フォントは読み込まない)
- ナビ: `$font-English`(JetBrains Mono。`main.rs` で Google Fonts から読み込んでいる。ファミリー名はスペースありの `"JetBrains Mono"`)

### 文字サイズは `fluid()` で可変にする

`styles/abstracts/_functions.scss` の `fluid($min, $max, $min-vw, $max-vw)` を使うと、画面幅に応じて連続的にサイズが変わる。メディアクエリで段階的に切り替えるより崩れにくい。

```scss
h1 { font-size: fluid(28px, 80px, 320px, 900px); }   // 幅320pxで28px 〜 900pxで80px
```

### カルーセル

横スクロールのスライドは `components::Carousel` を使う。子要素に `class: "slide"` を付ければ、スナップ・ドット表示は共通で動く。見た目(`.slide` の中身)は呼び出し側の SCSS で書く。

```rust
Carousel { id: "news", count: items.len(),   // id はページ内で重複させない
    for item in items {
        Link { class: "slide", to: /* ... */, "{item.title}" }
    }
}
```

親が `align-items: center` の flex の場合、カルーセルを包む要素に `width: 100%` を付けないと、全スライド分の幅まで広がって画面からはみ出す(`#cworks` 参照)。

## 本番用ビルド

```sh
dx bundle --platform web --release
```

出力先: `target/dx/portfolio/release/web/public/`

SCSS のコンパイルもこのコマンド内で自動的に行われるため、別途コンパイル済み CSS をコミットしておく必要はない。

## GitHub Pages 用の設定

### base_path

`https://<username>.github.io/<repo-name>/` の形式で公開する場合、`Dioxus.toml` に以下を設定する(このリポジトリでは既に設定済み)。

```toml
[web.app]
base_path = "portfolio"
```

ユーザーページ(`<username>.github.io`)や独自ドメインで公開する場合は不要。

### 404.html(Router 使用時のみ)

GitHub Pages は存在しないパスに 404 を返すため、Router を使った SPA のルーティングがリロード時に効かなくなる。ビルド出力の `index.html` を `404.html` としてコピーして回避する。

```sh
cp target/dx/portfolio/release/web/public/index.html target/dx/portfolio/release/web/public/404.html
```

GitHub Actions のワークフロー内で自動実行されるため、手動作業は不要。

## デプロイ(GitHub Actions)

`.github/workflows/deploy.yml` が `main` への push をトリガーに以下を行う。

1. `wasm32-unknown-unknown` ターゲット付きで Rust をセットアップ
2. `dioxus-cli` を `cargo binstall` でインストール
3. `dx bundle --platform web --release` でビルド(SCSS のコンパイルも含む)
4. `index.html` を `404.html` としてコピー
5. `target/dx/portfolio/release/web/public` を GitHub Pages にアップロード・デプロイ

初回のみ、リポジトリの **Settings → Pages → Source** を **GitHub Actions** に変更しておく。

## 注意点

- `dioxus-cli` のバージョンは `Cargo.toml` の `dioxus` クレートのバージョンと揃える。ずれると `dx` が起動時に「dx and dioxus versions are incompatible」という警告を出し、`.rs`/`styles/` の変更を保存しても自動リビルドが走らなくなることがある。症状としては「コードやスタイルを直したのにブラウザに反映されない」ので、その場合はまず `dioxus-cli` のバージョンを揃えるか、`dx serve` を再起動してフルビルドさせる。
- 画像などのアセットが表示されない場合は、`base_path` の設定漏れを疑う。

## 使用物
> アイコン: Simple Icons、Devicon