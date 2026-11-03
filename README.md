# WebPage

https://okonomiyak.github.io/portfolio

# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

# Portfolio

Dioxus (Rust) で作成した静的サイト。GitHub Pages で公開する。

## ビルド

### 完成用
```powershell
dx bundle --platform web --release
```

出力先: `target/dx/Portfolio/release/web/public/`

### 開発用
```powershell
dx serve
```

Url: http://127.0.0.1:8080/portfolio/

## GitHub Pages 用の設定

### base_path

`https://username.github.io/repo-name/` の形式で公開する場合、`Dioxus.toml` に追加する。

```toml
[web.app]
base_path = "repo-name"
```

ユーザーページ (`username.github.io`) や独自ドメインの場合は不要。

### 404.html (Router 使用時のみ)

GitHub Pages は存在しないパスに 404 を返すため、SPA のルーティングが効かなくなる。
`index.html` を `404.html` としてコピーして回避する。Router を使わない1ページだけのサイトでは不要。

実行場所はビルド出力の `public` フォルダ内にある `index.html` が対象。

プロジェクトのルートから実行する場合:

```powershell
Copy-Item target/dx/Portfolio/release/web/public/index.html target/dx/Portfolio/release/web/public/404.html
```

GitHub Actions を使う場合は、ワークフロー内で自動実行されるので手動作業は不要。

## デプロイ (GitHub Actions)

`.github/workflows/deploy.yml` を利用
その後、リポジトリの **Settings → Pages → Source** を **GitHub Actions** に変更する。
`main` に push すると自動で公開される。

## 注意点

- `dioxus-cli` のバージョンは、プロジェクトの `dioxus` クレートと揃える。ずれるとビルドが失敗する。`cargo binstall dioxus-cli@version` で固定できる。
- 出力パスの `Portfolio` 部分は、実際のプロジェクト名に合わせる。
- 画像などのアセットが表示されない場合は、`base_path` の設定漏れを疑う。