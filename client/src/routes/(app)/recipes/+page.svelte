<script lang="ts">
    import { endpoint } from "$lib/api";
    import UserPreview from "../../../components/UserPreview.svelte";
    import type { PageData } from "./$types";
    import defaultThumbnail from "$lib/images/default-thumbnail.jpg"
    import { user } from "$lib/login";
    import type { PublicUserProfileDetails } from "$lib/profile";

    export let data: PageData;
    let signedInUser: PublicUserProfileDetails | null = null;
    user.subscribe(val => {
        signedInUser = val;
    })
</script>

{#if data.error}
    <div>
        <p>Failed to load recipes</p>
    </div>
{:else if data.recipes}
    {#if signedInUser}
        <div class="h-20 flex flex-row items-center">
            <a
                href="/recipes/create"
                class="w-fit px-3 py-2 bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex flex-row gap-2 items-center"
            >
                <p class="text-base font-semibold">+</p>
                <p class="text-base font-semibold">CREATE YOUR OWN RECIPE</p>
            </a>
        </div>
    {:else}
        <div class="h-10" />
    {/if}
    <div class="flex flex-row flex-wrap gap-4 items-center justify-start">
        {#each data.recipes as post}
            <a href={`/recipes/${post.id}`}>
                <div
                    class="w-80 h-96 bg-gray-100 border border-[#00000000] hover:border-[var(--yellow)] box-border shadow-black duration-150 transition-all shadow-hover flex flex-col"
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
        {/each}
    </div>
{/if}
