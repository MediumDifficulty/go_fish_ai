<script lang="ts">
    import { browser } from "$app/environment";
    import type { GameObserver } from "$lib/types";
    import * as wasm from "$lib/wasm"

    function click() {
        // wasm.init(4, Uint32Array.of(0, 0, 1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0), 0)
        wasm.init(4, undefined, 0)
        let observer = wasm.get_observer() as GameObserver
        console.log(observer)
    }
</script>

{#if browser}
    <div>{wasm.rand()}</div>
    <button on:click={click}>Train</button>
{/if}
