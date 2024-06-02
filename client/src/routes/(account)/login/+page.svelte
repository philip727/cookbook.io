<script>
    import Title from "../../../components/Title.svelte";
</script>

<div class="flex flex-col justify-center items-center w-full h-screen">
    <Title textClass="text-4xl" />
    <div class="w-80 h-96 shadow-one mt-4 p-4">
        <h1 class="text-3xl font-bold">Sign in</h1>
        <input placeholder="Email or username" type="text" />
        <input placeholder="Password" type="password" />
    </div>
</div>
