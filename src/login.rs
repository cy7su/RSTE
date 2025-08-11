use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::{self, Write};
use zip::ZipArchive;
use anyhow::{Result, Context};

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

fn restore_steam_session(zip_path: &Path) -> Result<bool> {
    kill_process("Steam.exe");
    
    let steam_path = get_steam_path();
    
    if !steam_path.exists() {
        println!("Папка Steam не найдена: {:?}", steam_path);
        return Ok(false);
    }

    if !zip_path.exists() {
        println!("Файл архива не найден: {:?}", zip_path);
        return Ok(false);
    }
    
    let file = fs::File::open(zip_path)
        .with_context(|| format!("Failed to open zip file: {:?}", zip_path))?;
    
    let mut archive = ZipArchive::new(file)
        .with_context(|| "Failed to read zip archive")?;
    
    let mut has_config = false;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .with_context(|| format!("Failed to get file at index {}", i))?;
        
        let name = file.name().to_string();
        
        if name.starts_with("ssfn") {
            let outpath = steam_path.join(&name);
            let mut outfile = fs::File::create(&outpath)
                .with_context(|| format!("Failed to create file: {:?}", outpath))?;
            
            io::copy(&mut file, &mut outfile)
                .with_context(|| format!("Failed to extract file: {}", name))?;
            
            println!("Восстановлен: {}", name);
        } else if name.starts_with("config/") {
            has_config = true;
        }
    }
    
    if has_config {
        let file = fs::File::open(zip_path)
            .with_context(|| format!("Failed to open zip file: {:?}", zip_path))?;
        
        let mut archive = ZipArchive::new(file)
            .with_context(|| "Failed to read zip archive")?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .with_context(|| format!("Failed to get file at index {}", i))?;
            
            let name = file.name().to_string();
            let outpath = steam_path.join(&name);
            
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {:?}", parent))?;
            }
            
            let mut outfile = fs::File::create(&outpath)
                .with_context(|| format!("Failed to create file: {:?}", outpath))?;
            
            io::copy(&mut file, &mut outfile)
                .with_context(|| format!("Failed to extract file: {}", name))?;
        }
        
        println!("Конфиги восстановлены");
    }
    
    let steam_exe = steam_path.join("Steam.exe");
    if steam_exe.exists() {
        Command::new(&steam_exe)
            .spawn()
            .with_context(|| "Failed to start Steam")?;
        
        println!("Steam запущен с восстановленной сессией");
        Ok(true)
    } else {
        println!("Steam.exe не найден");
        Ok(false)
    }
}

fn main() -> Result<()> {
    print!("Введи путь до zip: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let zip_file = input.trim();
    
    if restore_steam_session(Path::new(zip_file))? {
        println!("Вход выполнен успешно");
    } else {
        println!("Не удалось войти в аккаунт");
    }
    
    Ok(())
}
