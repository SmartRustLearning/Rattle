#![deny(rust_2018_idioms)]
use crate::sandbox::Sandbox;

use std::default::Default;
mod sandbox;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use std::{
    convert::TryFrom,
    net::SocketAddr,
    path::{Path, PathBuf},
};

const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 5000;
pub const PLAYGROUND_GITHUB_TOKEN: &str = "PLAYGROUND_GITHUB_TOKEN";
pub const PLAYGROUND_UI_ROOT: &str = "PLAYGROUND_UI_ROOT";

#[tokio::main]
async fn main() {

    // Enable info-level logging by default. env_logger's default is error only.
    let env_logger_config = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env_logger_config).init();

    // let config = Config::from_env();
    // server_axum::serve(config);
    let sandbox = Sandbox::new().await.context(SandboxCreationSnafu).unwrap();
    let req = sandbox::CompileRequest::default();
    let res = sandbox.compile(&req);
    
    println!("{:?}", res.await);
}

        } else {
            println!("client disconnected");
            return;
        }
    }

    loop {
        if socket
            .send(Message::Text(String::from("Hi!")))
            .await
            .is_err()
        {
            println!("client disconnected");
            return;
        }
        println!("tick");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Sandbox creation failed: {}", source))]
    SandboxCreation { source: sandbox::Error },
    #[snafu(display("Compilation operation failed: {}", source))]
    Compilation { source: sandbox::Error },
    #[snafu(display("Execution operation failed: {}", source))]
    Execution { source: sandbox::Error },
    #[snafu(display("Evaluation operation failed: {}", source))]
    Evaluation { source: sandbox::Error },
    #[snafu(display("Linting operation failed: {}", source))]
    Linting { source: sandbox::Error },
    #[snafu(display("Expansion operation failed: {}", source))]
    Expansion { source: sandbox::Error },
    #[snafu(display("Formatting operation failed: {}", source))]
    Formatting { source: sandbox::Error },
    #[snafu(display("Interpreting operation failed: {}", source))]
    Interpreting { source: sandbox::Error },
    #[snafu(display("Caching operation failed: {}", source))]
    Caching { source: sandbox::Error },
    #[snafu(display("Gist creation failed: {}", source))]
    GistCreation { source: octocrab::Error },
    #[snafu(display("Gist loading failed: {}", source))]
    GistLoading { source: octocrab::Error },
    #[snafu(display("Unable to serialize response: {}", source))]
    Serialization { source: serde_json::Error },
    #[snafu(display("The value {:?} is not a valid target", value))]
    InvalidTarget { value: String },
    #[snafu(display("The value {:?} is not a valid assembly flavor", value))]
    InvalidAssemblyFlavor { value: String },
    #[snafu(display("The value {:?} is not a valid demangle option", value))]
    InvalidDemangleAssembly { value: String },
    #[snafu(display("The value {:?} is not a valid assembly processing option", value))]
    InvalidProcessAssembly { value: String },
    #[snafu(display("The value {:?} is not a valid channel", value,))]
    InvalidChannel { value: String },
    #[snafu(display("The value {:?} is not a valid mode", value))]
    InvalidMode { value: String },
    #[snafu(display("The value {:?} is not a valid edition", value))]
    InvalidEdition { value: String },
    #[snafu(display("The value {:?} is not a valid crate type", value))]
    InvalidCrateType { value: String },
    #[snafu(display("No request was provided"))]
    RequestMissing,
    #[snafu(display("The cache has been poisoned"))]
    CachePoisoned,
}
