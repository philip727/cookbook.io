<script lang="ts">
    import { endpoint } from "$lib/api";
    import { isResponseError } from "$lib/routes/error";
    import RecipeDisplay from "../../../../components/RecipeDisplay.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<article>
    {#if data && data.user.type !== "RESPONSE_ERROR"}
        <div class="p-4 bg-gray-100 w-full mt-32 flex flex-row">
            {#if data.user.picture != null}
                <img
                    crossorigin="anonymous"
                    src={endpoint(`/pfp/${data.user.picture}`)}
                    alt="User profile"
                    class="h-40 w-40 object-cover rounded-full"
                />
            {:else}
                <img
                    crossorigin="anonymous"
                    src={`https://api.dicebear.com/8.x/shapes/svg?seed=${data.user.username}`}
                    alt="User profile"
                    class="h-40 w-40 object-cover rounded-full"
                />
            {/if}
            <div class="ml-4">
                <p class="text-3xl font-bold">{data.user.username}</p>
                <div class="mt-1">
                    {#if data.user.pronouns}
                        <p class="text-base text-gray-700">
                            {data.user.pronouns}
                        </p>
                    {/if}
                    {#if data.user.location}
                        <p class="text-base text-gray-700">
                            {data.user.location}
                        </p>
                    {/if}
                </div>
            </div>
        </div>
        {#if !isResponseError(data.recipes) && data.recipes.type !== "RESPONSE_ERROR"}
            <div
                class="flex flex-row flex-wrap gap-4 items-center justify-start mt-8"
            >
                {#each data.recipes.collection as post}
                    <RecipeDisplay {post} />
                {/each}
            </div>
        {/if}
    {/if}
</article>
