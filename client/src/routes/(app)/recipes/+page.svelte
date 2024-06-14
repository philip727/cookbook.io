<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

{#if data.error}
    <div>
        <p>Failed to load recipes</p>
    </div>
{:else if data.recipes}
    <div class="flex flex-row flex-wrap gap-2 items-center justify-center">
        {#each data.recipes as post}
            <a href={`/recipes/${post.recipe.id}`}>
                <div class="w-80 h-72">
                    <h1 class="text-xl font-bold">{post.recipe.title}</h1>
                    <p class="text-sm text-gray-700">{post.recipe.description}</p>
                </div>
            </a>
        {/each}
    </div>
{/if}
