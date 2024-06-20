import { endpoint } from "$lib/api";
import type { ResponseError } from "./error";

export type UserDetails = {
    type: "USER_DETAILS",
    uid: number,
    username: string,
    bio: string | null,
    location: string | null,
    picture: string | null,
    pronouns: string | null,
}

export type MinimalUserDetails = {
    type: "MINIMAL_USER_DETAILS",
    uid: number,
    username: string,
    picture: string | null,

}

export const isUserDetails = (response: any) => {
    return (response) && (response as UserDetails).username !== undefined 
        && (response as UserDetails).bio !== undefined
        && (response as UserDetails).location !== undefined
        && (response as UserDetails).picture !== undefined
        && (response as UserDetails).pronouns !== undefined;
}

export const getUser = async (id: number, fetch: Function): Promise<ResponseError | UserDetails> => {
    let response = await fetch(endpoint(`/users/${id}`));

    let data = await response.json();
    if (!response.ok) {
        return {
            error: data.error,
            description: data.description
        } as ResponseError
    }

    return data as UserDetails;
}
