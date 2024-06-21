import { goto } from "$app/navigation";
import { getBearer, requestJWTVerification } from "$lib/login";
import { getRecipesByUser } from "$lib/routes/recipe";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }) => {
    let bearer = getBearer();
    if (bearer == null) {
        return goto("/");
    }


    let claims = await requestJWTVerification(bearer, fetch);
    if (claims == null) {
        return goto("/");
    }

    return await getRecipesByUser(claims.uid, fetch); 
}
