import { endpoint } from "$lib/api";
import type { ResponseError } from "./error";
import type { MinimalUserDetails } from "./user";

export type Recipe = {
    type: "RECIPE",
    poster: MinimalUserDetails,
    id: number,
    thumbnail: string,
    recipe: {
        title: string,
        description: string,
        ingredients: Array<Ingredient>,
        steps: Array<{ order: number, step_details: string }>,
    }
}

export type RecipeCollection<T> = {
    type: "RECIPE_COLLECTION",
    collection: T[],
}

export type RecipePreview = {
    poster: MinimalUserDetails,
    id: number,
    title: string,
    description: string,
    thumbnail: string | null
}

export type Ingredient = {
    ingredient: string,
    amount: number,
    measurement: Measurement
}

export enum Measurement {
    Millilitre = "Millilitre",
    Litre = "Litre",
    Teaspoon = "Teaspoon",
    Tablespoon = "Tablespoon",
    FluidOz = "FluidOz",
    Pint = "Pint",
    Gallon = "Gallon",
    Milligram = "Milligram",
    Gram = "Gram",
    Kilogram = "Kilogram",
    Pound = "Pound",
    Ounce = "Ounce",
    Celsius = "Celsius",
    Fahrenheit = "Fahrenheit",
    Piece = "Piece",
}

export const getRecipePage = async (page: number, fetch: Function) => {

}

export const getRecipe = async (id: number, fetch: Function): Promise<ResponseError | Recipe> => {
    let response = await fetch(endpoint(`/recipes/${id}`));

    let data = await response.json();
    if (!response.ok) {
        return data as ResponseError
    }

    return data as Recipe;
}

export const getRecipesByUser = async (uid: number, fetch: Function): Promise<ResponseError | RecipeCollection<RecipePreview>> => {
    let response = await fetch(endpoint(`/recipes/by/${uid}`));

    let data = await response.json();
    if (!response.ok) {
        return data as ResponseError
    }

    return { collection: data } as RecipeCollection<RecipePreview>;
}
