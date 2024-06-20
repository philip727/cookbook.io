export type ResponseError = {
    type: "RESPONSE_ERROR";
    error: string;
    description: string;
};

export const isResponseError = (response: any) => {
    return (response as ResponseError).error !== undefined && (response as ResponseError).description !== undefined;
}
