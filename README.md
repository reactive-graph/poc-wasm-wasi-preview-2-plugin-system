# poc-wasm-wasi-preview-2-plugin-system
Proof of Concept: Implement a Plugin System using WASM WASI preview 2

## Compile and run POC

### Prepare Environment

```shell
cargo install cargo-component --locked
rustup target add wasm32-wasi
```

### Compile Plugin API

```shell
cd reactive-graph-plugin-api
cargo component build
cd -
```

* Generates a reactive-graph-plugin-api/src/bindings.rs
* Generates libreactive_graph_plugin_api.rlib
* Used by the custom plugin and by the host system

### Compile Custom Plugin to WASM

```shell
cd reactive-graph-custom-plugin
cargo component build
cd -
```

* Generates a reactive-graph-custom-plugin/src/bindings.rs
* Generates reactive_graph_custom_plugin.wasm
* WASM file is loaded by the host system at runtime

### Run Host-System

```shell
cargo run
```

* The host system loads the WASM plugin
* The host system executes the greeting function of the loaded plugin
* The custom plugin directly prints to stdout
* The custom plugin receives a string parameter
* The custom plugin generates and returns a greeting message
* The host system prints the returned greeting message

## POC Status

* Completed
* Implementation functional
* Tooling functional

## Use cases

* Reactive Graph could use this to load plugins written in "any" language that compiles to WASM
* Reactive Graph could use this to load behaviours for reactive entities or reactive relations  

## Conclusion

* Very useful
* Can be used for behaviours AND for non-native-plugins
* Should be integrated
