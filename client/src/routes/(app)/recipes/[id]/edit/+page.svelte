<script lang="ts">
import type { PageData } from "./$types";

    export let data: PageData;
    console.log(data.type);
</script>

<section>
    {#if data.authorized}
        hai
    {:else}
        BYE
    {/if}
</section>
