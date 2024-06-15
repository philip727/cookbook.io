import type { PublicUserProfileDetails } from "$lib/profile"

export type RecipeInfo = {
    poster: PublicUserProfileDetails,
    id: number,
    title: string,
    description: string,
    thumbnail: string | null
}
