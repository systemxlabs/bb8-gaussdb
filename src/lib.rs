//! GaussDB support for the `bb8` connection pool.

#![deny(missing_docs, missing_debug_implementations)]

pub use bb8;
pub use tokio_gaussdb;

use std::fmt;
use std::str::FromStr;
use tokio_gaussdb::config::Config;
use tokio_gaussdb::tls::{MakeTlsConnect, TlsConnect};
use tokio_gaussdb::{Client, Error, Socket};

/// A `bb8::ManageConnection` for `tokio_gaussdb::Client`.
pub struct GaussDBConnectionManager<Tls: MakeTlsConnect<Socket>> {
    config: Config,
    tls: Tls,
}

impl<Tls: MakeTlsConnect<Socket> + Clone> GaussDBConnectionManager<Tls> {
    /// Creates a new `GaussDBConnectionManager` from the given GaussDB `Config`
    /// and TLS connector.
    pub fn new(config: Config, tls: Tls) -> Self {
        Self { config, tls }
    }

    /// Creates a new `GaussDBConnectionManager` from a stringlike connection
    /// parameter (e.g. `"host=localhost user=gaussdb"` or a URL).
    pub fn new_from_stringlike(params: impl ToString, tls: Tls) -> Result<Self, Error> {
        let params = params.to_string();
        let config = Config::from_str(&params)?;
        Ok(Self::new(config, tls))
    }
}

impl<Tls> fmt::Debug for GaussDBConnectionManager<Tls>
where
    Tls: MakeTlsConnect<Socket> + TlsConnect<Socket> + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GaussDBConnectionManager")
            .field("config", &self.config)
            .field("tls", &self.tls)
            .finish()
    }
}

impl<Tls> bb8::ManageConnection for GaussDBConnectionManager<Tls>
where
    Tls: MakeTlsConnect<Socket> + TlsConnect<Socket> + Clone + Send + Sync + 'static,
    <Tls as MakeTlsConnect<Socket>>::Stream: Send,
    <Tls as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<Tls as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    <Tls as TlsConnect<Socket>>::Future: Send,
    <Tls as TlsConnect<Socket>>::Stream: Send + Sync,
{
    type Connection = Client;
    type Error = Error;

    fn connect(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Connection, Self::Error>> + Send {
        let config = self.config.clone();
        let tls = self.tls.clone();
        async move {
            let (client, connection) = config.connect(tls).await?;
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("gaussdb connection error: {}", e);
                }
            });
            Ok(client)
        }
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.simple_query("").await.map(|_| ())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.is_closed()
    }
}
