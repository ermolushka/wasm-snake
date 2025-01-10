## About

A basic implementation of the snake game compiled to WASM afor running in the browser.



## 🚴 Usage


### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### Compression

* You can compress a final `.wasm` file using `wasm-opt` `wasm-opt -Oz -o pkg/wasm_snake_bg.wasm pkg/wasm_snake_bg.wasm`

### Running UI

```bash
cd www-new && npm install && NODE_OPTIONS=--openssl-legacy-provider npm run start
```

Then go to [url](http://localhost:8080)

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you


## License

Apache License, Version 2.0, [LICENSE-APACHE](LICENSE-APACHE)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
