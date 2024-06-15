<script lang="ts">
    import Combobox from "../../../../components/Combobox.svelte";
    import NumberInput from "../../../../components/NumberInput.svelte";
    import TextMultilineInput from "../../../../components/TextMultilineInput.svelte";
    import TextSinglelineInput from "../../../../components/TextSinglelineInput.svelte";
    import { Measurement, type Ingredient } from "./helpers";
    import deleteBin from "$lib/images/recycle-bin.svg";

    let ingredients: Array<Ingredient> = [];

    const addIngredientSlot = () => {
        ingredients.push({
            ingredient: "",
            amount: 0,
            measurement: Measurement.Gram,
        });

        ingredients = ingredients;
    };
</script>

<section class="flex w-full h-fit justify-center items-center">
    <div class="shadow-one flex flex-col gap-2 w-[580px] mt-20 p-4 bg-white">
        <div>
            <p class="text-sm tracking-wider font-semibold">title</p>
            <TextSinglelineInput
                placeholder={"Give your splendid recipe a title!"}
                extraClass="text-xs !py-px !pl-1"
            />
        </div>
        <div>
            <p class="text-sm tracking-wider font-semibold">description</p>
            <TextSinglelineInput
                placeholder={"Tell us a little about this dish"}
                extraClass="text-xs !py-px !pl-1"
            />
        </div>
        <div>
            <p class="text-sm tracking-wider font-semibold">ingredients</p>
            <div class="mt-1">
                {#each ingredients as ingredient, i}
                    <div class="w-full bg-gray-100 mb-2 flex flex-row">
                        <div class="h-fit w-1/2 p-1">
                            <TextSinglelineInput
                                bind:value={ingredient.ingredient}
                                placeholder={"Ingredient"}
                                extraClass="text-xs !py-px !pl-1"
                            />
                            <div class="flex flex-row mt-2">
                                <NumberInput
                                    bind:value={ingredient.amount}
                                    placeholder={"Amount"}
                                    extraClass="text-xs !py-px !pl-1 w-1/2 !mr-2"
                                />
                                <Combobox
                                    values={Object.values(Measurement)}
                                    extraClass="!h-full"
                                />
                            </div>
                        </div>
                        <div class="w-1/2 flex items-center justify-end pr-2">
                            <button
                                on:click={() => {
                                    ingredients.splice(i, 1);
                                    ingredients = ingredients;
                                }}
                                class="cursor-pointer h-12 w-12 bg-[var(--yellow)] ml-4 hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center"
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
            </div>
            <button
                on:click={addIngredientSlot}
                class="w-fit px-3 py-1 bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex flex-row gap-2 items-center mt-2"
            >
                <p class="text-2xl font-semibold">+</p>
            </button>
        </div>
    </div>
</section>
