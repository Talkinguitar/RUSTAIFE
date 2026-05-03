//IDEA DI BASE: costruisco un sistema di interpretazione degli input utente che registri:
//1-)il topic di discussione, 2-)il numero di turni di discussione, 3-)i modelli LLM coinvolti nella discussione

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug)]
pub struct Interfaccia {
    #[arg(short, long)]
    pub topic: String,
    #[arg(short, long, default_value = "1")]
    pub turns: u32,
    #[arg(short, long, value_delimiter = ',')]
    pub models: Vec<String>,
}
