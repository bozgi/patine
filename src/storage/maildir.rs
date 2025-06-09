use std::path::Path;
use std::process;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::warn;

pub static MAILDIR_ROOT: OnceLock<String> = OnceLock::new();
pub static DOMAIN: OnceLock<String> = OnceLock::new();


pub async fn check_maildir(user: &str) -> std::io::Result<()> {
    let base = Path::new("maildir")
        .join(user)
        .join("Maildir");

    for subdir in ["tmp", "new", "cur"] {
        let dir = base.join(subdir);
        if !dir.exists() {
            warn!("Creating missing maildir subdir: {:?}", dir);
            fs::create_dir_all(&dir).await?;
        }
    }

    Ok(())
}

pub async fn write_to_maildir(user: &str, mail: &[u8]) -> std::io::Result<()> {
    let maildir = Path::new("maildir")
        .join(user)
        .join("Maildir");

    let filename = generate_mail_filename("local");
    let tmp_path = maildir.join("tmp").join(&filename);
    let new_path = maildir.join("new").join(&filename);

    let mut file = File::create(&tmp_path).await?;
    file.write_all(mail).await?;
    file.sync_all().await?;

    fs::rename(&tmp_path, &new_path).await?;

    Ok(())
}

fn generate_mail_filename(hostname: &str) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!(
        "{}.M{}.P{}.Q{}.{}",
        now.as_secs(),
        now.subsec_micros(),
        process::id(),
        rand::random::<u64>(),
        hostname
    )
}
