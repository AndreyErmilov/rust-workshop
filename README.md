# Avito Rust Workshop

## Install
### Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Redis
```bash
wget http://download.redis.io/redis-stable.tar.gz
tar xvzf redis-stable.tar.gz
cd redis-stable
make
```
### IDE
Download Rust plugin for [PyCharm](https://plugins.jetbrains.com/plugin/8182-rust)
### Project
```bash
git clone git@github.com:AndreyErmilov/rust-workshop.git
```

## Run project

Run API
```bash
cargo run
```
Create first event
```bash
curl -X POST localhost:8080/v1/events/ -H "content-type: application/json" -d @test-data/send-message.json
```
View created event
```bash
curl localhost:8080/v1/users/42/events/
```

## Build project
```bash
cargo build --release
```
