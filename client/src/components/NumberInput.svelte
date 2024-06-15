<script lang="ts">
    export let name: string | undefined = undefined;
    export let placeholder = "...";
    export let extraClass = "";
    export let value: number | undefined = undefined;
</script>

<input
    type="number"
    class={"border-2 border-black w-full pl-2 py-2 focus:border-[var(--green)] focus:outline-none duration-150 transition-colors font-normal " +
        extraClass}
    {placeholder}
    {name}
    bind:value
/>
