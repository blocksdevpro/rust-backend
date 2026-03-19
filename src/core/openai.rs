use crate::config::Config;
use async_openai::{
    Client,
    config::OpenAIConfig,
    error::OpenAIError,
    types::chat::{
        ChatCompletionRequestMessageContentPartImage, ChatCompletionRequestMessageContentPartText,
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, ImageDetail, ImageUrl, ResponseFormat,
    },
};

pub fn create_openai_client(config: &Config) -> Client<OpenAIConfig> {
    let openai_config = OpenAIConfig::default();

    Client::with_config(openai_config)
}
pub fn build_chat_completion_request(
    system_prompt: String,
    user_prompt: String,
    image_content: String,
) -> Result<CreateChatCompletionRequest, OpenAIError> {
    CreateChatCompletionRequestArgs::default()
        .model("google/gemini-3.1-flash-lite-preview")
        .response_format(ResponseFormat::JsonObject)
        .messages(vec![
            ChatCompletionRequestSystemMessage::from(system_prompt).into(),
            ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Array(vec![
                    ChatCompletionRequestMessageContentPartText { text: user_prompt }.into(),
                    ChatCompletionRequestMessageContentPartImage {
                        image_url: ImageUrl {
                            url: image_content,
                            detail: Some(ImageDetail::High),
                        },
                    }
                    .into(),
                ]),
                name: None,
            }
            .into(),
        ])
        .build()
}
