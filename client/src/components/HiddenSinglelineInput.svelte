<script lang="ts">
    export let placeholder = "...";
    export let extraClass = "";
    export let value: any = undefined;
    export let name: string | undefined = undefined;
</script>

<input
    class={extraClass +
        " " +
        "border-2 border-black w-full pl-2 py-2 focus:border-[var(--green)] focus:outline-none duration-150 transition-colors font-normal"}
    {placeholder}
    {name}
    type="password"
    bind:value={value}
/>
