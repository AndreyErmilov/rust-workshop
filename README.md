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

## Run project

Run API
```bash
cargo run
```
Post events
```bash
./run.sh
```
View created event
```bash
curl localhost:8080/v1/users/42/events/
```

## Tasks
* Check with clippy and fix error
```cargo clippy --all-targets --all-features -- -D warnings```
* Add new event type – 112 (RefreshPage)
* Add new handler `v1/users/{user_id}/events/{event_type}/`

## Build project
```bash
cargo build --release
```
