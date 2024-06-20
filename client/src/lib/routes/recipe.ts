import { endpoint } from "$lib/api";
import type { Recipe } from "../../routes/(app)/recipes/[id]/helper";
import type { ResponseError } from "./error";

export const getRecipePage = async (page: number, fetch: Function) => {

}

export const getRecipe = async (id: number, fetch: Function): Promise<ResponseError | Recipe> => {
    let response = await fetch(endpoint(`/recipes/${id}`));

    let data = await response.json();
    if (!response.ok) {
        return {
            error: data.error,
            description: data.description
        } as ResponseError
    }

    return data as Recipe;
}
