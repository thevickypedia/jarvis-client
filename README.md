# Jarvis Client
Client to communicate with Jarvis API via HTTP calls built with Rust

### Health
```shell
cargo run --bin health
```

### Offline
```shell
export offline_pass=<TOKEN>
cargo run --bin offline 'turn off bedroom lights'
```

### Secure
```shell
cargo run --bin secure <TOKEN>
```

### Signals
```shell
cargo run --bin signal AAPL 100
```
