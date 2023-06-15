# wasm-game-of-life

Trying out wasm by following the [rust wasm book](https://rustwasm.github.io/docs/book/) and
[mdn wasm docs](https://developer.mozilla.org/en-US/docs/WebAssembly).

### Setup

To build the wams module run 
```
wasm-pack build --target bundler
```

To test the rust/wasm code run 
```
wasm-pack test --firefox --headless
```

To use the run the host the app using wasm api, 
```
cd www
npm install
npm run serve
```
