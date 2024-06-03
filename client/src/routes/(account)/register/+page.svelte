<script lang="ts">
    import { redirect } from "@sveltejs/kit";
    import ErrorBox, { type Error } from "../../../components/ErrorBox.svelte";
    import HiddenSinglelineInput from "../../../components/HiddenSinglelineInput.svelte";
    import SuccessBox from "../../../components/SuccessBox.svelte";
    import type { Success } from "../../../components/SuccessBox.svelte";
    import TextSinglelineInput from "../../../components/TextSinglelineInput.svelte";
    import Title from "../../../components/Title.svelte";
    import { goto } from "$app/navigation";

    let registerError: Error | null = null;
    let registerSucess: Success | null = null;
    let formData = {
        username: "",
        email: "",
        password: "",
        confirm_password: "",
    };

    async function handleSubmit(
        event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement },
    ) {
        event.preventDefault();

        let response = await window.fetch(
            "http://127.0.0.1:8080/v1/users/register",
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "Access-Control-Allow-Origin": "*",
                },
                body: JSON.stringify(formData),
            },
        );

        const data = await response.json();
        if (!response.ok) {
            registerError = {
                error: data.error,
                description: data.description,
            };
            return;
        }

        registerSucess = {
            title: "Welcome",
            description: "Thank you for registering with us!",
        };

        await goto("/login");
    }
</script>

<div class="flex flex-col justify-center items-center w-full h-screen">
    <Title textClass="text-4xl" />
    <form on:submit={handleSubmit} class="w-80 h-fit shadow-one mt-4 px-4 py-3">
        <h1 class="text-3xl font-bold">Register</h1>
        {#if registerError != null && registerSucess == null}
            <ErrorBox
                extraClass="mt-6"
                error={registerError.error}
                description={registerError.description}
            />
        {/if}
        {#if registerSucess != null}
            <SuccessBox
                extraClass="mt-6"
                title="Welcome"
                description="Thank you for registering with us!"
            />
        {/if}
        <TextSinglelineInput
            name="username"
            placeholder="Username"
            extraClass="mt-4"
            bind:value={formData.username}
        />
        <TextSinglelineInput
            name="email"
            placeholder="Email"
            extraClass="mt-6"
            bind:value={formData.email}
        />
        <HiddenSinglelineInput
            name="password"
            placeholder="Password"
            extraClass="mt-6"
            bind:value={formData.password}
        />
        <HiddenSinglelineInput
            name="confirm_password"
            placeholder="Confirm password"
            extraClass="mt-6"
            bind:value={formData.confirm_password}
        />
        <div class="mt-1">
            <p class=" font-medium text-sm">
                Already have an account?
                <a href="/login" class="text-[var(--green)]">Login</a>
            </p>
        </div>
        <button
            type="submit"
            class="w-full bg-white hover:bg-[var(--green)] py-2 mt-6 border-2 border-[var(--green)] duration-150 transition-all hover:text-white"
        >
            <p class="text-base font-semibold">REGISTER</p>
        </button>
    </form>
</div>
