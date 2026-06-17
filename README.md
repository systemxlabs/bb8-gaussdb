# bb8-gaussdb

[![Crates.io](https://img.shields.io/crates/v/bb8-gaussdb.svg)](https://crates.io/crates/bb8-gaussdb)
[![Documentation](https://docs.rs/bb8-gaussdb/badge.svg)](https://docs.rs/bb8-gaussdb/)
[![CI](https://github.com/systemxlabs/bb8-gaussdb/workflows/CI/badge.svg)](https://github.com/systemxlabs/bb8-gaussdb/actions?query=workflow%3ACI)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

Full-featured async (tokio-based) connection pool for [GaussDB](https://en.wikipedia.org/wiki/GaussDB) / [openGauss](https://opengauss.org/), built on [bb8](https://github.com/djc/bb8) and [tokio-gaussdb](https://crates.io/crates/tokio-gaussdb).

## Usage

```rust
use bb8_gaussdb::GaussDBConnectionManager;
use tokio_gaussdb::tls::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = GaussDBConnectionManager::new_from_stringlike(
        "host=localhost user=gaussdb",
        NoTls,
    )?;

    let pool = bb8_gaussdb::bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await?;

    let conn = pool.get().await?;
    // use the connection — it will be returned to the pool when it drops

    Ok(())
}
```

## Features

| Feature | Description |
|---------|-------------|
| `with-serde_json-1` | Enable `serde_json` support via `tokio-gaussdb` |
| `with-chrono-0_4` | Enable `chrono` support via `tokio-gaussdb` |
| `with-uuid-1` | Enable `uuid` support via `tokio-gaussdb` |

## License

MIT
