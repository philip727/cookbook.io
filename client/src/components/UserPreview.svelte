<script lang="ts">
    import { endpoint } from "$lib/api";
    import type { User } from "$lib/login";

    export let user: User | null = null;
</script>

{#if user}
    <div class="flex justify-end items-center flex-row-reverse gap-4">
        <img
            crossorigin="anonymous"
            src={endpoint(`/pfp/${user.uid}.png`)}
            alt="User profile"
            class="h-12 w-12 object-cover rounded-full"
        />
        <h1>
            {user.username}
        </h1>
    </div>
{/if}
