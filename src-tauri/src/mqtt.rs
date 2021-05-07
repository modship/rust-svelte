extern crate paho_mqtt;

use std::time::Duration;
use std::{process};
use log::{info, trace, warn};
use lazy_static::lazy_static;
use paho_mqtt::AsyncClient;

use serde::Serialize;

use crate::crypto::*;
// use crate::mqtt::test::MyMsg;
// use actix::Actor;

use anyhow::{Result};

use self::paho_mqtt::{Message, QOS_2, DisconnectOptions};

// const DFLT_BROKER: &str = "tcp://127.0.0.1:1883";
const DFLT_CLIENT: &str = "mqtt_tools";
// const DFLT_TOPICS: &[&str] = &["bpv/in/#", "rust/test"];

#[derive(Serialize, Debug)]
pub enum MqttStatus {
    Disconnect = 0,
    Connecting = 1,
    Connected = 2
}

lazy_static! {
    static ref BROKER_URL: String = std::env::var("BROKER_URL").unwrap() ;
    static ref CLI: AsyncClient = {
        let create_opts = paho_mqtt::CreateOptionsBuilder::new()
            .server_uri(std::env::var("BROKER_URL").unwrap())
            .client_id(DFLT_CLIENT.to_string())
            .finalize();

        let mut cli = paho_mqtt::AsyncClient::new(create_opts).unwrap_or_else(|err| {
            error!("MQTT : Error creating the client: {:?}", err);
            process::exit(1);
        });

        cli.set_connection_lost_callback(|cli| {
            info!("MQTT : Connexion LOST ");
            info!("MQTT : Tentative de reconnexion ... ");
            // cli.reconnect();
        });

        cli.set_message_callback(|_cli, msg| {
            if let Some(msg) = msg {
                on_message(msg);
            }
        });

        cli.set_disconnected_callback(|_cl,_a ,_b| {
             info!("MQTT : disconnected");
        });

        cli.set_connected_callback(|cli| {
            info!("MQTT : Connexion réussie à {}", BROKER_URL.as_str());
            // tauri::event::emit(&mut webview.as_mut(), "mqtt_status", Some(MqttStatus::Connected));
            cli.subscribe("bpv/in/#", QOS_2);
        });

        cli
    };
}

#[allow(dead_code)]
fn subscribe() {
    let su = CLI.subscribe("bpv/in/#", QOS_2);
    let res = su.wait();

    match res {
        Err(_) => {
            error!("MQTT : Subscribe error");
        }
        Ok(_r) => {
            info!("MQTT : Subscribe OK");
        }
    }
}

fn on_message(msg: Message) {
    /* let test = |msg: Message| {
        info!("callback is called");
    };*/

    let topic = msg.topic();
    let payload_str = msg.payload_str();

    if payload_str.contains("salt") {
        let encrypted: Encrypted = serde_json::from_str(payload_str.as_ref()).unwrap();

        let result = Crypto::decrypt_aes(
            "5e0f99aaa80e0cff07b8bfae8a3f7c5a13eb6be81b5de623054e14afe61d5de1",
            &encrypted,
        );

        match result {
            Ok(str) => {
                info!(
                    "MQTT : Réception d'un message sur -> {} : \r\n {}",
                    topic, str
                );
            }
            Err(e) => error!("{}", &e.to_string()),
        }
    } else {
        info!(
            "MQTT : Réception d'un message sur -> {} : \r\n {}",
            topic, payload_str
        );
    }
}

pub fn init() {
    let try_host_url = std::env::var("BROKER_URL");

    if try_host_url.is_err() {
        warn!("MQTT : BROKER_URL introuvable");
    } else {
        info!("MQTT : Initialisation de la connexion");

        lazy_static::initialize(&CLI);

        // let re = Regex::new(r"^medical-checkup/maj/\d{4}$").unwrap();
        // assert!(re.is_match("medical-checkup/maj/10"));

        // Define the set of options for the connection.
        let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .retry_interval(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(5))
            .automatic_reconnect(Duration::from_secs(3), Duration::from_secs(5))
            .finalize();

        // Connect and wait for it to complete or fail.
        CLI.connect(conn_opts);
    }
}

/*fn on_connect_success(cli: &paho_mqtt::AsyncClient, _msgid: u16) {
    info!("MQTT : Connexion réussie à {}", BROKER_URL.as_str());
    cli.subscribe("bpv/in/#", QOS_2);
}

fn on_connect_failure(cli: &paho_mqtt::AsyncClient, _msgid: u16, rc: i32) {
    warn!("Connection attempt failed with error code {}.\n", rc);
    thread::sleep(Duration::from_millis(2500));
    cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
}*/

pub fn send_message(topic: &str, msg: &str, pass_phrase: &str) {
    if CLI.is_connected() {
        let result = Crypto::encrypt_aes(pass_phrase, msg.as_bytes());

        match result {
            Ok(encrypted) => {
                let s = serde_json::to_string(&encrypted).unwrap();

                let mqtt_msg: Message = Message::new(topic, s.as_bytes(), QOS_2);
                let result = CLI.try_publish(mqtt_msg);

                match result {
                    Err(_e) => error!(
                        "MQTT : Erreur lors de la publication du message : {} sur {}",
                        msg, topic
                    ),
                    Ok(_t) => info!("MQTT : Envoi d'un message sur -> {} : \r\n {}", topic, msg),
                }
            }
            Err(_e) => error!("MQTT: Erreur lors du chiffrement des données"),
        }
    } else {
        warn!("MQTT : Client non connecté au moment de la tentative de publication du message");
    }
}

pub fn send_retained_message(
    topic: &str,
    msg: &str,
    pass_phrase_opts: Option<&str>,
) -> Result<()> {
    is_connected()?;

    let mqtt_msg: Message = new_mqtt_msg(topic, msg, pass_phrase_opts)?;

    let result = CLI.try_publish(mqtt_msg);
    match result {
        Err(_e) => {
            error!(
                "MQTT : Erreur lors de la publication du message : {} sur {}",
                msg, topic
            );

            bail!("Erreur lors de la publication du message".to_string())
        }
        Ok(_t) => {
            info!("MQTT : Envoi d'un message sur -> {} : \r\n {}", topic, msg);
            Ok(())
        }
    }
}

fn is_connected() -> Result<()> {
    if CLI.is_connected() {
        Ok(())
    } else {
        warn!("MQTT : Client non connecté au moment de la tentative de publication du message");
        bail!( "MQTT - Client non connecté au moment de la tentative de publication du message"
                .to_string())
    }
}

fn new_mqtt_msg(
    topic: &str,
    msg: &str,
    pass_phrase_opts: Option<&str>,
) -> Result<Message> {
    Ok(if let Some(pass_phrase) = pass_phrase_opts {
        // Si il y a une pass phrase on chiffre
        let result = Crypto::encrypt_aes(pass_phrase, msg.as_bytes());

        match result {
            Ok(encrypted) => {
                let s = serde_json::to_string(&encrypted)?;
                Message::new_retained(topic, s.as_bytes(), QOS_2)
            }
            Err(_e) => {
                error!("MQTT: Erreur lors du chiffrement des données");
                bail!("Erreur lors du chiffrement des données".to_string())
            }
        }
    } else {
        // Sinon on construit le message non chiffré
        Message::new_retained(topic, msg.as_bytes(), QOS_2)
    })
}

pub fn disconnect() -> Result<()> {
    if CLI.is_connected() {
        let disconnect = CLI.disconnect(DisconnectOptions::default());
        Ok(())
    } else {
        Ok(())
    }
}
