export const ssr = false;

import { endpoint } from "$lib/api";
import { JWT_TOKEN_KEY, requestJWTVerification, user } from "$lib/login";
import type { PublicUserProfileDetails } from "$lib/profile";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ fetch }) => {
    let key = window.localStorage[JWT_TOKEN_KEY];
    if (key == null) {
        return;
    }

    let jwtClaims = await requestJWTVerification(key, fetch);
    if (jwtClaims == null) {
        window.localStorage[JWT_TOKEN_KEY] = null;
        return;
    }

    let localUserDetails = await fetch(endpoint(`/users/${jwtClaims.uid}`));
    if (!localUserDetails.ok) {
        window.localStorage[JWT_TOKEN_KEY] = null;
        return;
    };

    let userDetails = await localUserDetails.json() as PublicUserProfileDetails;
    user.set({
        ...userDetails
    });
}
