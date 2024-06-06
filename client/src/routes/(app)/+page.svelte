<script>
    import { JWT_TOKEN_KEY } from "$lib/login";
    import axios from "axios";
    import { onMount } from "svelte";

    const test = async () => {
        let key = window.localStorage[JWT_TOKEN_KEY];
        if (key == null) {
            return;
        }

        try {
            let bearer = "Bearer " + key;
            console.log(bearer);
            let response = await window.fetch(
                "http://127.0.0.1:8080/v1/account/verify",
                {
                    headers: {
                        Authorization: bearer,
                    },
                },
            );

            const data = await response.json();
            console.log(data);
            if (!response.ok) {
                console.log(data.error, data.description);
                return;
            }

            console.log(data);
        } catch (error) {
            console.log(error);
        }
    };
</script>

<button
    on:click={() => {
        test();
    }}>YAY</button
>
