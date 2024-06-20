<script lang="ts">
    import type { PageData } from "./$types";
    import { user } from "$lib/login";
    import type { UserDetails } from "$lib/routes/user";
    import RecipeDisplay from "../../../components/RecipeDisplay.svelte";

    export let data: PageData;
    let signedInUser: UserDetails | null = null;
    user.subscribe(val => {
        signedInUser = val;
    })
</script>

{#if data.type == "RESPONSE_ERROR"}
    <div>
        <p>Failed to load recipes</p>
    </div>
{:else}
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
    <div class="recipes-container justify-start flex-row">
        {#each data.recipes as post}
            <RecipeDisplay post={post} />
        {/each}
    </div>
{/if}
