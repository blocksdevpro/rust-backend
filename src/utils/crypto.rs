use rustls::crypto::ring::default_provider;

pub fn setup_crypto() -> anyhow::Result<()> {
    default_provider()
        .install_default()
        .map_err(|_| anyhow::anyhow!("Failed to install default crypto provider"))
}
