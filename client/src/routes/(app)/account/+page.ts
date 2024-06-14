import { endpoint } from "$lib/api";
import { JWT_TOKEN_KEY, } from "$lib/login";
import { HttpStatusCode } from "axios";
import type { PageLoad } from "./$types";
import { goto } from "$app/navigation";

export type AccountInfo = {
    username: string,
    bio: string | null,
    location: string | null,
    pronouns: string | null,
    picture: string | null,
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
        if (response.status == HttpStatusCode.Unauthorized) {
            window.localStorage[JWT_TOKEN_KEY] = null;
            goto("/");
        }

        return { account: null };
    }

    let data = await response.json() as AccountInfo;
    return {
        account: data,
    };
}
