<script>
    export let placeholder = "...";
    export let type = "text";
    export let extraClass = "text";
</script>

<input
    class={extraClass +
        " " +
        "border-2 border-black w-full pl-2 py-2 focus:border-[var(--green)] focus:outline-none duration-150 transition-colors font-normal"}
    {placeholder}
    {type}
/>
