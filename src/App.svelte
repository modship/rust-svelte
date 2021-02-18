<script>
    import {invoke} from 'tauri/api/tauri'
    import {emit, listen} from 'tauri/api/event'

    import Tailwind from './Tailwind.svelte';
    import Subscribe from "./Subscribe.svelte";

    let count = 0;
    let messages = [];
    let mqtt_url;


    listen('my_event', evnt => {
        count += 1;
        messages = [...messages, JSON.stringify(evnt.payload)];
    });

    function send_connect_info() {
        if (mqtt_url === undefined) {

        } else {
            invoke({
                cmd: 'MyCustomCommand',
                count: 5,
                payload: {
                    state: 'some string data',
                    data: 17
                }
            })
        }
    }

    export let name;

</script>

<main>


    mqtt_url : {mqtt_url} <input bind:value={mqtt_url}>
    <div class="flex">
        <span class="text-sm border border-2 rounded-l px-4 py-2 bg-gray-300 whitespace-no-wrap">MQTT</span>
        <input name="field_name" class="border border-2 rounded-r px-4 py-2 w-full" type="text" placeholder="tcp://127.0.0.1:1883" />
    </div>

    <Subscribe/>


    <h1>Hello {name}!</h1>
    <p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
    <button on:click={send_connect_info}>
        Connect MQTT
    </button>
    <ul>
        {#each messages as msg}
            <li>
                {msg}
            </li>
        {/each}
    </ul>


</main>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4em;
        font-weight: 100;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>
