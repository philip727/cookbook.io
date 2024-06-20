<script lang="ts">
    import { endpoint } from "$lib/api";
    import UserPreview from "../../../../components/UserPreview.svelte";
    import { Measurement, type Ingredient } from "../create/helpers";
    import type { PageData } from "./$types";

    export let data: PageData;

    console.log(data);

    const formatIngredient = (ingredient: Ingredient) => {
        let following = ingredient.measurement as string;

        switch (ingredient.measurement) {
            case Measurement.Gram: {
                following = "g";
                break;
            }
            case Measurement.Millilitre: {
                following = "ml";
                break;
            }
            case Measurement.Litre: {
                following = "l";
                break;
            }
            case Measurement.Teaspoon: {
                following = "tsp.";
                break;
            }
            case Measurement.Tablespoon: {
                following = "tbsp.";
                break;
            }
            case Measurement.FluidOz: {
                following = "fl. oz.";
                break;
            }
            case Measurement.Pint: {
                following = "pt";
                break;
            }
            case Measurement.Gallon: {
                following = "gal";
                break;
            }
            case Measurement.Milligram: {
                following = "mg";
                break;
            }
            case Measurement.Kilogram: {
                following = "kg";
                break;
            }
            case Measurement.Pound: {
                following = "lbs";
                break;
            }
            case Measurement.Ounce: {
                following = "oz.";
                break;
            }
            case Measurement.Celsius: {
                following = "C";
                break;
            }
            case Measurement.Fahrenheit: {
                following = "F";
                break;
            }
            case Measurement.Piece: {
                if (ingredient.amount > 1) {
                    following = "pcs";
                    break;
                }

                following = "piece";
            }
        }

        return `${ingredient.ingredient} ${ingredient.amount}${following}`;
    };
</script>

<section>
    <div class="mt-32">
        {#if data.recipe}
            <div class="w-fit">
                <article class="w-fit">
                    <div class="w-full flex justify-end">
                        <UserPreview user={data.recipe.poster} />
                    </div>
                    <h1 class="text-4xl font-bold mt-2">
                        {data.recipe.recipe.title}
                    </h1>
                    <p class="text-lg font-light">
                        {data.recipe.recipe.description}
                    </p>
                    <img
                        alt="Recipe thumbnail"
                        class="w-[600px] h-96 mt-2"
                        src={endpoint(
                            `/thumbnails/${data.recipe.thumbnail_path}`,
                        )}
                    />
                </article>
                <p class="mt-4 text-3xl font-semibold">Ingredients</p>
                <ul class="list-inside list-disc mt-2">
                    {#each data.recipe.recipe.ingredients as ingredient}
                        <li>{formatIngredient(ingredient)}</li>
                    {/each}
                </ul>
            </div>
        {/if}
    </div>
</section>
