<script>
    import {term} from './term.js';
    let my_term;
    let a;

    async function open_about() {
            term.set(my_term);
            const { WebviewWindow } = await import('@tauri-apps/api/window');
            a = new WebviewWindow("label", {
                url: "/about"
            });
            await a.emit("term", my_term);
    }
</script>
<div class="searchbox">
<h1>The search term is {my_term}</h1>
<h2>the store is {$term}</h2>
<input bind:value={my_term} on:input={ async () => open_about() }>

</div>
