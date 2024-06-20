import { isResponseError, type ResponseError } from "$lib/routes/error";
import { getRecipe, type Recipe } from "$lib/routes/recipe";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, params }) => {
    let response = await getRecipe(parseInt(params.id), fetch);

    if (isResponseError(response)) {
        return response as ResponseError;
    }

    return response as Recipe;
}
