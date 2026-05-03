//IDEA: scrivo delle procedure per il salvataggio su disco delle conversazioni in modo standardizzato 

use chrono::Local;
use std::fs;
use std::io;
use std::path::{PathBuf , Path};

pub fn conversation_mkdir() -> io::Result<PathBuf> {

    let timestamp = Local::now()
        .format("%d-%m-%Y_%H:%M:%S") //trait del tipo DateTime<Local> per formattare (anno-mese-giorno_ore-minuti-secondi)
        .to_string();

    let conversations_repository = Path::new("conversations");
    fs::create_dir_all(conversations_repository)?;

    let conversation_path = conversations_repository.join(format!("conversation_{}", timestamp));
    fs::create_dir_all(conversation_path.as_path())?;

    Ok(conversation_path)
}

pub fn write_and_save<P: AsRef<Path>>(path: P , conversation: &str) -> io::Result<()> {

    let path = path.as_ref(); //as_path() non usabile

    if let Some(parent) = path.parent() { //controlla se esistono le parent directories
        fs::create_dir_all(parent)?;
    }

    fs::write(path, conversation)
}