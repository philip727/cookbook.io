<script lang="ts">
    import { endpoint } from "$lib/api";
    import { JWT_TOKEN_KEY, user } from "$lib/login";
    import { HttpStatusCode } from "axios";
    import type { ResponseError } from "../../../components/ErrorBox";
    import ErrorBox from "../../../components/ErrorBox.svelte";
    import type { Success } from "../../../components/SuccessBox.svelte";
    import SuccessBox from "../../../components/SuccessBox.svelte";
    import TextMultilineInput from "../../../components/TextMultilineInput.svelte";
    import TextSinglelineInput from "../../../components/TextSinglelineInput.svelte";
    import type { PageData } from "./$types";
    import { goto } from "$app/navigation";
    import type { PublicUserProfileDetails } from "$lib/profile";

    let currentUser: PublicUserProfileDetails | null;
    user.subscribe((value) => {
        currentUser = value;
    });

    // Retrieved account data
    export let data: PageData;

    // The data we submit/change
    let submitError: ResponseError | null = null;
    let submitSuccess: Success | null = null;
    let changeData = {
        bio: data.account?.bio,
        pronouns: data.account?.pronouns,
        location: data.account?.location,
    };

    async function updateAccountDetails(
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

    const uploadProfilePicture = async (formData: FormData) => {
        let key = window.localStorage[JWT_TOKEN_KEY];
        if (key == null) {
            return;
        }

        let bearer = "Bearer " + key;
        const response = await window.fetch(endpoint("/account/upload_pfp"), {
            method: "POST",
            body: formData,
            headers: {
                Authorization: bearer,
            },
        });

        if (!response.ok) {
            if (response.status == HttpStatusCode.Unauthorized) {
                window.localStorage[JWT_TOKEN_KEY] = null;
                goto("/");
            }

            return;
        }

        // Updates all the current pictures with the new one given if updated already
        let data = await response.text();
        if (currentUser != null) {
            user.set({ ...currentUser, picture: data });
        }
    };

    let files: FileList;
    // Submit as soon as it changes
    $: if (files && files.item(0)) {
        // Uploads the profile picture
        const formData = new FormData();
        formData.append("picture", files.item(0) as File);
        uploadProfilePicture(formData);
    }
</script>

<section class="w-full">
    {#if data.account != null}
        <form class="flex flex-col gap-2" on:submit={updateAccountDetails}>
            <div class="">
                <p class="text-sm tracking-wider font-semibold">username</p>
                <p class="text-xs py-px h-fit">
                    {data.account.username}
                </p>
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
                    placeholder={data.account.pronouns == null
                        ? ""
                        : data.account.pronouns}
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
            <div>
                <p class="text-sm tracking-wider font-semibold">profile picture</p>
                <div class="h-fit flex flex-row mt-1">
                    {#if currentUser?.picture != null}
                        <img
                            class="h-16 w-16 object-cover"
                            src={endpoint(`/pfp/${currentUser?.picture}`)}
                            alt="User profile avatar"
                        />
                    {:else}
                        <img
                            class="h-16 w-16 object-cover"
                            src={`https://api.dicebear.com/8.x/avataaars-neutral/svg?seed=${currentUser?.username}`}
                            alt="User profile avatar"
                        />
                    {/if}
                    <label for="pfp-upload">
                        <div
                            class="cursor-pointer h-16 w-32 bg-[var(--yellow)] ml-4 hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center"
                        >
                            <p class="text-xl font-bold tracking-widest">UPLOAD</p>
                        </div>
                    </label>
                    <input
                        id="pfp-upload"
                        class="h-16 w-16 hidden"
                        type="file"
                        accept="image/png, image/jpeg"
                        bind:files
                    />
                </div>
            </div>
            <!-- 
            {#if submittableDataIsUnchanged()}
                <div
                    class="w-full bg-gray-400 hover:bg-gray-500 py-2 mt-2 transition-all duration-200 flex flex-row items-center justify-center cursor-pointer"
                >
                    <p class="text-base font-semibold">SAVE CHANGES</p>
                </div>
            {:else}
                -->
            <button
                type="submit"
                class="w-full bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] py-2 mt-2 transition-all duration-200"
            >
                <p class="text-base font-semibold">SAVE CHANGES</p>
            </button>
            <!-- 
            {/if}
            -->
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
