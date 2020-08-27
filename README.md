# GWG + `wasm-bindgen` example

This experiment shows connecting [`wasm-bindgen`](https://rustwasm.github.io/docs/wasm-bindgen/)
generated code to a [`good-web-game`](https://github.com/not-fl3/good-web-game) application.

It uses `simple_logger` crate to `log!` in native mode and `web_logger` in WASM mode.
Both versions build from the same codebase. You should see gwg main-loop lifecycle
methods calls in console.

It requires a modification in the bindgen generated `.js` file (see sed call below)
and a boilerplate plugin for `miniquad` wasm loader - see `index.html`.

## Building

### Native

    $ cargo run
        Finished dev [unoptimized + debuginfo] target(s)
         Running `target/debug/gwg_bindgen`

### WASM

#### pre-req

    $ rustup target add wasm32-unknown-unknown
    info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
    info: installing component 'rust-std' for 'wasm32-unknown-unknown'
    $ cargo install wasm-bindgen-cli

#### build & run

    $ cargo build --release --target wasm32-unknown-unknown
       Compiling gwg_bindgen v0.1.0 (gwg_bindgen)
        Finished release [optimized] target(s)
    $ wasm-bindgen --out-dir target --target web target/wasm32-unknown-unknown/release/gwg_bindgen.wasm
    $ sed -i 's/import.*from .env.;/init.set_wasm = w => wasm = w;/;s/imports\[.env.\] =.*/return imports;/' target/gwg_bindgen.js

Then serve project dir to browser. i.e.

    $ basic-http-server .
    [INFO ] basic-http-server 0.8.1
    [INFO ] addr: http://127.0.0.1:4000
    [INFO ] root dir: .
