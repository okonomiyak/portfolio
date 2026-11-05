# Portfolio

Dioxus 0.7 (Rust) で作成した個人ポートフォリオサイト。GitHub Pages で公開している。

公開URL: https://okonomiyak.github.io/portfolio

## 技術スタック

- [Dioxus 0.7](https://dioxuslabs.com/learn/0.7)(`web` フィーチャー、WASM としてビルド)
- SCSS([Dart Sass](https://sass-lang.com/) でコンパイル)
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
│  │  ├─ works.rs         # 使用技術を横スクロールで見せるスライダー
│  │  ├─ hero.rs          # (未使用。テンプレート由来)
│  │  └─ mod.rs
│  └─ views/
│     ├─ home.rs          # トップページ(Hero + 自己紹介 + Works)
│     └─ mod.rs
├─ styles/                 # SCSS ソース(partial 単位で分割)
│  ├─ main.scss            # 各 partial を @use するだけのエントリーファイル
│  ├─ _variables.scss       # 色・フォント・サイズなどの変数
│  ├─ _base.scss            # html/body のリセットと基本フォント
│  ├─ _buttons.scss         # .bluebutton / .whitebutton
│  ├─ _navbar.scss
│  ├─ _home.scss
│  ├─ _works.scss
│  └─ _footer.scss
├─ assets/
│  ├─ main.css             # styles/*.scss をコンパイルした生成物(コミット対象)
│  ├─ main.css.map
│  ├─ favicon.ico
│  └─ img/                 # 技術ロゴなどの画像
├─ Cargo.toml               # Rust 依存とフィーチャーフラグ
├─ Dioxus.toml               # dx のビルド設定(base_path など)
├─ package.json             # sass (npm) の依存のみ
└─ .github/workflows/deploy.yml  # GitHub Pages への自動デプロイ
```

## 必要なツール

- Rust ツールチェーン + `wasm32-unknown-unknown` target
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- [dioxus-cli](https://dioxuslabs.com/learn/0.7/CLI/) — `Cargo.toml` の `dioxus` クレートとバージョンを揃えること(ずれるとビルドが失敗する)
  ```sh
  cargo binstall dioxus-cli@0.7.1
  ```
- Node.js(SCSS コンパイル用の `sass` を使うため)
  ```sh
  npm install
  ```

## 開発用ビルド

SCSS は Dioxus のビルドパイプラインでは自動コンパイルされないため、`sass` で `styles/main.scss` を `assets/main.css` に変換してから `dx serve` を起動する。2つのプロセスを並行して動かす。

```sh
npx sass --watch styles/main.scss assets/main.css
```

```sh
dx serve
```

起動後は以下にアクセスする(`Dioxus.toml` の `base_path = "portfolio"` のため、パスに `/portfolio/` が付く)。

```
http://127.0.0.1:8080/portfolio/
```

SCSS を編集したあと、`assets/main.css` が更新されていることを確認してからコミットする(`sass --watch` を止めた状態でコミットしないこと)。

## 本番用ビルド

```sh
npx sass styles/main.scss assets/main.css
dx bundle --platform web --release
```

出力先: `target/dx/portfolio/release/web/public/`

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
3. `dx bundle --platform web --release` でビルド
4. `index.html` を `404.html` としてコピー
5. `target/dx/portfolio/release/web/public` を GitHub Pages にアップロード・デプロイ

初回のみ、リポジトリの **Settings → Pages → Source** を **GitHub Actions** に変更しておく。

SCSS のコンパイル(`npx sass`)はこのワークフローには含まれていない。`assets/main.css` は事前にコンパイルしてコミットしておく必要がある。

## 注意点

- `dioxus-cli` のバージョンは `Cargo.toml` の `dioxus` クレートのバージョンと揃える。ずれるとビルドに失敗する。
- `assets/main.css` は SCSS から生成される成果物だが、デプロイ用ワークフローがコンパイルを行わないため、コミットし忘れると本番に反映されない。
- 画像などのアセットが表示されない場合は、`base_path` の設定漏れを疑う。
