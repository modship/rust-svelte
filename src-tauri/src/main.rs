#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod cmd;

use tauri::event;

fn main() {
    tauri::AppBuilder::new()
        .invoke_handler(|webview, arg| {
            use cmd::Cmd::*;
            event::emit(&mut webview.as_mut(), "my_event", Some("test_ayload"));
            match serde_json::from_str(arg) {
                Err(e) => {
                    Err(e.to_string())
                }
                Ok(command) => {
                    println!("{:?}", &command);
                    match command {
                        // definitions for your custom commands from Cmd here
                        MyCustomCommand { argument } => {
                            //  your command code
                            println!("{}", argument);
                        }
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
}
