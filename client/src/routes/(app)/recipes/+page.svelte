<script lang="ts">
    import UserPreview from "../../../components/UserPreview.svelte";
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
                    <div>
                        <h1 class="text-xl font-bold">{post.recipe.title}</h1>
                        <p class="text-sm text-gray-700">{post.recipe.description}</p>
                    </div>
                    <div>
                        <UserPreview user={post.poster} />
                    </div>
                </div>
            </a>
        {/each}
    </div>
{/if}
