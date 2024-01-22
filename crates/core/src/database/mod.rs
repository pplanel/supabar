use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to connect to database")]
    MissingConnection,
    #[error("Unable find current_library in the client config")]
    MalformedConfig,
    #[error("Unable to initialize the Prisma client")]
    ClientError
}

pub async fn create_connection(path: &str) -> anyhow::Result<(), DatabaseError> {
    println!("Creating database connection: {:?}", path);
    Ok(())
}
