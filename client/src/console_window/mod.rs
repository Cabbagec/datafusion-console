use std::time::Duration;

use gloo_timers::future::sleep;
use lazy_static::lazy_static;
use log::{error, info};
use tokio::sync::RwLock;

pub use app::ConsoleApp;

use super::client::Client;
use super::status::Mode;

mod app;
mod ui;

lazy_static! {
    pub static ref CURRENT_HOST: RwLock<String> = RwLock::new("".to_string());
}

pub async fn set_current_host(url: String) {
    for _ in 0..10 {
        if let Ok(mut current_url) = CURRENT_HOST.try_write() {
            *current_url = url;
            info!("current location: {current_url}");
            return;
        } else {
            error!("failed to set current url as {url}");
            sleep(Duration::from_millis(100)).await;
        }
    }

    error!("failed to set current url as {url}");
}

pub fn get_current_host() -> String {
    for _ in 0..10 {
        if let Ok(r) = CURRENT_HOST.try_read() {
            return r.to_string();
        } else {
            continue;
        }
    }

    error!("failed to get current url, return empty string");
    "".to_string()
}
