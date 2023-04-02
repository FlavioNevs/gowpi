use gowpi_api::{handler::test_handler, api::Api};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = Api::new();

    api.router.get("/", Box::new(test_handler)).await;
    api.run("0.0.0.0", "19283").await?;

    Ok(())
}