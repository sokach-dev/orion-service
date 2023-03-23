pub mod learn_word;
mod rpc;
mod story;
mod vocabulary;
mod word_list;

use abi::orion_service_server::OrionServiceServer;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct ModelService {
    pool: PgPool,
}

impl ModelService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
    pub async fn from_config(config: &abi::DbConfig) -> Result<Self, sqlx::Error> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;
        Ok(Self::new(pool))
    }
}

#[derive(Debug, Clone)]
pub struct OrionService {
    model: ModelService,
}

impl OrionService {
    pub async fn from_config(config: &abi::DbConfig) -> Result<Self, sqlx::Error> {
        Ok(Self {
            model: ModelService::from_config(config).await?,
        })
    }
}

pub async fn start_server(config: &abi::Config) -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = format!("{}:{}", config.host, config.port).parse()?;

    let svc = OrionService::from_config(&config.db_config).await?;
    let orion_svc = OrionServiceServer::new(svc.clone());

    tracing::info!("Listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(orion_svc)
        .serve(addr)
        .await?;
    Ok(())
}
