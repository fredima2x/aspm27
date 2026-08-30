// !!!
#[tracing::instrument]
pub async fn hi() -> &'static str {
    tracing::info!("Got hi Request.");
    "Leck Eier!"
}
