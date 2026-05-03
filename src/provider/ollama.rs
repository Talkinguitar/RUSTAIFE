//IDEA: stabilire come si comporta il programma con Ollama

use crate::provider::provider::{ErrorHandling, Provider};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct Ollama {
    pub provider_url: String,
    pub llm: String,
    client: reqwest::Client,
}

impl Ollama {
    pub fn new(provider_url: impl Into<String>, llm: impl Into<String>) -> Ollama {
        Ollama {
            provider_url: provider_url.into(),
            llm: llm.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InquirySettings {
    //i nomi dei campi sono obbligatoriamente dettati dall'endpoint dell'API di Ollama. Altrimenti quando serde serializza la struct non riesce a far coincidere i campi nel JSON
    model: String,
    prompt: String,
    stream: bool, //questo campo serve per stabilire come far rispondere il modello, se tutto insieme in un unico JSON oppure in vari JSON separati.
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSettings {
    //vale lo stesso discorso di prima per i nomi
    response: String,
    done: bool,            //status della risposta (finito, in corso)
    error: Option<String>, //campo per eventuali errori. Restituisce None se è vuoto.
}

#[async_trait]
impl Provider for Ollama {
    async fn identify_and_answer(&self, prompt: &str) -> Result<String, ErrorHandling> {
        let url = format!("{}/api/generate", self.provider_url.trim_end_matches('/')); //api/generate è l'endpoint esposto dal server di Ollama

        let inquiry = InquirySettings {
            model: self.llm.clone(), //modello da usare. Uso clone() perché nella struct InquirySettings serve un tipo String (ovvero richiede ownership) e non un semplice riferimento ma negli argomenti ho preso &self (ovvero un riferimento) che non ha ownership della propria memoria e non la può trasferire
            prompt: prompt.to_string(),
            stream: false, //lo imposto sempre falso in modo che la risposta sia sempre un unico JSON invece che tanti JSON differenti
        };

        let response = self
            .client //costruisco la richiesta HTTP
            .post(&url) //.post è un metodo di reqwest::client e serve a creare un oggetto intermedio RequestBuilder che serve a inviare dati a un certo url
            .json(&inquiry) //aggiungo il corpo della richiesta HTTP serializzandolo in JSON
            .send() //mando la richiesta
            .await?; //serve per richiesta asincrona

        let parsed_response: ResponseSettings = response.json().await?;

        Ok(parsed_response.response)
    }
}
