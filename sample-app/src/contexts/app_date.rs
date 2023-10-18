use async_trait::async_trait;

#[async_trait]
pub trait AppDateContext<T: AppDateService> {
    async fn get(&self) -> T;
}
