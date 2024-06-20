#[tokio::main]
async fn main() {
    env_logger::builder()
        .parse_default_env()
        .format_target(false)
        .format_module_path(false)
        .init();

}
