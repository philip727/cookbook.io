import { isResponseError, type ResponseError } from "$lib/routes/error";
import { getUser, type UserDetails } from "$lib/routes/user";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({fetch, params}) => {
    let response = await getUser(parseInt(params.id), fetch);

    console.log(response);

    if (isResponseError(response)) {
        return response as ResponseError;
    }

    return response as UserDetails
}
