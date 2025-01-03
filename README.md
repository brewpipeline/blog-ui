# blog-ui
Blog UI made with Yew/WASM/Bootstrap

![1](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/1.png)

Features
---
- **Posts system** (page/list/create/edit/delete/publish)
- **Authors system** (page/list/create/edit)
- **Tags system** (page/create)
- **Comments system** (list/create/delete)
- Minimal administration system (roles/bans/control)
- Authorization (telegram/yandex/internal)
- Server-Side Rendering (SSR) (posts/post/authors/author/tag)
- SEO optimized (search/social)
- Search (posts/authors)
- Telegram notifications (post publish)
- Images mirroring
- Deploy

How-to
---
1. Configure ENV vars mentioned in [job](https://github.com/brewpipeline/blog-ui/blob/main/.github/workflows/builds.yml) or in [lib](https://github.com/brewpipeline/blog-ui/blob/main/src/lib.rs) file, where some items can be optional, based on selected features
```rust
#[cfg(all(feature = "client", feature = "yandex"))]
const YANDEX_CLIENT_ID: &'static str = std::env!("YANDEX_CLIENT_ID"); // ee156ec6ee994a748e724f604db8e305
#[cfg(feature = "client")]
const API_URL: &'static str = std::env!("API_URL"); // http://127.0.0.1:3000/api
#[cfg(feature = "telegram")]
const TELEGRAM_BOT_LOGIN: &'static str = std::env!("TELEGRAM_BOT_LOGIN"); // AnyBlogBot
const TITLE: &'static str = std::env!("TITLE"); // BLOG
const DESCRIPTION: &'static str = std::env!("DESCRIPTION"); // BLOG DESCRIPTION
const KEYWORDS: &'static str = std::env!("KEYWORDS"); // BLOG, KEYWORDS
const ACCORDION_JSON: &'static str = std::env!("ACCORDION_JSON"); // [{"title":"О блоге","body":"<strong>Ты ошибка эволюции.</strong><br/>А блог этот про хороших людей в плохое время."},{"title":"Контент","body":"Привет!"}]
```
2. Build YEW/app by [tutorial](https://yew.rs/docs/tutorial)

Links
---
- Project: https://github.com/orgs/brewpipeline/projects/3
- UI(current) part: https://github.com/brewpipeline/blog-ui
- Server part: https://github.com/brewpipeline/blog-server
- Notifications part: https://github.com/brewpipeline/BlogNotificationService
- Images part: https://github.com/brewpipeline/images-processor-service
- Deploy part: https://github.com/brewpipeline/blog-deploy

Images
---

![2](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/2.png)

![3](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/3.png)

![4](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/4.png)

![5](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/5.png)

![6](https://raw.githubusercontent.com/brewpipeline/blog-ui/main/images/6.png)
