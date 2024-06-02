<script>
    import TextSinglelineInput from "../../../components/TextSinglelineInput.svelte";
    import Title from "../../../components/Title.svelte";
</script>

<div class="flex flex-col justify-center items-center w-full h-screen">
    <Title textClass="text-4xl" />
    <div class="w-80 h-fit shadow-one mt-4 px-4 py-3">
        <h1 class="text-3xl font-bold">Login</h1>
        <TextSinglelineInput
            placeholder="Email or username"
            extraClass="mt-4"
        />
        <TextSinglelineInput
            placeholder="Password"
            type="password"
            extraClass="mt-6"
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
        >
            <p class="text-base font-semibold">LOGIN</p>
        </button>
        <div class="flex flex-row w-full justify-center items-center mt-2">
            <hr class="border-b-1 w-full border-black"/>
            <p class="mx-4 text-sm pb-[2px]">or</p>
            <hr class="border-b-1 w-full border-black"/>
        </div>
        <button
            class="w-full bg-[var(--yellow)] hover:bg-[var(--dark-yellow)] py-2 mt-2 transition-all duration-200"
        >
            <p class="text-base font-semibold">SIGN UP</p>
        </button>
    </div>
</div>
