import { writable } from 'svelte/store';
import { endpoint } from './api';
import type { UserDetails } from './routes/user';
export const JWT_TOKEN_KEY = "jwt_authorization_token";

export type JWTClaims = {
    uid: number,
    username: string,
}

export const user = writable<UserDetails | null>(null);

export const asBearer = (key: string): string => {
    return "Bearer " + key;
}

export const getBearer = (): string | null => {
    let key = window.localStorage.getItem(JWT_TOKEN_KEY);
    if (key == null) {
        return null;
    }

    return "Bearer " + key;
}

export const requestJWTVerification = async (key: string, fetch: Function): Promise<JWTClaims | null> => {
    try {
        let response = await fetch(
            endpoint("/account/verify") as string,
            {
                headers: {
                    Authorization: key,
                },
            },
        );

        if (!response.ok) {
            //let _ = await response.text();
            window.localStorage.removeItem(JWT_TOKEN_KEY);
            return null;
        }

        // return jwt claims
        return await response.json() as JWTClaims;
    } catch (error) {
        return null
    }
}

// Attempt to login and get user details
export const attemptJWTLogin = async (key: string, fetch: Function): Promise<UserDetails | null> => {
    let jwtClaims = await requestJWTVerification(key, fetch);
    if (jwtClaims == null) {
        return null;
    }

    let localUserDetails = await fetch(endpoint(`/users/${jwtClaims.uid}`));
    if (!localUserDetails.ok) {
        return null;
    };

    let userDetails = await localUserDetails.json() as UserDetails;

    return userDetails;
}
