use std::path::{Path, PathBuf};
use std::process;
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::warn;

pub const MAILDIR_ROOT: &str = "maildir";

pub async fn check_maildir(maildir: &str) -> std::io::Result<()> {
    for subdir in ["tmp", "new", "cur"] {
        let dir = Path::new(MAILDIR_ROOT).join(maildir).join("Maildir").join(subdir);
        if !dir.exists() {
            warn!("maildir {} does not exist, creating...", maildir);
            fs::create_dir_all(dir).await?;
        }
    }
    Ok(())
}

pub async fn write_to_maildir(maildir: &Path, mail: &Vec<u8>) -> std::io::Result<()> {
    let maildir = Path::new(MAILDIR_ROOT).join(maildir).join("Maildir");
    let filename = generate_mail_filename("cwl");
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
    let secs = now.as_secs();
    let micros = now.subsec_micros();
    let pid = process::id();
    let unique = rand::random::<u64>();

    format!("{}.M{}.P{}.Q{}.{}", secs, micros, pid, unique, hostname)
}