use std::path::PathBuf;

use corrode::{run, AppState};

#[tokio::main]
async fn main() {
    let app_state = AppState {
        content_root: PathBuf::from("content"),
    };

    run(app_state).await;
}
