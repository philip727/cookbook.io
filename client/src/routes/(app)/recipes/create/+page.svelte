<script lang="ts">
    import Combobox from "../../../../components/Combobox.svelte";
    import NumberInput from "../../../../components/NumberInput.svelte";
    import TextSinglelineInput from "../../../../components/TextSinglelineInput.svelte";
    import { Measurement, type Ingredient } from "./helpers";
    import uploadArrow from "$lib/images/upload-arrow.svg";
    import TextMultilineInput from "../../../../components/TextMultilineInput.svelte";
    import { JWT_TOKEN_KEY, getBearer } from "$lib/login";
    import { endpoint } from "$lib/api";
    import { goto } from "$app/navigation";
    import { HttpStatusCode } from "axios";

    let title: string | null = null;
    let description: string | null = null;
    let ingredients: Array<Ingredient> = [];
    let steps: Array<string> = [];
    let files: FileList;

    const addIngredientSlot = () => {
        ingredients.push({
            ingredient: "",
            amount: 0,
            measurement: Measurement.Gram,
        });

        ingredients = ingredients;
    };

    const addStepSlot = () => {
        steps.push("");

        steps = steps;
    };

    const uploadRecipe = async () => {
        let form = new FormData();

        if (files && files.item(0)) {
            form.append("thumbnail", files.item(0) as File);
        }

        let orderedSteps = []
        for (let i = 0; i < steps.length; i++) {
            const step = steps[i];

            orderedSteps.push({ order: i, step_details: step });
        }

        let recipe = {
            title: title,
            description: description,
            ingredients: ingredients,
            steps: orderedSteps,
        }
        form.append("recipe", JSON.stringify(recipe));

        let bearer = getBearer();
        if (!bearer) {
            return;
        }

        const response = await window.fetch(endpoint("/recipes/create"), {
            method: "POST",
            body: form,
            headers: {
                Authorization: bearer,
            }
        })

        if (!response.ok) {
            if (response.status == HttpStatusCode.Unauthorized) {
                window.localStorage.removeItem(JWT_TOKEN_KEY);
                goto("/");
            }

            return;
        }

    }
</script>

<section class="flex w-full h-fit justify-center items-center">
    <form on:submit={uploadRecipe} class="shadow-one flex flex-col gap-2 w-[580px] mt-20 p-4 bg-white">
        <h1 class="text-3xl font-bold">Create a recipe</h1>
        <article>
            <p class="text-sm tracking-wider font-semibold">title</p>
            <TextSinglelineInput
                bind:value={title}
                placeholder={"Give your splendid recipe a title!"}
                extraClass="text-xs !py-px !pl-1"
            />
        </article>
        <article>
            <p class="text-sm tracking-wider font-semibold">description</p>
            <TextSinglelineInput
                bind:value={description}
                placeholder={"Tell us a little about this dish"}
                extraClass="text-xs !py-px !pl-1"
            />
        </article>
        <article>
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
                        <div class="w-1/2 flex items-center justify-end p-1">
                            <button
                                on:click={() => {
                                    ingredients.splice(i, 1);
                                    ingredients = ingredients;
                                }}
                                class="cursor-pointer h-full w-fit bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center px-3"
                                type="button"
                            >
                                <p class="font-semibold tracking-wider text-xs">
                                    REMOVE
                                </p>
                            </button>
                        </div>
                    </div>
                {/each}
                <button
                    on:click={addIngredientSlot}
                    type="button"
                    class="w-fit px-3 py-1 bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex flex-row gap-2 items-center mt-1"
                >
                    <p class="text-2xl font-semibold">+</p>
                </button>
            </div>
        </article>
        <article class="mt-2">
            <p class="text-sm tracking-wider font-semibold">steps</p>
            <div class="mt-1">
                {#each steps as step, i}
                    <div class="w-full bg-gray-100 mb-2 flex flex-row h-20">
                        <div class="w-3/4 p-1">
                            <TextMultilineInput
                                bind:value={step}
                                placeholder={"Tell us about this step in the recipe"}
                                extraClass="text-xs !py-px !pl-1 !h-full"
                            />
                        </div>
                        <div
                            class="w-1/4 flex items-start justify-end p-1 gap-2"
                        >
                            <div class="w-full h-full flex flex-col gap-2">
                                <button
                                    on:click={() => {
                                        if (i == 0) {
                                            return;
                                        }

                                        // Swap with previous step
                                        let temp = steps[i];
                                        steps[i] = steps[i - 1];
                                        steps[i - 1] = temp;

                                        steps = steps;
                                    }}
                                    class="cursor-pointer h-1/2 w-full bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center px-3"
                                    type="button"
                                >
                                    <p>^</p>
                                </button>
                                <button
                                    on:click={() => {
                                        if (i == steps.length - 1) {
                                            return;
                                        }

                                        // Swap with next step
                                        let temp = steps[i];
                                        steps[i] = steps[i + 1];
                                        steps[i + 1] = temp;

                                        steps = steps;
                                    }}
                                    class="cursor-pointer h-1/2 w-full bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center px-3"
                                    type="button"
                                >
                                    <p>∨</p>
                                </button>
                            </div>
                            <button
                                on:click={() => {
                                    steps.splice(i, 1);
                                    steps = steps;
                                }}
                                class="cursor-pointer h-full w-fit bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center px-3"
                                type="button"
                            >
                                <p class="font-semibold tracking-wider text-xs">
                                    REMOVE
                                </p>
                            </button>
                        </div>
                    </div>
                {/each}
                <button
                    on:click={addStepSlot}
                    class="w-fit px-3 py-1 bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex flex-row gap-2 items-center mt-1"
                    type="button"
                >
                    <p class="text-2xl font-semibold">+</p>
                </button>
            </div>
        </article>
        <article>
            <p class="text-sm tracking-wider font-semibold">thumbnail</p>
            <div class="w-10">
                <label for="thumbnail-upload">
                    <div
                        class="cursor-pointer h-10 w-10 bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center mt-1"
                    >
                        <img
                            class="h-5 w-5"
                            src={uploadArrow}
                            alt="Upload button"
                        />
                    </div>
                </label>
            </div>
            <input
                id="thumbnail-upload"
                class="h-12 w-12 hidden"
                type="file"
                accept="image/png, image/jpeg"
                bind:files
            />
        </article>
        <button
            class="w-fit px-3 py-1 bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200 flex flex-row gap-2 items-center mt-1"
            type="submit"
        >
            <p class="text-base font-semibold">SAVE CHANGES</p>
        </button>
    </form>
</section>
