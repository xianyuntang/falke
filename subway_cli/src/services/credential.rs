use std::error::Error;
use std::path::Path;
use std::sync::LazyLock;
use tokio::fs;

static TOKEN_PATH: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{}/.subway/.token",
        dirs::home_dir().unwrap().to_str().unwrap()
    )
});

pub async fn read_token() -> Result<String, Box<dyn Error>> {
    if let None = Path::new(&TOKEN_PATH.as_str()).parent() {
        tracing::error!("You must sign in first.");
        panic!()
    }
    let access_token = String::from_utf8(fs::read(&TOKEN_PATH.as_str()).await?)?;
    Ok(access_token)
}

pub async fn write_token(token: String) -> Result<(), Box<dyn Error>> {
    if let Some(path) = Path::new(&TOKEN_PATH.as_str()).parent() {
        fs::create_dir_all(path).await?;
    }

    fs::write(&TOKEN_PATH.as_str(), token).await?;

    Ok(())
}
