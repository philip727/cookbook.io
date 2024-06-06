import { writable } from 'svelte/store';
export const JWT_TOKEN_KEY = "jwt_authorization_token";

export type User = {
    uid: number,
    username: string,
}
export const user = writable<User | null>(null);
