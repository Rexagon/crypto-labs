# crypto-labs

### lab1
```
cargo build --release

head -c10000 test/large.txt \
    | target/release/lab1-encrypt 12 \
    | target/release/lab1-decrypt test/large.txt
```
