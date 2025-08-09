# blog-ui

Blog UI built with [Yew](https://yew.rs/) and WebAssembly, styled with Bootstrap.

![1](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/1.png)

## Features

- **Posts system** – page, list, create, edit, delete and publish
- **Authors system** – page, list and edit authors
- **Tags system** – create and manage tags
- **Comments system** – list, create and delete comments
- Minimal administration panel (roles, bans and access control)
- Authentication via Telegram, Yandex or internal accounts
- Server‑Side Rendering (SSR) for posts, authors and tags
- SEO‑friendly metadata for search and social networks
- Search across posts and authors
- Telegram notifications when a post is published
- Image mirroring and deployment helpers

## Getting started

### Prerequisites

- Rust with the `wasm32-unknown-unknown` target
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- [Trunk](https://trunkrs.dev/) for building and serving the app
  ```bash
  cargo install trunk
  ```

### Environment variables

The UI is configured through environment variables. Some of the most common ones are listed below:

| Variable | Description |
|---------|-------------|
| `YANDEX_CLIENT_ID` | OAuth identifier for Yandex authentication |
| `API_URL` | Base URL of the backend API, e.g. `http://127.0.0.1:3000/api` |
| `TELEGRAM_BOT_LOGIN` | Telegram bot login for notifications |
| `TITLE` | Title of the blog |
| `DESCRIPTION` | Meta description for SEO |
| `KEYWORDS` | Comma‑separated keywords |
| `ACCORDION_JSON` | JSON definition for the landing page accordion |

See [`src/lib.rs`](src/lib.rs) for the full list and optional items.

### Development

Start a development server with live reloading:

```bash
trunk serve
```

### Production build

Generate optimized assets:

```bash
trunk build --release
```

### Running tests

Unit tests can be executed with:

```bash
cargo test
```

## Related projects

- [Project board](https://github.com/orgs/brewpipeline/projects/3)
- [UI (current repository)](https://github.com/brewpipeline/blog-ui)
- [Server](https://github.com/brewpipeline/blog-server)
- [Notifications service](https://github.com/brewpipeline/BlogNotificationService)
- [Images processor](https://github.com/brewpipeline/images-processor-service)
- [Deployment scripts](https://github.com/brewpipeline/blog-deploy)

## Screenshots

![2](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/2.png)

![3](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/3.png)

![4](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/4.png)

![5](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/5.png)

![6](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/6.png)

![7](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/7.png)

## License

This project is licensed under the [MIT License](LICENSE).

