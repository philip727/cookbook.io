import { endpoint } from "$lib/api";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ fetch }) => {
    let response = await fetch(
        endpoint("/recipes/all?offset=0&limit=10"),
    )

    let data = await response.json();
    console.log(data);
}
