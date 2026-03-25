//IDEA: mi serve un tipo per l'input che raccolga l'argomento, i partecipanti e la durata della conversazione

use serde::Serialize;

#[derive(Serialize)]
#[derive(Debug)] //e ce lo mettiamo pure qui Debug
pub struct Discussione {
    pub argomento: String,
    pub turni: u32,
    pub partecipanti: Vec<String>,
}