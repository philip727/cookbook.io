import { writable } from 'svelte/store';
import { endpoint } from './api';
import type { PublicUserProfileDetails } from './profile';
export const JWT_TOKEN_KEY = "jwt_authorization_token";

export type JWTClaims = {
    uid: number,
    username: string,
}

export const user = writable<PublicUserProfileDetails | null>(null);

export const requestJWTVerification = async (key: string, fetch: Function): Promise<JWTClaims | null> => {
    try {
        let bearer = "Bearer " + key;
        let response = await fetch(
            endpoint("/account/verify") as string,
            {
                headers: {
                    Authorization: bearer,
                },
            },
        );

        if (!response.ok) {
            //let _ = await response.text();
            return null;
        }

        // return jwt claims
        const data = await response.json() as JWTClaims;
        return data;
    } catch (error) {
        return null
    }
}

// Attempt to login and get user details
export const attemptJWTLogin =  async (key: string, fetch: Function): Promise<PublicUserProfileDetails | null> => {
    let jwtClaims = await requestJWTVerification(key, fetch);
    if (jwtClaims == null) {
        return null;
    }

    let localUserDetails = await fetch(endpoint(`/users/${jwtClaims.uid}`));
    if (!localUserDetails.ok) {
        return null;
    };

    let userDetails = await localUserDetails.json() as PublicUserProfileDetails;

    return userDetails;
}
