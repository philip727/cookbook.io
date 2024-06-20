<script lang="ts">
    import TextSinglelineInput from "../../../components/TextSinglelineInput.svelte";
    import Title from "../../../components/Title.svelte";
    import HiddenSinglelineInput from "../../../components/HiddenSinglelineInput.svelte";
    import ErrorBox from "../../../components/ErrorBox.svelte";
    import { JWT_TOKEN_KEY, attemptJWTLogin, user } from "$lib/login";
    import { goto } from "$app/navigation";
    import { endpoint } from "$lib/api";
    import type { ResponseError } from "$lib/routes/error";

    let loginError: ResponseError | null = null;
    let formData = {
        identifier: "",
        password: "",
    };

    async function login(
        event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement },
    ) {
        event.preventDefault();

        let response = await window.fetch(endpoint("/users/login"), {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        });

        const data = await response.json();
        if (!response.ok) {
            loginError = {
                error: data.error,
                description: data.description,
            } as ResponseError;
            return;
        }

        // Verifies after login, not really necessary but yea
        // Makes sure nothin tampered with u feel
        window.localStorage[JWT_TOKEN_KEY] = data.jwt;

        let userDetails = await attemptJWTLogin(data.jwt, window.fetch);
        if (userDetails == null) {
            return;
        }

        user.set({
            ...userDetails,
        });

        await goto("/");
    }
</script>

<div class="flex flex-col justify-center items-center w-full h-screen">
    <Title textClass="text-4xl" />
    <form
        on:submit={login}
        method="POST"
        class="w-80 h-fit shadow-one mt-4 px-4 py-3"
    >
        <h1 class="text-3xl font-bold">Login</h1>
        {#if loginError != null}
            <ErrorBox
                extraClass="mt-6"
                error={loginError.error}
                description={loginError.description}
            />
        {/if}
        <TextSinglelineInput
            name="identifier"
            placeholder="Email or username"
            extraClass="mt-4"
            bind:value={formData.identifier}
        />
        <HiddenSinglelineInput
            name="password"
            placeholder="Password"
            extraClass="mt-6"
            bind:value={formData.password}
        />
        <div class="mt-1">
            <a
                href="/forgot-password"
                class="text-[var(--green)] font-medium text-sm"
                >Forgot password?</a
            >
        </div>
        <button
            class="w-full bg-white hover:bg-[var(--green)] py-2 mt-6 border-2 border-[var(--green)] duration-150 transition-all hover:text-white"
            type="submit"
        >
            <p class="text-base font-semibold">LOGIN</p>
        </button>
        <div class="flex flex-row w-full justify-center items-center mt-2">
            <hr class="border-b-1 w-full border-black" />
            <p class="mx-4 text-sm pb-[2px]">or</p>
            <hr class="border-b-1 w-full border-black" />
        </div>
        <a href="/register">
            <button
                class="w-full bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] py-2 mt-2 transition-all duration-200"
            >
                <p class="text-base font-semibold">SIGN UP</p>
            </button>
        </a>
    </form>
</div>
