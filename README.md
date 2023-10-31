# Falco dummy plugin written in Rust
## How to compile

```
git clone https://github.com/mrgian/dummy-rs.git
cd dummy-rs
cargo build
```
## How to use
Enable the plugin in the Falco configuration file:

```
rules_file:
  - <path_to_dummy-rs>/dummy_rules.yaml

load_plugins: [dummy-rs]

plugins:
  - name: dummy-rs
    library_path: <path_to_dummy-rs>/target/debug/libdummy_rs.so
```
