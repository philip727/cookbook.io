import { endpoint } from "$lib/api";
import type { ResponseError } from "../../../../components/ErrorBox";
import type { PageLoad } from "./$types";
import type { Recipe } from "./helper";

export const load: PageLoad = async ({ fetch, params }) => {
    let response = await fetch(endpoint(`/recipes/${params.id}`));

    let data = await response.json();
    if (!response.ok) {
        return {
            error: data.error,
            description: data.description
        } as ResponseError
    }

    return {
        recipe: data as Recipe,
    };
}
