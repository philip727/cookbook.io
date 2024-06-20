import type { MinimalUserDetails } from "$lib/routes/user"
import type { Ingredient } from "../create/helpers"

export type Recipe = {
    poster: MinimalUserDetails,
    id: number,
    thumbnail_path: string,
    recipe: {
        title: string,
        description: string,
        ingredients: Array<Ingredient>,
        steps: Array<{ order: number, step_details: string }>,
    }
}
