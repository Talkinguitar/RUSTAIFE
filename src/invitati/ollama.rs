//IDEA: stabilire come si comporta il programma con Ollama

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use crate::invitati::invitato::{Invitato, ComeScusa};

pub struct Ollama {
    pub indirizzo: String, //qui ci va l'URL del provider
    pub vestito: String, //si Riccardo del futuro, potevo chiamarlo "modello", comunque è il campo per il modello di Ollama che vuoi usare
    client: reqwest::Client, //penso sia meglio salvare un client permanente nella struct piuttosto che ricrearlo ogni volta (controllare)
}

impl Ollama {
    pub fn costruttoreollama(indirizzo: impl Into<String> , vestito: impl Into<String>) -> Ollama { //ci metto into string per potergli passare pure le &str
        Ollama{
            indirizzo: indirizzo.into(),
            vestito: vestito.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Invito { //qui i nomi dei campi sono obbligatoriamente dettati dall'endpoint dell'API di Ollama. Altrimenti quando serde serializza la struct non riesce a far coincidere i campi nel JSON
    model: String, //nome del modello da usare
    prompt: String, //tema della discussione
    stream: bool, //questo serve per stabilire come far rispondere il modello, se tutto insieme in un unico JSON oppure in vari JSON separati. In un unico JSON è più comodo. Lo impongo dopo.
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Risposta { //vale lo stesso discorso di prima per i nomi
    response: String, //campo per la risposta
    done: bool, //campo che segna lo status di fine di generazione della risposta
    error: Option<String>, //campo per eventuali errori. Siccome non viene sempre generato ci ho messo "option" in modo che restituisca None se è vuoto.
}

#[async_trait]
impl Invitato for Ollama { //vediamo cosa deve saper fare Ollama
    async fn chiacchiera(&self, prompt: &str) -> Result<String, ComeScusa> {

        let url = format!("{}/api/generate", self.indirizzo.trim_end_matches('/')); //api/generate è l'endpoint esposto dal server di Ollama

        let invito = Invito{
            model: self.vestito.clone(), //modello da usare. Uso clone() perchè nella struct Invito serve un tipo String (ovvero richiede ownership) e non un semplice riferimento ma negli argomenti ho preso &self (ovvero un riferimento) che non ha ownership della propria memoria e non la può trasferire
            prompt: prompt.to_string(), //prompt dato in pasto al modello
            stream: false, //lo imposto sempre falso in modo che la risposta sia sempre un unico JSON invece che tanti JSON differenti
        };

        let risposta = self.client //questa è la parte complicata. Costruisco la richiesta HTTP
            .post(&url) //.post è un metodo di reqwest::client e serve a creare un oggetto intermedio RequestBuilder che serve a inviare dati a un certo url
            .json(&invito) //aggiungo il corpo della richiesta HTTP serializzandolo in JSON
            .send() //mando la richiesta
            .await?; //serve per richiesta asincrona

        let rispostaparsed: Risposta = risposta.json().await?;

        Ok(rispostaparsed.response)
    }
}