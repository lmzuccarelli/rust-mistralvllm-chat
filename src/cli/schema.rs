use clap::Parser;
use serde_derive::{Deserialize, Serialize};

/// rust-container-tool cli struct
#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A chat service based on Rust implementation of paged attention (vLLM)", long_about = None)]
#[command(
    help_template = "{author-with-newline} {about-section}Version: {version} \n {usage-heading} {usage} \n {all-args} {tab}"
)]

pub struct Cli {
    /// config file to use
    #[arg(short, long, value_name = "config")]
    pub config: String,

    /// set the loglevel. Valid arguments are info, debug, trace
    #[arg(value_enum, long, value_name = "loglevel", default_value = "info")]
    pub loglevel: String,
}

/// Application configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationConfig {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub spec: Spec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spec {
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "block_size")]
    pub block_size: usize,
    #[serde(rename = "context_size")]
    pub context_size: usize,
    /// refer  to the in situ quantization section (mistralrs_quant) for more details
    #[serde(rename = "isq")]
    pub isq: String,
    #[serde(rename = "system_message")]
    pub system_message: String,
}
