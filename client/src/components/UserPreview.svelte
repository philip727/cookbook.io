<script lang="ts">
    import { endpoint } from "$lib/api";
    import type { PublicUserProfileDetails } from "$lib/profile";

    export let user: PublicUserProfileDetails | null = null;
</script>

{#if user}
    <div class="flex justify-end items-center flex-row-reverse gap-4">
        {#if user.picture != null}
            <img
                crossorigin="anonymous"
                src={endpoint(`/pfp/${user.picture}`)}
                alt="User profile"
                class="h-12 w-12 object-cover rounded-full"
            />
        {:else}
            <img
                crossorigin="anonymous"
                src={`https://api.dicebear.com/8.x/shapes/svg?seed=${user.username}`}
                alt="User profile"
                class="h-12 w-12 object-cover rounded-full"
            />
        {/if}
        <h1 class="text-lg font-bold">
            {user.username}
        </h1>
    </div>
{/if}
