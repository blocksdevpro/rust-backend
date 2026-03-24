use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    // google oauth2 credentials;
    pub google_client_id: String,
    pub google_client_secret: String,

    // google oauth2 uris;
    pub google_redirect_uri: String,

    // jwt credentials;
    pub jwt_secret: String,
    pub jwt_expiration: u32,

    // openai credentials;
    pub openai_api_key: String,
    pub openai_base_url: String,

    // cloudflare;
    pub cf_r2_bucket: String,
    pub cf_account_id: String,
    pub cf_access_key: String,
    pub cf_access_secret: String,
    pub cf_r2_public_hash: String,

    // database credentials;
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            google_client_id: env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not found"),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .expect("GOOGLE_CLIENT_SECRET not found"),

            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI")
                .expect("GOOGLE_REDIRECT_URI not found"),

            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not found"),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .expect("JWT_EXPIRATION not found")
                .parse()
                .expect("JWT_EXPIRATION is not a valid number"),

            openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found"),
            openai_base_url: env::var("OPENAI_BASE_URL").expect("OPENAI_BASE_URL not found"),

            cf_r2_bucket: env::var("CF_R2_BUCKET").expect("CF_R2_BUCKET not found"),
            cf_account_id: env::var("CF_ACCOUNT_ID").expect("CF_ACCOUNT_ID not found"),
            cf_access_key: env::var("CF_ACCESS_KEY").expect("CF_ACCESS_KEY not found"),
            cf_access_secret: env::var("CF_ACCESS_SECRET").expect("CF_ACCESS_SECRET not found"),
            cf_r2_public_hash: env::var("CF_R2_PUBLIC_HASH").expect("CF_R2_PUBLIC_HASH not found"),

            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not found"),
        }
    }
}
