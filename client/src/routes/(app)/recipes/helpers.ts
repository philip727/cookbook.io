import type { MinimalUserDetails } from "$lib/routes/user"

export type RecipePreview = {
    poster: MinimalUserDetails,
    id: number,
    title: string,
    description: string,
    thumbnail: string | null
}
