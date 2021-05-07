use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    // your custom commands
    // multiple arguments are allowed
    // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
    MyCustomCommand { argument: String },

    ConnectMqtt {
        url: String,
        callback: String,
        error: String,
    },
    SetPassPhrase {
        pass_phrase: String,
        callback: String,
        error: String,
    },
    DisconnectMqtt {
        callback: String,
        error: String,
    },
    Subscribe {
        topic: String,
        callback: String,
        error: String,
    },
    Unsubscribe {
        topic: String,
        callback: String,
        error: String,
    }
}
