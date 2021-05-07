<script>
    import {emit} from 'tauri/api/event'


    import Tailwind from './Tailwind.svelte';
    import Text from "./Components/Text.svelte"
    import Configuration from "./Components/config/Configuration.svelte";

    function emitEvent() {
        emit("js-event", "this is the payload string");
    }

</script>

<main class="h-full bg-gradient-to-b from-blue-100 to-green-100">
        <div class="h-1/5">
                <Configuration/>
        </div>


        <div class="h-4/5 mt-2">
                <Text/>
        </div>

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
