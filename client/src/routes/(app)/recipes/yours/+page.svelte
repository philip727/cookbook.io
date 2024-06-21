<script lang="ts">
    import { endpoint } from "$lib/api";
    import type { PageData } from "./$types";
    import deleteBin from "$lib/images/recycle-bin.svg";

    export let data: PageData;

    console.log(data);
</script>

<section>
    <div class="mt-32 pb-16">
        <h1 class="text-4xl font-bold mt-2">Your recipes</h1>
        {#if data.collection}
            {#each data.collection as recipe}
                <div
                    class="bg-gray-100 h-20 w-full mt-6 flex items-center justify-start px-2"
                >
                    <img
                        alt="Recipe thumbnail"
                        class="h-16 w-16"
                        src={endpoint(`/thumbnails/${recipe.thumbnail}`)}
                    />
                    <div class="h-full py-1 ml-4 w-full">
                        <p class="text-xl font-semibold">{recipe.title}</p>
                        <p class="text-base font-light">{recipe.description}</p>
                    </div>
                    <div class="w-full flex justify-end">
                        <button
                            class="cursor-pointer h-16 w-16 bg-[var(--yellow)] ml-4 hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center"
                            type="button"
                        >
                            <p class="tracking-widest font-medium">EDIT</p>
                        </button>
                        <button
                            class="cursor-pointer h-16 w-16 bg-[var(--yellow)] ml-4 hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center"
                            type="button"
                        >
                            <img
                                class="h-10 w-10"
                                src={deleteBin}
                                alt="Delete button"
                            />
                        </button>
                    </div>
                </div>
            {/each}
        {/if}
    </div>
</section>
