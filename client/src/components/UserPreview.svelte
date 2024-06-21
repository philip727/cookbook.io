<script lang="ts">
    import { endpoint } from "$lib/api";
    import type { MinimalUserDetails, UserDetails } from "$lib/routes/user";

    export let user: UserDetails | MinimalUserDetails | null = null;
</script>

{#if user}
    <div class="flex justify-end items-center flex-row-reverse gap-4">
        {#if user.picture != null}
            <img
                crossorigin="anonymous"
                src={endpoint(`/pfp/${user.picture}`)}
                alt="User profile"
                class="h-12 w-12 object-cover rounded-full"
                draggable="false"
            />
        {:else}
            <img
                crossorigin="anonymous"
                src={`https://api.dicebear.com/8.x/shapes/svg?seed=${user.username}`}
                alt="User profile"
                class="h-12 w-12 object-cover rounded-full"
                draggable="false"
            />
        {/if}
        <h1 class="text-lg font-bold">
            {user.username}
        </h1>
    </div>
{/if}
