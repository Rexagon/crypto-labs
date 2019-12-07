# crypto-labs

### lab1
```shell script
cargo build -p lab1 --release

head -c10000 test/large.txt \
    | target/release/lab1-encrypt 12 \
    | target/release/lab1-decrypt lab1/texts/large.txt
```

### lab2
```shell script
cargo build -p lab2 --release

target/release/lab2
```

### lab3
```shell script
cargo build -p lab3 --release

target/release/lab3
```

### lab4
```shell script
cargo build -p lab4 --release

target/release/lab4
```
