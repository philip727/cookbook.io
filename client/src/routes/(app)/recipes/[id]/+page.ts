import { isResponseError, type ResponseError } from "$lib/routes/error";
import { getRecipe } from "$lib/routes/recipe";
import type { PageLoad } from "./$types";
import type { Recipe } from "./helper";

export const load: PageLoad = async ({ fetch, params }) => {
    let response = await getRecipe(parseInt(params.id), fetch);

    if (isResponseError(response)) {
        return response as ResponseError;
    }

    return response as Recipe;
}
