//IDEA: scrivo delle procedure per il salvataggio su disco delle conversazioni in modo standardizzato 

use chrono::Local; //modulo per la data e ora locale
use std::fs; //operazioni su filesystem
use std::io; //input-output
use std::io::prelude::*;
use std::path::{PathBuf , Path};

pub fn crea_directory_conversazione() -> io::Result<PathBuf> { //la funzione restituisce il buffer del path della directory o un errore

    let timestamp = Local::now() //salvo la data e ora corrente con la funzione now()
        .format("%Y-%m-%d_%H:%M:%S") //trait del tipo DateTime<Local> per formattare (anno-mese-giorno_ore-minuti-secondi)
        .to_string(); //trait per trasformare nel tipo String

    let path = Path::new("conversazioni"); //variabile di tipo Path (path della directory "conversazioni" che conterrà tutte le conversazioni) NOTA: Le variabili &Path esistono solo nella loro versione borrowed, perché hanno almeno un campo non noto a compile time e che quindi richiede allocazione di memoria a runtime. Ergo non possono possedere la loro memoria
    fs::create_dir_all(path)?; //crea directory dalla variabile "path" (e tutte le parent directories) se non esiste già, altrimenti non fa nulla. Se la funzione fallisce termina il programma (grazie a ?)

    let path_conversazione = path.join(format!("verbale_{}", timestamp)); //creo il buffer per il path della singola conversazione (quella corrente) con nome "conversazione_YYYY-MM-DD_HH:MM:SS
    fs::create_dir_all(path_conversazione.as_path())?; //crea directory dalla variabile "path_conversazione" (parsata a un &Path) etc etc... (come sopra)

    Ok(path_conversazione)
}

pub fn scrivi_conversazione<T: AsRef<Path>>(path:T , conversazione: &str) -> io::Result<()> { //la funzione accetta come path qualunque roba possa essere vista come un path (ovvero implementa AsRef<Path>), restituisce un void (dai Riccardo, lo sai che non è un void, è per capirsi, non essere pedante)

    let path = path.as_ref(); //qui stiamo trasformando in un &Path qualunque roba abbiamo gettato in pasto alla funzione come tipo di "path". Tocca usare il più generico as_ref() fidandoci che il compilatore inferisca che è un &Path perché non sa ancora se "path" implementa as_path()

    if let Some(parent) = path.parent() { //questo blocco controlla se esistono le directories "antenate" di quella individuata da path e se non ci sono le crea
        fs::create_dir_all(parent)?; //di base non dovrebbe servire nel programma ma non si sa mai che voglia usare la funzione da qualche altra parte
    }

    fs::write(path, conversazione) //e qui scriviamo la conversazione nel file
}