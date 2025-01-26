use handler::UserServiceImpl;
use serde::Deserialize;
use sqlx::pool::PoolOptions;
use sqlx::Postgres;
use std::sync::Arc;
use tonic::transport::Server;
use user_proto::user::v1::user_service_server::UserServiceServer;
use user_proto::user::v1::FILE_DESCRIPTOR_SET;

mod handler;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "String::default")]
    pub port: String,
    #[serde(default = "String::default")]
    pub reader_database_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config =
        envy::from_env::<Config>().map_err(|e| anyhow::anyhow!("failed to load config: {}", e))?;

    let reader_pool = PoolOptions::<Postgres>::new()
        .max_connections(10)
        .connect(&config.reader_database_url)
        .await
        .map_err(|e| anyhow::anyhow!("failed to connect to database: {e}"))?;

    // query_users_by_ids
    let query_users_by_ids_repository =
        Arc::new(user_repository::query_users_by_ids::QueryUsersByIDs::new());
    let query_users_by_ids_usecase =
        Arc::new(user_usecase::query_users_by_ids::QueryUsersByIDs::new(
            reader_pool.clone(),
            query_users_by_ids_repository,
        ));

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;
    let user_service = UserServiceImpl::new(query_users_by_ids_usecase);

    let addr = format!("0.0.0.0:{}", config.port);
    Server::builder()
        .add_service(reflection_service)
        .add_service(UserServiceServer::new(user_service))
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
