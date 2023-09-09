use errors::AppError;

mod errors;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Ok(())
}
