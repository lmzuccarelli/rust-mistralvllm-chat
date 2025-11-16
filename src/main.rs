use crate::cli::schema::{ApplicationConfig, Cli};
use anyhow::Result;
use clap::Parser;
use custom_logger as log;
use mistralrs::{
    IsqType, MemoryGpuConfig, PagedAttentionMetaBuilder, PagedCacheType, TextMessageRole,
    TextMessages, TextModelBuilder,
};
use std::fs;
use std::io;
use std::io::Write;
use std::str::FromStr;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    // Read config
    let config_data = fs::read_to_string(&args.config)?;
    let cfg: ApplicationConfig = serde_json::from_str(&config_data)?;

    // Setup logging
    let log_level = log::LevelFilter::from_str(&args.loglevel)?;
    log::Logging::new().with_level(log_level).init()?;

    log::info!("[main] application : {}", env!("CARGO_PKG_NAME"));
    log::info!("[main] author      : {}", env!("CARGO_PKG_AUTHORS"));
    log::info!("[main] version     : {}", env!("CARGO_PKG_VERSION"));
    println!();
    log::info!("[main] using mistral.rs paged attention (vLLM) for Rust");
    log::info!("[main] welcome!! input your question at the prompt");
    println!();
    log::info!("menu :");
    log::info!("     : type 'exit' to quit");
    println!();

    let isq = match cfg.spec.isq.as_str() {
        "Q4_0" => IsqType::Q4_0,
        "Q4_1" => IsqType::Q4_1,
        "Q5_0" => IsqType::Q5_0,
        "Q5_1" => IsqType::Q5_1,
        "Q8_0" => IsqType::Q8_0,
        "Q8_1" => IsqType::Q8_1,
        "Q2K" => IsqType::Q2K,
        "Q3K" => IsqType::Q3K,
        "Q4K" => IsqType::Q4K,
        "Q5K" => IsqType::Q5K,
        "Q6K" => IsqType::Q6K,
        "Q8K" => IsqType::Q8K,
        "HQQ8" => IsqType::HQQ8,
        "HQQ4" => IsqType::HQQ4,
        "F8E4M3" => IsqType::F8E4M3,
        "AFQ8" => IsqType::AFQ8,
        "AFQ6" => IsqType::AFQ6,
        "AFQ4" => IsqType::AFQ4,
        "AFQ3" => IsqType::AFQ3,
        "AFQ2" => IsqType::AFQ2,
        &_ => IsqType::Q8_0,
    };

    let model = TextModelBuilder::new(cfg.clone().spec.model)
        .with_isq(isq)
        .with_logging()
        .with_paged_attn(|| {
            PagedAttentionMetaBuilder::default()
                .with_block_size(cfg.spec.block_size)
                .with_gpu_memory(MemoryGpuConfig::ContextSize(cfg.spec.context_size))
                .with_paged_cache_type(PagedCacheType::Auto)
                .build()
        })?
        .build()
        .await?;

    loop {
        print!("prompt> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            println!("[main] exiting session");
            break;
        }

        let messages = TextMessages::new()
            .add_message(TextMessageRole::System, cfg.spec.system_message.clone())
            .add_message(TextMessageRole::User, input.clone());

        let response = model.send_chat_request(messages).await?;
        println!("{}", response.choices[0].message.content.as_ref().unwrap());
        dbg!(
            response.usage.avg_prompt_tok_per_sec,
            response.usage.avg_compl_tok_per_sec
        );
        println!();
    }

    Ok(())
}
