import * as env from '$env/static/public'
export const endpoint = (ep: string): string => {
    return env.PUBLIC_SERVER_URL + env.PUBLIC_SERVER_API_VERSION + ep
}
