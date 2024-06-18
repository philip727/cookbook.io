import type { PublicUserProfileDetails } from "$lib/profile"
import type { Ingredient } from "../create/helpers"

export type Recipe = {
    title: string,
    description: string,
    ingredients: Array<Ingredient>,
    steps: Array<{order: number, step_details: string}>,
    poster: PublicUserProfileDetails,
    id: number,
    thumbnail_path: string,
}
