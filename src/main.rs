use clap::Parser;
mod cli;

// use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
// use reqwest::Client;
// use reqwest_middleware::{ClientBuilder, Result};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let client = ClientBuilder::new(Client::new())
    //     .with(Cache(HttpCache {
    //         mode: CacheMode::Default,
    //         manager: CACacheManager::default(),
    //         options: HttpCacheOptions::default(),
    //     }))
    //     .build();

    cli::Cli::parse().command.run().await
}
