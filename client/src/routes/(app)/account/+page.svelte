<script lang="ts">
    import { endpoint } from "$lib/api";
    import { JWT_TOKEN_KEY, getBearer, signedInUser, user } from "$lib/login";
    import { HttpStatusCode } from "axios";
    import type { ResponseError } from "../../../components/ErrorBox";
    import ErrorBox from "../../../components/ErrorBox.svelte";
    import type { Success } from "../../../components/SuccessBox.svelte";
    import SuccessBox from "../../../components/SuccessBox.svelte";
    import TextMultilineInput from "../../../components/TextMultilineInput.svelte";
    import TextSinglelineInput from "../../../components/TextSinglelineInput.svelte";
    import type { PageData } from "./$types";
    import { goto } from "$app/navigation";
    import uploadArrow from "$lib/images/upload-arrow.svg";
    import deleteBin from "$lib/images/recycle-bin.svg";

    // Retrieved account data
    export let data: PageData;
    let files: FileList;

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

        let bearer = getBearer();
        if (!bearer) {
            return;
        }

        // Need to null empty values for serialization on server side
        let json = {
            bio: changeData.bio == "" ? null : changeData.bio,
            pronouns: changeData.pronouns == "" ? null : changeData.pronouns,
            location: changeData.location == "" ? null : changeData.location,
        };

        const form = new FormData();
        form.append("details", JSON.stringify(json));

        // Picture doesn't have to be present
        if (files && files.item(0)) {
            form.append("picture", files.item(0) as File);
        }

        let response = await window.fetch(endpoint("/account/update_details"), {
            method: "POST",
            headers: {
                Authorization: bearer,
            },
            body: form,
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

    const deleteProfilePicture = async () => {
        let bearer = getBearer();
        if (!bearer) {
            return;
        }

        const response = await window.fetch(endpoint("/account/delete_pfp"), {
            method: "GET",
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
        if (signedInUser != null) {
            user.set({ ...signedInUser, picture: null });
        }
    };
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
                <p class="text-sm tracking-wider font-semibold">
                    profile picture
                </p>
                <div class="h-fit flex flex-row mt-1">
                    {#if signedInUser?.picture != null}
                        <img
                            class="h-16 w-16 object-cover"
                            src={endpoint(`/pfp/${signedInUser?.picture}`)}
                            alt="User profile avatar"
                        />
                    {:else}
                        <img
                            class="h-16 w-16 object-cover"
                            src={`https://api.dicebear.com/8.x/shapes/svg?seed=${signedInUser?.username}`}
                            alt="User profile avatar"
                        />
                    {/if}
                    <label for="pfp-upload">
                        <div
                            class="cursor-pointer h-16 w-16 bg-[var(--yellow)] ml-4 hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center"
                        >
                            <img
                                class="h-8 w-8"
                                src={uploadArrow}
                                alt="Upload button"
                            />
                        </div>
                    </label>
                    <input
                        id="pfp-upload"
                        class="h-16 w-16 hidden"
                        type="file"
                        accept="image/png, image/jpeg"
                        bind:files
                    />
                    {#if signedInUser?.picture != null}
                        <button
                            class="cursor-pointer h-16 w-16 bg-[var(--yellow)] ml-4 hover:bg-[var(--dark-yellow)] duration-200 flex justify-center items-center"
                            on:click={deleteProfilePicture}
                            type="button"
                        >
                            <img
                                class="h-10 w-10"
                                src={deleteBin}
                                alt="Delete button"
                            />
                        </button>
                    {/if}
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
