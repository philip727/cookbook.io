import { endpoint } from "$lib/api";
import { JWT_TOKEN_KEY, } from "$lib/login";
import type { PageLoad } from "./$types";

export type AccountInfo = {
    uid: string,
    username: string,
    email: string,
}

export const load: PageLoad = async ({ fetch }) => {
    let key = window.localStorage[JWT_TOKEN_KEY];
    if (key == null) {
        return;
    }

    let bearer = "Bearer " + key;
    let response = await fetch(
        endpoint("/account/"),
        {
            headers: {
                Authorization: bearer,
            },
        },
    );

    if (!response.ok) {
        return { account: null };
    }

    let data = await response.json() as AccountInfo;
    return {
        account: data,
    };
}
