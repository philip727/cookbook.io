<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { v4 as uuidv4 } from "uuid";

    let identifier = uuidv4();
    export let values: Array<any>;
    export let extraClass = "";
    let open = false;
    export let selectedValue = values[0];

    function closeComboboxWhenNotClicked(event: MouseEvent) {
        if (!event.target) {
            return;
        }

        const target = event.target as HTMLElement;
        if (
            !target.closest(`#cb-${identifier}`) &&
            !target.closest(`#cb-btn-${identifier}`)
        ) {
            open = false;
        }
    }

    onMount(() => {
        document.addEventListener("click", closeComboboxWhenNotClicked);
    });

    onDestroy(() => {
        document.removeEventListener("click", closeComboboxWhenNotClicked);
    });
</script>

<div class="relative w-full">
    <button
        id={`cb-btn-${identifier}`}
        class={"w-full px-3 py-px bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex flex-row items-center justify-between combobox-btn z-40" +
            " " +
            extraClass}
        on:click={() => (open = !open)}
        type="button"
    >
        <p class="text-xs font-semibold">{selectedValue}</p>
        <p class="ml-8 text-xs font-semibold">{open ? "-" : ">"}</p>
    </button>
    {#if open}
        <div
            id={`cb-${identifier}`}
            class="absolute top-full left-0 w-full z-50"
        >
            {#each values as value}
                <button
                    class={"w-full px-3 py-px bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex flex-row items-center z-50" +
                        " " +
                        extraClass}
                    on:click={() => {
                        open = false;
                        selectedValue = value;
                    }}
                    type="button"
                >
                    <p class="text-xs font-semibold">{value}</p>
                </button>
            {/each}
        </div>
    {/if}
</div>
