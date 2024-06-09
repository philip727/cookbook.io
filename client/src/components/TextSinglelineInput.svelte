<script lang="ts">
    export let name: string | undefined = undefined;
    export let placeholder = "...";
    export let extraClass = "text";
    export let value: any = undefined;
</script>

<input
    class={"border-2 border-black w-full pl-2 py-2 focus:border-[var(--green)] focus:outline-none duration-150 transition-colors font-normal " +
        extraClass}
    {placeholder}
    {name}
    type="text"
    bind:value
/>
