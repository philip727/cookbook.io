<script lang="ts">
    import TextSinglelineInput from "../../../components/TextSinglelineInput.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<section>
    {#if data.account != null}
        <div class="flex flex-col gap-2">
        <div>
            <p class="text-sm tracking-wider font-semibold">username</p>
            <TextSinglelineInput
                placeholder={data.account.username}
                extraClass="text-xs py-px border h-fit"
            />
        </div>
        <div>
            <p class="text-sm tracking-wider font-semibold">email</p>
            <p class="text-xs py-px h-fit">
                {data.account.email}
            </p>
        </div>
        </div>
    {:else}
        <p class="text-sm tracking-wider">Failed to load account data</p>
    {/if}
</section>
