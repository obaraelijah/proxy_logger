use std::env;

use clap::Parser;
use proxy_logger::Arguments;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let arguments = Arguments::parse();

    env::set_var("RUST_LOG", arguments.level.to_string());
    env_logger::builder()
        .parse_default_env()
        .format_target(false)
        .format_module_path(false)
        .init();

}
