#[async_std::main]
async fn main() -> tide::Result<()> {
	let mut app = tide::new();
	app.at("/").post(tide_challenge::handle);
	app.listen("127.0.0.1:8000").await?;
	Ok(())
}