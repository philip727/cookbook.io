import { getRecipesByUser } from "$lib/routes/recipe";
import { getUser } from "$lib/routes/user";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, params }) => {
    return {
        user: await getUser(parseInt(params.id), fetch),
        recipes: await getRecipesByUser(parseInt(params.id), fetch)
    }
}
