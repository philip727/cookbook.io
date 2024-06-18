import type { PublicUserProfileDetails } from "$lib/profile"

export type RecipePreview = {
    poster: PublicUserProfileDetails,
    id: number,
    title: string,
    description: string,
    thumbnail: string | null
}
