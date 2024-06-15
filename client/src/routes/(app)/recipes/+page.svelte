<script lang="ts">
    import { endpoint } from "$lib/api";
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
            <a href={`/recipes/${post.id}`}>
                <div class="w-80 h-72">
                    {#if post.thumbnail != null}{/if}
                    <img
                        class="w-full h-48"
                        src={endpoint(`/thumbnails/${post.thumbnail}`)}
                    />
                    <div>
                        <h1 class="text-xl font-bold">{post.title}</h1>
                        <p class="text-sm text-gray-700">
                            {post.description}
                        </p>
                    </div>
                    <div>
                        <UserPreview user={post.poster} />
                    </div>
                </div>
            </a>
        {/each}
    </div>
{/if}
