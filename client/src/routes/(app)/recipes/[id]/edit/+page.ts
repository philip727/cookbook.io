import { goto } from "$app/navigation";
import { getBearer} from "$lib/login";
import { requestRecipeEdit } from "$lib/routes/recipe";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, params }) => {
    let bearer = getBearer();
    if (bearer == null) {
        return goto("/");
    };

    return await requestRecipeEdit(bearer, parseInt(params.id), fetch);
}
