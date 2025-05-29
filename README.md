<div align="center">

  <h1><code>mazeweb</code></h1>

  <strong>A simple, visual maze generator and solver built for the web.</strong>

  <sub>Powered by 🦀 Rust and 🕸 WebAssembly — by <a href="https://github.com/Chrisser1">Chrisser1</a></sub>
</div>

---

## Cell Representation

Each cell in the maze is a `u8`, using bit flags to efficiently encode both its **type** and **wall configuration**.



## Cells
```java
Bits: 7 6 5 4 | 3 2 1 0
      | | | |   | | | |
      | | | |   | | | └── North wall (1 = wall present)
      | | | |   | | └──── East wall
      | | | |   | └────── South wall
      | | | |   └──────── West wall
      └─┴─┴─┴──────────── Cell type (4-bit enum: 16 possible types)
```

### Example Types (bits 4–7):
- `0x00` = Empty
- `0x10` = Wall
- `0x20` = Start
- `0x30` = End
- `0x40` = Path
- `0x50` = Visited
- `0x60` = LookingAt
- `0x70` = Changing


## Usage
### Build with `wasm-pack build`

```
wasm-pack build --target web
```

### Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox web
```

## Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
