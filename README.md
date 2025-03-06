# `goose-rss`

A desktop RSS/Atom feed reader built with Rust + Tauri.  The initial implementation will be based on [collie](https://github.com/collie-reader/collie), but I plan to add more features which suit my personal feed reader needs.

## Comparison to `collie`

`goose` improves upon `collie` in the following ways:

* upgrade from `tauri@1` to `tauri@2`
* use `tauri-specta` to generate TypeScript bindings for Tauri commands
