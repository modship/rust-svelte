<script>
    import {promisified} from "tauri/api/tauri";

    let subscribes = [];
    let topic;

    for (let i = 0; i < 3; i++) {
        let msg = "Topic : " + i;
        subscribes = [...subscribes, msg];
    }

    function subscribe() {
        promisified({
            cmd: 'subscribe',
            topic: topic
        }).then((res) => {
            subscribes = [...subscribes, res];
            topic = '';
        });
    }

    function unsubscribe(current) {
        promisified({
            cmd: 'unsubscribe',
            topic: current
        }).then((res) => {
            subscribes = subscribes.filter(t => t !== res);
        });
    }

</script>

<div class="flex-1 ">
    <span>Subscribes</span>

    <div class="flex justify-center items-center">
        <div class="flex-1 relative border-b border-gray-300">
            <input bind:value={topic} type="text"
                   class="h-8 pl-5 w-full rounded-lg focus:shadow focus:outline-none"
                   placeholder="topic">
        </div>
        <svg on:click={subscribe} xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
             stroke="green">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"/>
        </svg>

    </div>

    <ul class="flex-1  overflow-auto h-24">
        {#each subscribes as current}
            <li class="flex w-full">
                <div class="w-4/5"><span>{current}</span></div>
                <div class="w-1/5 flex justify-center items-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                         stroke="red" on:click={unsubscribe(current)}>
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                    </svg>
                </div>
            </li>
        {/each}
    </ul>
</div>

<style>

</style>
