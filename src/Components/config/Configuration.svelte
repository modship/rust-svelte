<script>

    import {tooltip} from "../tooltip";
    import {promisified} from "tauri/api/tauri";
    import {listen} from "tauri/api/event";
    import Subscribe from "./Subscribe.svelte";

    let mqtt_url = 'ws://192.168.1.208:10192';
    let status = 'disconnected';
    let aes_key;

    let login;
    let password;

    /** listen mqtt status event **/
    listen('mqtt_status', (evnt) => {
        status = evnt.payload;
        console.log(evnt);
    });

    async function connect_mqtt(url) {
        return await promisified({
            cmd: 'connectMqtt',
            url: url
        });
    }

    async function disconnect_mqtt() {
        return await promisified({
            cmd: 'disconnectMqtt'
        });
    }

    async function set_aes(aes) {
        const set_aes = await promisified({
            cmd: 'setPassPhrase',
            pass_phrase: aes
        });
        return set_aes;
    }

    function send_connect_info() {
        if (mqtt_url === undefined) {

        } else {
            connect_mqtt(mqtt_url).then((res) => {
                console.log(res);
            });

            /*   promisified({
                   cmd: 'MyCustomCommand',
                   count: 5,
                   payload: {
                       state: 'some string data promise',
                       data: 17
                   }
               }).then((result) => {
                   console.log(result);
               });*/

            /*  invoke({
                  cmd: 'myCustomCommand',
                  payload: {
                      url: 'ws://192.168.1.208:1092'
                  }
              });*/
        }
    }
</script>

<div class="flex h-auto">
    <div class=" w-2/5 bg-white rounded border border-gray-300 mr-4">


        <div class="flex justify-center items-center">
            <div class="flex-1 relative border-b border-gray-300 ">
                <input bind:value={login} type="text"
                       class="h-8 pl-5 w-full rounded-lg focus:shadow focus:outline-none"
                       placeholder="login">
            </div>
        </div>

        <div class="flex justify-center items-center">
            <div class="flex-1 relative border-b border-gray-300">
                <input bind:value={password} type="text"
                       class="h-8 pl-5 w-full rounded-lg focus:shadow focus:outline-none"
                       placeholder="password">
            </div>
        </div>

        <div class="flex justify-center items-center">
            <div class="flex-1 relative border-b border-gray-300 ">
                <input bind:value={mqtt_url} type="text" disabled={status === 'Connected'}
                       class="h-8 pl-5 w-full rounded-lg focus:shadow focus:outline-none"
                       placeholder="ws://ip:port">
                <div class="absolute top-1 right-2">
                    {#if status === 'Connected'}
                        <button class="h-6 w-24 text-white rounded-lg bg-red-500 hover:bg-red-600"
                                on:click={disconnect_mqtt}>Disconnect
                        </button>
                    {:else}
                        <button class="h-6 w-20 text-white rounded-lg bg-green-500 hover:bg-green-600"
                                on:click={send_connect_info}>Connect
                        </button>
                    {/if}
                </div>
                <span use:tooltip={{text: "MQTT Status"}}
                      class=" absolute w-4 h-4 -mt-2 -ml-1 top-0 left-0 rounded-full {status === 'Connected' ? 'bg-green-600' : 'bg-red-600'}"></span>
            </div>
        </div>

        <div class="flex justify-center items-center">
            <div class="flex-1 relative">
                <input bind:value={aes_key} type="text"
                       class="h-8 pl-5 w-full rounded-lg focus:shadow focus:outline-none"
                       placeholder="aes_key">
                <div class="absolute top-1 right-2">
                    <button class="h-6 w-20 text-white rounded-lg bg-blue-500 hover:bg-blue-600"
                            on:click={set_aes(aes_key)}>Apply
                    </button>
                </div>
            </div>
        </div>

    </div>

    <div class="flex w-3/5">
        <Subscribe/>
    </div>

</div>

