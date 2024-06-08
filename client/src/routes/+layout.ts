export const ssr = false;

import { JWT_TOKEN_KEY, requestJWTVerification, user } from "$lib/login";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ fetch }) => {
    let key = window.localStorage[JWT_TOKEN_KEY];
    if (key == null) {
        return;
    }

    let jwtClaims = await requestJWTVerification(key, fetch);
    if (jwtClaims == null) {
        return;
    }

    user.set({
        ...jwtClaims
    });
}
