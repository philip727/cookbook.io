import { writable } from 'svelte/store';
import { endpoint } from './api';
export const JWT_TOKEN_KEY = "jwt_authorization_token";

export type User = {
    uid: number,
    username: string,
}

export const user = writable<User | null>(null);

export const requestJWTVerification = async (key: string): Promise<User | null> => {
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
