//IDEA: mi serve un tipo per l'input che raccolga l'argomento, i partecipanti e la durata della conversazione

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ConversationData {
    pub topic: String,
    pub turns: u32,
    pub models: Vec<String>,
}
