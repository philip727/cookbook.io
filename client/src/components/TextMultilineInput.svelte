<script lang="ts">
    export let name: string | undefined = undefined;
    export let placeholder = "...";
    export let extraClass = "";
    export let value: any = undefined;
</script>

<textarea
    class={extraClass +
        " " +
        "border-2 border-black w-full px-2 py-2 focus:border-[var(--green)] focus:outline-none duration-150 transition-colors font-normal resize-none h-32 pt-1"}
    {placeholder}
    {name}
    bind:value
/>
