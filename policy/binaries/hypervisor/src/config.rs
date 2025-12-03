use std::{net::SocketAddr, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub executor_path: PathBuf,
    pub app_path: PathBuf,
    pub listening: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            executor_path: "./data/executor".parse().expect("executor path"),
            app_path: "./data/apps".parse().expect("app path"),
            listening: "0.0.0.0:3000".parse().expect("hypervisor listen address"),
        }
    }
}
