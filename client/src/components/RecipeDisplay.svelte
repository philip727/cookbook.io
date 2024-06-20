<script lang="ts">
    import { endpoint } from "$lib/api";
    import UserPreview from "./UserPreview.svelte";
    import defaultThumbnail from "$lib/images/default-thumbnail.jpg"
    import type { RecipePreview } from "$lib/routes/recipe";

    export let post: RecipePreview;
</script>

<a href={`/recipes/${post.id}`} class="recipe-display">
    <div
        class="h-96 bg-gray-100 border border-[#00000000] hover:border-[var(--yellow)] box-border shadow-black duration-150 transition-all shadow-hover flex flex-col"
    >
        {#if post.thumbnail != null}
            <img
                class="w-full h-48"
                alt="Food thumbnail"
                src={endpoint(`/thumbnails/${post.thumbnail}`)}
            />
        {:else}
            <img
                class="w-full h-48"
                alt="Food thumbnail"
                src={defaultThumbnail}
            />
        {/if}
        <div class="pb-3 px-3 pt-2 flex flex-col h-full">
            <div class="h-1/2">
                <h1 class="text-xl font-bold">{post.title}</h1>
                <p class="text-sm text-gray-700">
                    {post.description}
                </p>
            </div>
            <div class="mt-4 flex flex-row justify-end items-end h-1/2">
                <UserPreview user={post.poster} />
            </div>
        </div>
    </div>
</a>
