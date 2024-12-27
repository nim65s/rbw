use crate::service;
use std::future::pending;
use zbus::connection;

pub struct State {
    _collections: Vec<String>,
}

pub struct Agent {
    _state: std::sync::Arc<tokio::sync::Mutex<State>>,
}

impl Agent {
    pub fn new() -> anyhow::Result<Self> {
        let _config = rbw::config::Config::load()?;
        Ok(Self {
            _state: std::sync::Arc::new(tokio::sync::Mutex::new(State {
                _collections: Vec::new(),
            })),
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let service = service::Service {
            collections: Vec::new(),
        };

        let _service = connection::Builder::session()?
            .name("org.freedesktop.Secret")?
            .serve_at("/org/freedesktop/secrets", service)?
            .build()
            .await?;

        // Do other things or go to wait forever
        pending::<()>().await;

        Ok(())
    }
}
