# Portfolio

Dioxus 0.7 (Rust) で作成した個人ポートフォリオサイト。GitHub Pages で公開している。

公開URL: https://okonomiyak.github.io/portfolio

## 技術スタック

- [Dioxus 0.7](https://dioxuslabs.com/learn/0.7)(`web` フィーチャー、WASM としてビルド)
- SCSS(`dx` のアセットパイプラインが内蔵の [grass](https://github.com/connorskees/grass) で自動コンパイル。別途 Node.js や `sass` コマンドは不要)
- GitHub Actions + GitHub Pages でのホスティング

## プロジェクト構成

```
portfolio/
├─ src/
│  ├─ main.rs            # エントリーポイント。Router とルート定義 (Home / Blog)
│  ├─ data.rs             # Works セクションで使う技術スタックのデータ
│  ├─ components/
│  │  ├─ navbar.rs        # 共通ナビバー(レイアウト、Outlet を含む)
│  │  ├─ footer.rs        # 共通フッター
│  │  ├─ cworks.rs        # 使用技術を横スクロールで見せるスライダー(id="cworks")
│  │  ├─ hero.rs          # (未使用。テンプレート由来)
│  │  └─ mod.rs
│  └─ views/
│     ├─ home.rs          # トップページ(Hero + 自己紹介 + Cworks)
│     ├─ works.rs         # 作業中の Works ページ(未配線・プレースホルダー)
│     └─ mod.rs
├─ styles/                      # SCSS ソース。dx が main.scss を起点に自動コンパイルする
│  ├─ main.scss                  # 各 partial を @use するだけのエントリーファイル
│  ├─ abstracts/
│  │  ├─ _variables.scss          # 色・フォント・サイズなどの変数
│  │  └─ _functions.scss          # fluid() など、画面幅に応じて滑らかに変化する値を作る関数
│  ├─ base/
│  │  └─ _reset.scss              # html/body のリセットと基本フォント
│  ├─ components/
│  │  ├─ _buttons.scss            # .bluebutton / .whitebutton
│  │  ├─ _navbar.scss
│  │  └─ _cworks.scss
│  ├─ layout/
│  │  └─ _footer.scss
│  └─ pages/
│     ├─ _home.scss
│     └─ _work.scss               # Works ページ用(未着手)
├─ assets/
│  ├─ favicon.ico
│  └─ img/                 # 技術ロゴなどの画像
├─ Cargo.toml               # Rust 依存とフィーチャーフラグ
├─ Dioxus.toml               # dx のビルド設定(base_path など)
└─ .github/workflows/deploy.yml  # GitHub Pages への自動デプロイ
```

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

```sh
dx serve
```

これだけで Rust コードと SCSS の両方がビルド・監視される。`styles/` 以下を編集すると自動でリコンパイル・ホットリロードされる。

起動後は以下にアクセスする(`Dioxus.toml` の `base_path = "portfolio"` のため、パスに `/portfolio/` が付く)。

```
http://127.0.0.1:8080/portfolio/
```

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
