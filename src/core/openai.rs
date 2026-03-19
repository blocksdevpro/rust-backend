use crate::config::Config;
use async_openai::{Client, config::OpenAIConfig};

pub fn create_openai_client(config: &Config) -> Client<OpenAIConfig> {
    let openai_config = OpenAIConfig::default();

    Client::with_config(openai_config)
}
