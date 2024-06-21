import { isResponseError, type ResponseError } from "$lib/routes/error";
import { getRecipesByUser, type RecipeCollection, type RecipePreview } from "$lib/routes/recipe";
import { getUser } from "$lib/routes/user";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, params }) => {
    return {
        user: await getUser(parseInt(params.id), fetch),
        recipes: await getRecipesByUser(parseInt(params.id), fetch)
    }
}
