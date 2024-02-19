# WIP
## Usage
```
git clone https://github.com/mrgian/plugin-sdk-rs.git
cd plugin-sdk-rs
cargo build --package=dummy
sysdig -H target/debug/libdummy.so -I dummy -p"Dummy event: sample.hello=%sample.hello sample.num=%sample.num"
```