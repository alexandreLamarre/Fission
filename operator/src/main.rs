use operator::run_controller;

#[tokio::main]
pub async fn main() {
    run_controller().await;
}
