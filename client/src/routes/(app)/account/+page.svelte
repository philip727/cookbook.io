<script lang="ts">
    import { endpoint } from "$lib/api";
    import { JWT_TOKEN_KEY } from "$lib/login";
    import ErrorBox, { type Error } from "../../../components/ErrorBox.svelte";
    import type { Success } from "../../../components/SuccessBox.svelte";
    import SuccessBox from "../../../components/SuccessBox.svelte";
    import TextMultilineInput from "../../../components/TextMultilineInput.svelte";
    import TextSinglelineInput from "../../../components/TextSinglelineInput.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;
    let submitError: Error | null = null;
    let submitSuccess: Success | null = null;
    let changeData = {
        display_name: data.account?.display_name,
        bio: data.account?.bio,
        pronouns: data.account?.pronouns,
        location: data.account?.location,
    };

    async function submitChanges(
        event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement },
    ) {
        event.preventDefault();

        let key = window.localStorage[JWT_TOKEN_KEY];
        if (key == null) {
            return;
        }

        let bearer = "Bearer " + key;

        // Need to null empty values for serialization on server side
        let json = {
            display_name:
                changeData.display_name == "" ? null : changeData.display_name,
            bio: changeData.bio == "" ? null : changeData.bio,
            pronouns: changeData.pronouns == "" ? null : changeData.pronouns,
            location: changeData.location == "" ? null : changeData.location,
        };

        let response = await window.fetch(endpoint("/account/update_details"), {
            method: "POST",
            headers: {
                Authorization: bearer,
                "Content-Type": "application/json",
            },
            body: JSON.stringify(json),
        });

        if (!response.ok) {
            const data = await response.json();
            submitError = {
                error: data.error,
                description: data.description,
            };
            return;
        }

        submitSuccess = {
            title: "Updated",
            description: "Successfully updated your account details",
        };
    }
</script>

<section class="w-full">
    {#if data.account != null}
        <form class="flex flex-col gap-2" on:submit={submitChanges}>
            <div class="">
                <p class="text-sm tracking-wider font-semibold">username</p>
                <p class="text-xs py-px h-fit">
                    {data.account.username}
                </p>
            </div>
            <div class="">
                <p class="text-sm tracking-wider font-semibold">display name</p>
                <TextSinglelineInput
                    bind:value={changeData.display_name}
                    placeholder={data.account.display_name == null
                        ? ""
                        : data.account.display_name}
                    extraClass="text-xs !py-px !pl-1"
                />
            </div>
            <div>
                <p class="text-sm tracking-wider font-semibold">bio</p>
                <TextMultilineInput
                    bind:value={changeData.bio}
                    placeholder={"Tell us a little bit about yourself"}
                    extraClass="text-xs !py-px !pl-1 !h-24"
                />
            </div>
            <div class="">
                <p class="text-sm tracking-wider font-semibold">pronouns</p>
                <TextSinglelineInput
                    bind:value={changeData.pronouns}
                    placeholder={data.account.pronouns}
                    extraClass="text-xs !py-px !pl-1"
                />
            </div>
            <div class="">
                <p class="text-sm tracking-wider font-semibold">location</p>
                <TextSinglelineInput
                    bind:value={changeData.location}
                    placeholder="Enter your location (do not put your address)"
                    extraClass="text-xs !py-px !pl-1"
                />
            </div>
            <button
                type="submit"
                class="w-full bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] py-2 mt-2 transition-all duration-200"
            >
                <p class="text-base font-semibold">SAVE CHANGES</p>
            </button>
            {#if submitError != null && submitSuccess == null}
                <ErrorBox
                    error={submitError.error}
                    description={submitError.description}
                />
            {:else if submitSuccess != null}
                <SuccessBox
                    title={submitSuccess.title}
                    description={submitSuccess.description}
                />
            {/if}
        </form>
    {:else}
        <p class="text-sm tracking-wider">Failed to load account data</p>
    {/if}
</section>
