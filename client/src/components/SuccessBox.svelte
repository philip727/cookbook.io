<script lang="ts" context="module">
    export type Success = {
        title: string;
        description: string;
    };
</script>

<script lang="ts">
    export let extraClass = "";
    export let title;
    export let description;
</script>

<div
    class={extraClass +
        " " +
        "border-2 border-[var(--green)] w-full px-2 py-2 font-normal"}
>
    <h1 class="font-bold text-[var(--green)]">{title}</h1>
    <p class="text-[var(--green)] text-sm">{description}</p>
</div>
