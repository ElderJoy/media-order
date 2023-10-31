mod errors;
mod local;

use crate::errors::Result;

#[tokio::main]
async fn main() -> Result<()> {
	local::create_database().await?;
	Ok(())
}
