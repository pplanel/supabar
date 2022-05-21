use crate::{prisma, prisma::PrismaClient};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to connect to database")]
    MissingConnection,
    #[error("Unable find current_library in the client config")]
    MalformedConfig,
    #[error("Unable to initialize the Prisma client")]
    ClientError(#[from] prisma::NewClientError),
}

pub async fn create_connection(path: &str) -> anyhow::Result<PrismaClient, DatabaseError> {
    println!("Creating database connection: {:?}", path);
    let client = prisma::new_client_with_url(&format!("file:{}", &path)).await?;

    Ok(client)
}
