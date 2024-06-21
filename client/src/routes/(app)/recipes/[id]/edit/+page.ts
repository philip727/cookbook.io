import { goto } from "$app/navigation";
import { getBearer } from "$lib/login";
import { requestRecipeEdit } from "$lib/routes/recipe";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, params }) => {
    let bearer = getBearer();
    if (bearer == null) {
        return goto("/");
    };

    let data = await requestRecipeEdit(bearer, parseInt(params.id), fetch);
    if (data.type == "REQUEST_RECIPE_EDIT" && !data.authorized) {
        return goto("/");
    }

    return data;
}
