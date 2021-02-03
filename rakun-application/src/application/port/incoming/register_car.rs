use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait RegisterCarUseCase {
    async fn register_car(&self) -> Result<()>;
}
