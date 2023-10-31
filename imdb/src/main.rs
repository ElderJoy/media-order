mod errors;
mod local;

use crate::errors::Result;

#[tokio::main]
async fn main() -> Result<()> {
	let res = local::create_database("local_imdb").await?;
	dbg!(&res);
	Ok(())
}
