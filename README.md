# lightning-nodes

A tiny Rust service that imports Lightning Network node rankings from mempool.space, stores them in SQLite, and exposes the list through an HTTP endpoint.

---

## Build tools & versions used

* **Rust**: 1.86.0 (stable)
* **Cargo**: 1.86.0
* **actix-web**: 4
* **tracing**: 0.1
* **tracing-subscriber**: 0.3
* **tokio**: 1 (rt-multi-thread, macros, time)
* **tokio-util**: 0.7
* **reqwest**: 0.12.15 (json, rustls-tls)
* **serde / serde\_json**: 1.0
* **sqlx (sqlite)**: 0.8.5
* **chrono**: 0.4
* **dotenvy**: 0.15
* **anyhow**: 1.0.98
* **envy**: 0.4.2
* **async-trait**: 0.1.88

---

## Steps to run the app

```bash
# clone repository
git clone https://github.com/your-user/lightning-nodes.git
cd lightning-nodes

# configure environment variables
# copy .env.example to .env and adjust if needed
cp .env.example .env

# run server (creates nodes.db, 60s import interval)
cargo run --bin server

# test endpoints
curl http://localhost:8080/healthz
curl http://localhost:8080/nodes

# run tests
cargo test
```

## Configuration

Create a `.env` file at the project root with the following default values (or export them in your shell):

```env
DATABASE_URL=sqlite://nodes.db
POLL_INTERVAL_SECS=60
RUST_LOG=info
```

---

## What was the reason for your focus? What problems were you trying to solve?

The goal was to build an idiomatic asynchronous Rust microservice that combines periodic background tasks, reliable database persistence, a simple REST API interface, and comprehensive error handling without any runtime panics.

---

## How long did you spend on this project?

≈ 10 hours (development, testing, documentation).

---

## Did you make any trade-offs for this project? What would you have done differently with more time?

* Code-driven table creation instead of setting up formal migrations.
* Row-by-row inserts instead of implementing a bulk upsert.
* Skipped pagination, metrics.

---

## What do you think is the weakest part of your project?

* No built-in metrics to track performance or error rates.

---

## Is there any other information you’d like us to know?

* All production paths are panic-free (Clippy `-D warnings`).
* Dependencies are audited (`cargo audit`).
* Graceful shutdown on Ctrl-C/SIGTERM.
