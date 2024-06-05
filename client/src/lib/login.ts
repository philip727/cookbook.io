import * as jose from 'jose'
export const JWT_TOKEN_KEY = "jwt_authorization_token";

export function storeJWT(jwt: string) {
    window.localStorage[JWT_TOKEN_KEY] = jwt;
}

export const decodeLocalJWT = (): boolean => {
    let jwt_token = window.localStorage[JWT_TOKEN_KEY];
    if (!jwt_token) {
        return false;
    };

    const claims = jose.decodeJwt(jwt_token);
    console.log(claims);

    return true;
}
