# GWG + `wasm-bindgen` example

This experiment shows connecting [`wasm-bindgen`](https://rustwasm.github.io/docs/wasm-bindgen/)
generated code to a [`good-web-game`](https://github.com/not-fl3/good-web-game) application.

It uses `simple_logger` crate to `log!` in native mode and `web_logger` in WASM mode.
Both versions build from the same codebase. You should see ggwg main-loop lifecycle
methods calls in console.

It requires a simple modification in the generated `_bg.js` file (replace
`import * as wasm ...` with `export const wasm = {}`)
and a simple boilerplate plugin for `miniquad` wasm loader - see `index.html`.

## Building

Native:

    $ cargo run
        Finished dev [unoptimized + debuginfo] target(s)
         Running `target/debug/gwg_bindgen`

WASM:

    $ cargo build --release --target wasm32-unknown-unknown
       Compiling gwg_bindgen v0.1.0 (gwg_bindgen)
        Finished release [optimized] target(s)
    $ wasm-bindgen --out-dir target --target bundler target/wasm32-unknown-unknown/release/gwg_bindgen.wasm
    $ sed -i 's/import.*as wasm .*/export const wasm = {};/' target/gwg_bindgen_bg.js

Then serve and open current dir. i.e.

    $ basic-http-server .
    [INFO ] basic-http-server 0.8.1
    [INFO ] addr: http://127.0.0.1:4000
    [INFO ] root dir: .
