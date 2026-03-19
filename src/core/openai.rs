use crate::config::Config;
use async_openai::{
    Client,
    config::OpenAIConfig,
    error::OpenAIError,
    types::chat::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPartImage,
        ChatCompletionRequestMessageContentPartText, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, ChatCompletionRequestUserMessageContentPart,
        CreateChatCompletionRequest, CreateChatCompletionRequestArgs, ImageDetail, ImageUrl,
        ResponseFormat,
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
    let request = CreateChatCompletionRequestArgs::default()
        .model("openai/gpt-4o-mini")
        .response_format(ResponseFormat::JsonObject)
        .messages(vec![
            ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                content: ChatCompletionRequestSystemMessageContent::Text(system_prompt),
                name: None,
            }),
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Array(vec![
                    ChatCompletionRequestUserMessageContentPart::Text(
                        ChatCompletionRequestMessageContentPartText { text: user_prompt },
                    ),
                    ChatCompletionRequestUserMessageContentPart::ImageUrl(
                        ChatCompletionRequestMessageContentPartImage {
                            image_url: ImageUrl {
                                url: image_content,
                                detail: Some(ImageDetail::High),
                            },
                        },
                    ),
                ]),
                name: None,
            }),
        ])
        .build()?;

    Ok(request)
}
