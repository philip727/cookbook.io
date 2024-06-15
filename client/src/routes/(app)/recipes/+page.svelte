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
    <div class="flex flex-row flex-wrap gap-2 items-center justify-center py-4">
        {#each data.recipes as post}
            <a href={`/recipes/${post.id}`}>
                <div class="w-80 h-fit bg-gray-100 border border-[#00000000] hover:border-[var(--yellow)] box-border shadow-black duration-150 transition-all shadow-hover">
                    {#if post.thumbnail != null}{/if}
                    <img
                        class="w-full h-48"
                        src={endpoint(`/thumbnails/${post.thumbnail}`)}
                    />
                    <div class="pb-3 px-3 pt-2">
                        <div>
                            <h1 class="text-xl font-bold">{post.title}</h1>
                            <p class="text-sm text-gray-700">
                                {post.description}
                            </p>
                        </div>
                        <div class="mt-4 flex flex-row justify-end">
                            <UserPreview user={post.poster} />
                        </div>
                    </div>
                </div>
            </a>
        {/each}
    </div>
{/if}
