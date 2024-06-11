import { writable } from 'svelte/store';
import { endpoint } from './api';
export const JWT_TOKEN_KEY = "jwt_authorization_token";

export type JWTClaims = {
    uid: number,
    username: string,
}

export const user = writable<JWTClaims | null>(null);

export const requestJWTVerification = async (key: string, fetch: Function): Promise<JWTClaims | null> => {
    try {
        let bearer = "Bearer " + key;
        console.log(bearer);
        let response = await fetch(
            endpoint("/account/verify") as string,
            {
                headers: {
                    Authorization: bearer,
                },
            },
        );

        if (!response.ok) {
            let text = await response.text();
            console.log(text);
            return null;
        }

        const data = await response.json();
        return data;
    } catch (error) {
        console.log(error);

        return null
    }
}
