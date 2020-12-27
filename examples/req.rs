use rakun::request::Request;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    fuck().await
}

async fn fuck() -> Result<()> {
    let req = Request::new("http://httpbin.org/ip");
    req.execute().await;

    Ok(())
}
