import { endpoint } from "$lib/api";
import type { ResponseError } from "$lib/routes/error";
import type { PageLoad } from "./$types";
import type { RecipePreview } from "./helpers";

export const load: PageLoad = async ({ fetch }) => {
    let response = await fetch(
        endpoint("/recipes/all?offset=0&limit=10"),
    )

    let data = await response.json();
    if (!response.ok) {
        return {
            error: data.error,
            description: data.description,
        } as ResponseError;
    }

    return {
        recipes: data as RecipePreview[]
    };
}

