export const ssr = false;

import { JWT_TOKEN_KEY, requestJWTVerification, user } from "$lib/login";


export const load = async () => {
    let key = window.localStorage[JWT_TOKEN_KEY];
    if (key == null) {
        return;
    }

    let jwtClaims = await requestJWTVerification(key);
    if (jwtClaims == null) {
        return;
    }

    user.set({
        ...jwtClaims
    });
}
