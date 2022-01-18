# pr0t0n-orchestrator

Simplify orchestration of complex robotics systems that rely on low-latency AI processing at edge.

## Setup

You must specify your Postgres connection using the `.env` file.

```py
# Note: do NOT submit your username/password changes.
DATABASE_URL=postgres://USERNAME:PASSWORD@HOST/DATABASE_NAME
```

## Commands

To run the server:

```
cargo run --package pr0t0n_orch
```

To run the test client:

```
cargo run --package pr0t0n_orch_client
```

To run websocket loadtest:

```
cargo test --package pr0t0n_orch --test websocket_loadtest -- --nocapture
```

To test the database:

```
cargo test --package pr0t0n_orch_db -- --test-threads 1
```
