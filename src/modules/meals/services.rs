use async_openai::{
    Client as OpenAIClient,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestMessageContentPartImage, ChatCompletionRequestMessageContentPartText,
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse, ImageDetail, ImageUrl,
        ResponseFormat,
    },
};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{self as aws3, operation::put_object::PutObjectOutput};
use axum::{Json, body::Bytes};
use uuid::Uuid;

use crate::{
    core::prompts,
    error::AppError,
    modules::{
        meals::{
            models::{MealResponse, ScanMealAIResponse},
            repo::MealRepository,
        },
        profiles::{models::TargetsResponse, repo::ProfileRepository},
    },
    state::AppState,
};

pub struct OpenAIService {
    client: OpenAIClient<OpenAIConfig>,
}

impl OpenAIService {
    pub fn new(api_key: &str, base_url: &str) -> Self {
        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(base_url);
        let client = OpenAIClient::with_config(config);

        Self { client }
    }

    pub fn build_request(
        &self,
        model: &str,
        system_prompt: String,
        user_prompt: String,
        image_url: String,
    ) -> Result<CreateChatCompletionRequest, AppError> {
        CreateChatCompletionRequestArgs::default()
            .model(model)
            .response_format(ResponseFormat::JsonObject)
            .messages(vec![
                ChatCompletionRequestSystemMessage::from(system_prompt).into(),
                ChatCompletionRequestUserMessage {
                    content: ChatCompletionRequestUserMessageContent::Array(vec![
                        ChatCompletionRequestMessageContentPartText { text: user_prompt }.into(),
                        ChatCompletionRequestMessageContentPartImage {
                            image_url: ImageUrl {
                                url: image_url,
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
            .map_err(|e| {
                tracing::error!("Failed to build chat completion request, {}", e);
                AppError::InternalServerError(Some(
                    "Failed to build chat completion request".to_string(),
                ))
            })
    }

    pub async fn chat_completion(
        &self,
        request: CreateChatCompletionRequest,
    ) -> Result<CreateChatCompletionResponse, AppError> {
        let response = self.client.chat().create(request).await.map_err(|e| {
            tracing::error!("Failed to create chat completion, {}", e);
            AppError::InternalServerError(Some("Failed to create chat completion".to_string()))
        })?;
        Ok(response)
    }
}

pub struct StorageService {
    client: aws3::Client,
    bucket: String,
    public_url: String,
}

impl StorageService {
    pub fn new(
        access_key: &str,
        secret_key: &str,
        account_id: &str,
        bucket_name: &str,
        public_url: &str,
    ) -> Self {
        let creds = aws3::config::Credentials::new(access_key, secret_key, None, None, "R2");

        let config = aws3::config::Builder::new()
            .behavior_version(BehaviorVersion::latest())
            .endpoint_url(format!("https://{}.r2.cloudflarestorage.com", account_id))
            .credentials_provider(creds)
            .region(Region::new("auto"))
            .build();

        let client = aws3::Client::from_conf(config);

        Self {
            client,
            bucket: bucket_name.to_string(),
            public_url: public_url.to_string(),
        }
    }

    pub async fn store(&self, path: &str, contents: Bytes) -> Result<PutObjectOutput, AppError> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(path)
            .body(contents.into())
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to upload object to R2 bucket, {}", e);
                AppError::InternalServerError(Some(
                    "Failed to store image inside r2 bucket.".to_string(),
                ))
            })
    }

    pub fn get_public_url(&self, path: &str) -> String {
        format!("{}/{}", self.public_url, path)
    }
}

pub struct MealService {
    repo: MealRepository,
    profile: ProfileRepository,
    openai: OpenAIService,
    storage: StorageService,
}

impl MealService {
    pub fn new(state: AppState) -> Self {
        Self {
            repo: MealRepository::new(state.pool.clone()),
            profile: ProfileRepository::new(state.pool.clone()),
            openai: OpenAIService::new(&state.config.openai_api_key, &state.config.openai_base_url),
            storage: StorageService::new(
                &state.config.cf_access_key,
                &state.config.cf_access_secret,
                &state.config.cf_account_id,
                &state.config.cf_r2_bucket,
                &state.config.cf_r2_public_hash,
            ),
        }
    }

    pub async fn get_meal(&self, id: Uuid, user_id: Uuid) -> Result<Json<MealResponse>, AppError> {
        let meal = self.repo.find_by_user_id(id, user_id).await?;

        meal.map_or(
            Err(AppError::ItemNotFound(Some("Meal not found".to_string()))),
            |meal| Ok(Json(MealResponse::from(meal))),
        )
    }

    pub async fn get_meals(&self, user_id: Uuid) -> Result<Json<Vec<MealResponse>>, AppError> {
        let meals = self.repo.find_all_by_user_id(user_id).await?;

        Ok(Json(meals.into_iter().map(MealResponse::from).collect()))
    }

    pub async fn scan_meal(
        &self,
        user_id: Uuid,
        image: Bytes,
        extension: String,
    ) -> Result<Json<ScanMealAIResponse>, AppError> {
        // Get user profile and targets.
        let profile = self.profile.find_by_user_id(user_id).await?;
        let target = profile.map_or(None, |p| Some(TargetsResponse::from(p)));

        // Store image in R2 bucket and request chat completion from OpenAI.
        let path = format!("meals/{}/{}.{}", user_id, Uuid::new_v4(), extension);
        let image_url = self.storage.get_public_url(&path);
        let system_prompt = prompts::SCAN_SYSTEM_PROMPT.to_string();
        let user_prompt = prompts::build_user_scan_prompt(None, target.as_ref());

        let completion_request = self.openai.build_request(
            "google/gemini-3.1-flash-lite-preview",
            system_prompt,
            user_prompt,
            image_url,
        )?;

        let store_handle = self.storage.store(&path, image);
        let completion_handle = self.openai.chat_completion(completion_request);

        let (store_result, completion_result) = tokio::join!(store_handle, completion_handle);

        let completion_result = completion_result?;

        let response = completion_result
            .choices
            .first()
            .and_then(|c| c.message.content.as_ref())
            .ok_or_else(|| {
                tracing::error!("Failed to get completion response.");
                AppError::InternalServerError(Some(
                    "Failed to get completion response.".to_string(),
                ))
            })?;
        let scan_response: ScanMealAIResponse = serde_json::from_str(response).map_err(|e| {
            tracing::error!("Failed to parse scan response, {}", e);
            AppError::InternalServerError(Some("Failed to parse scan response".to_string()))
        })?;

        Ok(Json(scan_response))
    }
}
