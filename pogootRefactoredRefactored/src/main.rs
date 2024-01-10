mod services;

#[tokio::main]
async fn main() {
    services::corporate::Coordinator::start_all_services().await;
}

