export const ssr = false;

import { JWT_TOKEN_KEY, attemptJWTLogin, user } from "$lib/login";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ fetch }) => {
    let key = window.localStorage[JWT_TOKEN_KEY];
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
