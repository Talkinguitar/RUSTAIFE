//IDEA: definisco cosa devono saper fare gli invitati alla discussione

use std::fmt; //per formattazione testuale, serve per il trait Display
use async_trait::async_trait; //

#[derive(Debug)] //e che non ce lo mettiamo Debug?
pub enum ComeScusa { //per quando non ci capiamo per bene (gestione errori)
    NonTiSento(reqwest::Error), //uno degli invitati non ci sente (la richiesta HTTP ha avuto qualche problema)
    InCheSenso(String), //uno degli invitati ha detto qualcosa di strano (risposte flaggate come vuote o incomplete, oppure modello inesistente)
}

impl fmt::Display for ComeScusa { //implemento il trait Display per ComeScusa
    fn fmt(&self, comescusa: &mut fmt::Formatter) -> fmt::Result { //metodo di Display che legge (per questo &self e non self) il valore corrente (&self) e lo scrive formattato in f
        match self { //a seconda dei casi mostriamo un errore diverso
            ComeScusa::NonTiSento(CheSuccede) => write!(comescusa, "Non ti sento! (errore HTTP): {}", CheSuccede),
            ComeScusa::InCheSenso(CheVuolDire) => write!(comescusa, "In che senso? (errore API):{}", CheVuolDire),
        }
    }
}

impl std::error::Error for ComeScusa { //implemento il trait generico per gli errori di Rust
} //blocco vuoto perché "Rust sa", grande Rust!

impl From<reqwest::Error> for ComeScusa { //per wrappare gli errori generici in un ComeScusa
    fn from(chesuccede: reqwest::Error) -> Self { //trasformo gli errori dal tipo "reqwest::Error" che mi passa la crate reqwest in un tipo "ComeScusa"
        ComeScusa::NonTiSento(chesuccede)
    }
}

#[async_trait]
pub trait Invitato { //cose che ogni invitato deve saper fare
    async fn chiacchiera(&self, argomento: &str) -> Result<String, ComeScusa>; //tutti gli invitati devono saper chiacchierare dell'argomento in discussione (ritornano la chiacchiera come una stringa e se si impicciano ritornano un ComeScusa)}
}
