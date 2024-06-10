# News
Application that displays news from various RSS feeds.

## Features
- [x] Fetching RSS news
- [x] Managing source of RSS news.
- [x] Summaration of news articles by AI.
- [ ] Local LLM models support
- [x] OpenAI LLM model support.
- [ ] GUI app

## Technology
- Rust
- Tauri
- Svelte
- Sqlite
- AI

## Workspaces
- shared - shared code between server and gui.
- news-gui - tauri app
- server - tokio server handling fetching rss feeds and calling OpenAI etc.

## License
MIT