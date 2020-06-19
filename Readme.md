# Rust web text editor

Experiment building a web text editor using the DOM API, yew & rust bindgen

Note: Clipboard support requires the web_sys unstable [Clipboard API](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard) enabled.

```toml
# .cargo/config

[target.wasm32-unknown-unknown]
rustflags = ['--cfg=web_sys_unstable_apis']
```