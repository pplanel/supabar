<script>
    import {term } from "./term.js"
    import {onMount} from 'svelte';
    onMount(async () => {
            const {appWindow} =await import('@tauri-apps/api/window');
            await appWindow.listen("term", ({event, payload}) => {
                    my_term = payload
                })
    })
    let my_term = "vazio";
    term.subscribe(v => {
        my_term = v;
    })
</script>

<h1>The term is {my_term} aaa</h1>
