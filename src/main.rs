mod cli;
mod salotto;
mod discussione;
mod invitati;

use discussione::Discussione;
use clap::Parser;
use async_trait::async_trait;
use crate::invitati::ollama::Ollama;
use crate::invitati::invitato::Invitato;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = cli::Cli::parse(); //invoco il trait parse del tipo Cli che ho definito (vedere il modulo cli per ulteriori info)

    let discussione = Discussione {
        argomento: cli.argomento,
        turni: cli.turni,
        partecipanti: cli.modelli
    };

    let mut cronologia: Vec<String> = Vec::new();
    cronologia.push(format!("Tema: {}", discussione.argomento));

    let path_conversazioni = salotto::salvataggiofile::crea_directory_conversazione()?; //creo directory della conversazione corrente (vedere modulo salvataggiofile.rs)

    let discussione_json = serde_json::to_string_pretty(&discussione)?; //scrivo "discussione" in formato JSON

    let path_discussione_json = path_conversazioni.join("discussione.json"); //creo il path del file dove salvo "discussione_jason"
    salotto::salvataggiofile::scrivi_conversazione(path_discussione_json, &discussione_json)?; //scriviamo la stringa JSON di "discussione" nel file identificato da "path_discussione_json"

    if discussione.partecipanti.is_empty() {
        return Err("Serve invitare almeno un modello".into());
    }

    let mut modelli: Vec<Ollama> = Vec::new();
    for m in &discussione.partecipanti {
        let modello = Ollama::costruttoreollama("http://localhost:11434", m.clone());
        modelli.push(modello);
    }

    for turno in 0..discussione.turni {
        for modello in &modelli {

            let prompt = if cronologia.len() == 1 {
                format!(
                    "Sei {}.\nApri una discussione sul tema seguente:\n{}\n\nRispondi:",
                    modello.vestito,
                    discussione.argomento
                )
            } else {
                format!(
                    "Sei {}.\nQuesta è la discussione finora:\n{}\n\nContinua la discussione con un nuovo intervento:",
                    modello.vestito,
                    cronologia.join("\n")
                )
            };

            let risposta = modello.chiacchiera(&prompt).await?;

            let risposta_formattata = format!("{}: {}", modello.vestito, risposta);

            println!("{}", risposta_formattata);

            cronologia.push(risposta_formattata);
        }
    }


    let verbale = cronologia.join("\n\n");

    let path_verbale = path_conversazioni.join("verbale.txt");
    salotto::salvataggiofile::scrivi_conversazione(path_verbale, &verbale)?;

    Ok(())
}
