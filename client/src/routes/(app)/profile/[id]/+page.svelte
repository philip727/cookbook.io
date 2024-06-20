<script lang="ts">
    import { endpoint } from "$lib/api";
    import { isUserDetails } from "$lib/routes/user";
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<article>
    {#if data && isUserDetails(data)}
        <div class="bg-white p-4 shadow-one w-full mt-32 flex flex-row">
            {#if data.picture != null}
                <img
                    crossorigin="anonymous"
                    src={endpoint(`/pfp/${data.picture}`)}
                    alt="User profile"
                    class="h-40 w-40 object-cover rounded-full"
                />
            {:else}
                <img
                    crossorigin="anonymous"
                    src={`https://api.dicebear.com/8.x/shapes/svg?seed=${data.username}`}
                    alt="User profile"
                    class="h-40 w-40 object-cover rounded-full"
                />
            {/if}
            <div class="ml-4">
                <p class="text-3xl font-bold">{data.username}</p>
                <div class="mt-1">
                    {#if data.pronouns}
                        <p class="text-base text-gray-700">{data.pronouns}</p>
                    {/if}
                    {#if data.location}
                        <p class="text-base text-gray-700">{data.location}</p>
                    {/if}
                </div>
            </div>
        </div>
    {/if}
</article>
