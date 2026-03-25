//IDEA DI BASE: costruisco un sistema di interpretazione degli input utente (una Command Line Interface) che registri:
//1-)il topic di discussione, 2-)il numero di turni di discussione, 3-)i modelli LLM coinvolti nella discussione
//poi questa roba viene data in pasto a una struct di tipo Discussione. Più chiaro che mettere tutto in "Discussione"

use clap::Parser; //importo il trait in clap per fare il parsing (ricontrollare come è definito il trait Parser e cosa fa esattamente)
use serde::{Deserialize, Serialize}; //importo i trait Serialize e Deserialize (ricontrollare come sono definiti)(servono a "costruire" (e "decostruire") oggetti a partire dalla (o arrivando a) loro rappresentazione in particolari formati (come JSON))

//Costruisco la struct pubblica (in modo che sia visibile nel main) "CLI" con i campi {argomento, turni, modelli)
#[derive(Parser, Serialize, Deserialize, Debug)] //tramite "derive" chiedo di implementare i trait elencati per "CLI" ("derive" genera del codice standard che fa come spiegato)
pub struct Cli {
    #[arg(long)] //così se il metodo .parse() incontra la flag lunga "--argomento" nella command line sa che deve mettere il valore successivo nel campo "argomento"
    pub argomento: String, //"argomento" è una String
    #[arg(long)] //come sopra
    pub turni: u32, //"turni" deve essere un intero (32 bit) unsigned
    #[arg(long)] //come sopra
    pub modelli: Vec<String> //"modelli" deve essere un vettore (dinamico) di stringhe per "derive"
}

