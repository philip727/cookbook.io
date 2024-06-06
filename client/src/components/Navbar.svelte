<script lang="ts">
    import Title from "./Title.svelte";
    import { type User, user } from "$lib/login";
    import UserPreview from "./UserPreview.svelte";
    let currentUser: User | null = null;

    user.subscribe((value) => {
        currentUser = value;
    });
</script>

<div
    class="w-full h-20 border-b border-[var(--green)] px-60 flex flex-row justify-center items-center absolute top-0 tracking-wide font-bold"
>
    <section class="flex flex-row text-3xl w-1/3 justify-start">
        <Title />
    </section>
    <section
        class="flex flex-row gap-8 text-lg w-1/3 justify-center font-semibold"
    >
        <a class="" href="/">HOME</a>
        <a class="" href="/recipes">RECIPES</a>
    </section>
    <section class="flex flex-row gap-8 w-1/3 justify-end">
        {#if currentUser}
            <UserPreview user={currentUser} />
        {:else}
            <a
                href="/login"
                class="w-fit px-4 py-2 bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] duration-200"
            >
                <p class="text-base font-semibold">LOGIN</p>
            </a>
        {/if}
    </section>
</div>
