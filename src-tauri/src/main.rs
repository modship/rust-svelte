#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod cmd;
mod mqtt;
#[macro_use]
extern crate log;

extern crate crypto;

#[macro_use]
extern crate anyhow;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use tauri::{event, App};
use crate::cmd::Cmd;
use crate::mqtt::MqttStatus;

pub struct Config {
    mqtt_url: String,
    aes_key: String
}

fn main() {

    // Init logger
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let app = tauri::AppBuilder::new()
        .invoke_handler(|webview, arg| {
            use cmd::Cmd::*;

            match serde_json::from_str(arg) {
                Err(e) => {
                    error!("{}", e.to_string());
                }
                Ok(command) => {
                    match command {
                        // definitions for your custom commands from Cmd here
                        MyCustomCommand { argument } => {
                            //  your command code
                            info!("args : {}", argument);
                        },
                        ConnectMqtt {
                            url,
                            callback,
                            error,
                        } => tauri::execute_promise(
                            webview,
                            move || {
                                info!("Event connexion MQTT");
                                std::env::set_var("BROKER_URL", url);
                                mqtt::init();
                                Ok(())
                            },
                            callback,
                            error,
                        ),
                        DisconnectMqtt {
                            callback,
                            error,
                        } => tauri::execute_promise(
                            webview,
                            move || {
                                info!("Event disconnect MQTT");
                                std::env::remove_var("BROKER_URL");
                                let resul = mqtt::disconnect();
                                Ok(())
                            },
                            callback,
                            error,
                        ),
                        SetPassPhrase {
                            pass_phrase, callback, error
                        } => tauri::execute_promise(
                            webview,
                            move || {
                                info!("Event set AES");
                                std::env::set_var("AES_KEY", pass_phrase);
                                show_notif();
                                Ok(())
                            },
                            callback,
                            error,
                        ),
                        Subscribe {topic, callback, error} => tauri::execute_promise(
                            webview,
                            move || {
                                info!("Event subsribe topic");
                                Ok(topic)
                            },
                            callback,
                            error,),
                        Unsubscribe {topic, callback, error} => tauri::execute_promise(
                            webview,
                            move || {
                                info!("Event unsubscribe topic : {}", topic);
                                Ok(topic)
                            },
                            callback,
                            error,),
                    }
                }
            };


            tauri::event::emit(&mut webview.as_mut(), "mqtt_status", Some(MqttStatus::Connected));
            Ok(())
        })
        .build();


    tauri::event::listen("js-event", move |event| {
        println!("got js-event with message '{:?}'", event);


     /*   let reply = Reply {
            data: "something else".to_string(),
        };*/

       /* window_
            .emit("rust-event", Some(reply))
            .expect("failed to emit");*/
    });

    app.run();
}

fn show_notif() {
    tauri::api::notification::Notification::new()
        .title("New message")
        .body("You've got a new message.")
        .show();
}

fn send_event(app: &App) {

}

fn print_env(env: &str) {
    match std::env::var(env) {
        Ok(aes) => info!("AES = {}", aes),
        Err(e) => error!("{}", e.to_string())
    };
}


