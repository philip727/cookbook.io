import { isResponseError, type ResponseError } from "$lib/routes/error";
import { getRecipe } from "$lib/routes/recipe";
import type { PageLoad } from "./$types";
import type { Recipe } from "./helper";

export const load: PageLoad = async ({ fetch, params }) => {
    let response = await getRecipe(parseInt(params.id), fetch);

    if (isResponseError(response)) {
        return {
            error: (response as ResponseError).error,
            description: (response as ResponseError).description
        } as ResponseError;
    }

    return {
        recipe: response as Recipe,
    };
}
