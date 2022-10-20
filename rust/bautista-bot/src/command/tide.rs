use super::*;
use crate::telegram::Message;
use reqwest::blocking::Client;
use serde::Deserialize;

pub struct TideCommand {
    client: Client,
}

impl TideCommand {
    pub fn new() -> TideCommand {
        TideCommand {
            client: Client::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Reply {
    pub mareas: Mareas,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Mareas {
    pub datos: Datos,
    pub fecha: String,
    pub puerto: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Datos {
    marea: Vec<Dato>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Dato {
    altura: String,
    hora: String,
    tipo: String,
}

impl Command for TideCommand {
    fn run(&self, msg: &Message) -> Result<Option<String>> {
        let text = &msg.text;

        if !text.starts_with("TIDE") {
            return Ok(None);
        }

        let url = "https://ideihm.covam.es/api-ihm/getmarea?request=gettide&id=4&format=json";

        let reply: Reply = self.client.get(url).send()?.json()?;

        let reply = reply.mareas;

        let mut text = format!(
            "🌊  Mareas del {} en {}  🌊\n\n",
            &reply.fecha, &reply.puerto
        );

        for dato in reply.datos.marea.iter() {
            let icon = if dato.tipo == "pleamar" { "↗" } else { "↘" };

            text.push_str(&format!(
                "          {}  {}  {}  ({} m)\n",
                &icon, &dato.hora, &dato.tipo, &dato.altura
            ));
        }

        Ok(Some(text))
    }
}
