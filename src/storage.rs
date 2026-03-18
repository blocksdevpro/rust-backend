use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3 as s3;

use crate::config::Config;

pub async fn create_r2_client(config: &Config) -> s3::Client {
    let creds = s3::config::Credentials::new(
        &config.cf_access_key,
        &config.cf_access_secret,
        None,
        None,
        "R2",
    );

    let aws_config = aws_sdk_s3::config::Builder::new()
        .behavior_version(BehaviorVersion::latest())
        .endpoint_url(format!(
            "https://{}.r2.cloudflarestorage.com",
            &config.cf_account_id
        ))
        .credentials_provider(creds)
        .region(Region::new("auto"))
        .build();

    s3::Client::from_conf(aws_config)
}
