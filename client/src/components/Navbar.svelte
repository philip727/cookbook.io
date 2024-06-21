<script lang="ts">
    import Title from "./Title.svelte";
    import { type JWTClaims, user, JWT_TOKEN_KEY } from "$lib/login";
    import UserPreview from "./UserPreview.svelte";
    import { onDestroy, onMount } from "svelte";
    import { goto } from "$app/navigation";
    import type { UserDetails } from "$lib/routes/user";
    let showDropdown = false;

    let signedInUser: UserDetails | null = null;
    user.subscribe(val => {
        console.log(val);
        signedInUser = val;
    })

    function closeDropdownWhenNotClicked(event: MouseEvent) {
        if (!event.target) {
            return;
        }

        const target = event.target as HTMLElement;
        if (!target.closest(".dropdown") && !target.closest(".dropdown-btn")) {
            showDropdown = false;
        }
    }

    onMount(() => {
        document.addEventListener("click", closeDropdownWhenNotClicked);
    });

    onDestroy(() => {
        document.removeEventListener("click", closeDropdownWhenNotClicked);
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
        {#if signedInUser}
            <div class="relative dropdown-btn">
                <button
                    on:click={() => {
                        showDropdown = !showDropdown;
                    }}
                >
                    <UserPreview user={signedInUser} />
                </button>
                {#if showDropdown}
                    <div
                        class="shadow-one absolute w-fit h-fit mt-7 right-0 flex flex-col items-end dropdown py-2 px-2 gap-2 font-medium text-sm bg-white"
                    >
                        <a
                            on:click={() => (showDropdown = false)}
                            href={`/profile/${signedInUser.uid}`}
                            draggable="false"
                        >
                            <h1 class="hover:text-[var(--green)] duration-150">
                                PROFILE
                            </h1>
                        </a>
                        <a
                            on:click={() => (showDropdown = false)}
                            href={`/recipes/yours`}
                            draggable="false"
                        >
                            <h1 class="hover:text-[var(--green)] duration-150">
                                MY RECIPES
                            </h1>
                        </a>
                        <a
                            on:click={() => (showDropdown = false)}
                            href="/account"
                            draggable="false"
                        >
                            <h1 class="hover:text-[var(--green)] duration-150">
                                ACCOUNT
                            </h1>
                        </a>
                        <button
                            on:click={() => {
                                localStorage.removeItem(JWT_TOKEN_KEY);
                                user.set(null);
                                goto("/")
                            }}
                            class="w-32 bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] py-2 transition-all duration-200"
                            draggable="false"
                        >
                            <p class="text-sm">LOGOUT</p>
                        </button>
                    </div>
                {/if}
            </div>
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
