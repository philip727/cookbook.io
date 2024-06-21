export const ssr = false;

import { JWT_TOKEN_KEY, attemptJWTLogin, getBearer, user } from "$lib/login";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ fetch }) => {
    let key = getBearer(); 
    if (key == null) {
        return;
    }

    let userDetails = await attemptJWTLogin(key, fetch);
    if (userDetails == null) {
        return;
    }

    user.set({
        ...userDetails
    });
}
