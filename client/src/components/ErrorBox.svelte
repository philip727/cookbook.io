<script lang="ts">
    export let extraClass = "";
    export let error;
    export let description;
</script>

<div
    class={extraClass +
        " " +
        "border-2 border-[#ff0000] w-full px-2 py-2 font-normal"}
>
    <h1 class="font-bold text-[#ff0000]">{error}</h1>
    <p class="text-[#ff0000] text-sm">{description}</p>
</div>
