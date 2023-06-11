# RustOfLife #
Game of life simulation implemented with Rust.

### Setup ###
This considers that your Rust environment is already installed, install the following:
```shell
rustup target add wasm32-unknown-unknown # Wasm compiler
cargo install wasm-bindgen-cli            # Generated JS bindings
cargo install basic-http-server           # Serves the web application 
```

### Build ###
- In order to create the `wasm` binary, execute the following command:
```shell
cargo build --release --target wasm32-unknown-unknown
```
- Generate the Javascript binding:
 ```shell
wasm-bindgen --out-name game_of_life \ 
  --out-dir ../web/wasm \
  --target web target/wasm32-unknown-unknown/release/rust_of_life.wasm
```

### Usage ###
- Navigate to the `web` folder and execute the following:
```shell
basic-http-server .
```
- Some information about the server will be displayed in which one of them should be the web application address
