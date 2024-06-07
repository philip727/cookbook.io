import { JWT_TOKEN_KEY, user } from "$lib/login";
import type { PageLoad } from "./$types";
import '$lib/config'


export const load: PageLoad = async ({ fetch }) => {
    let key = window.localStorage[JWT_TOKEN_KEY];
    if (key == null) {
        return;
    }

    try {
        let bearer = "Bearer " + key;
        console.log(bearer);
        let response = await fetch(
            "http://127.0.0.1:8080/v1/account/verify",
            {
                headers: {
                    Authorization: bearer,
                },
            }
        );

        if (!response.ok) {
            let text = await response.text();
            console.log(text);
            return;
        }

        const data = await response.json();
        user.set({
            ...data
        });
    } catch (error) {
        console.log(error);
    }

};

