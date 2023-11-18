<script>
    import Textfield from '@smui/textfield';
    import Button from '@smui/button';
    import { Label } from '@smui/common';
    import { goto } from '$app/navigation';

    let plz = "";
</script>

<svelte:head>
    <link rel="preload" as="image" href="/img/bg.png?enhanced" />
</svelte:head>

<h1 class="text-2xl"><b>Handwerker</b> - Profis in Ihrer Nähe</h1>
<span class="text-[#008300]">Kostenlos Angebote innerhalb weniger Sekunden erhalten.</span>

<div class="flex flex-row items-center gap-2">
    <span class="material-symbols-rounded text-24-blue">done</span> Professionelle Handwerkerleistungen für Ihr Projekt
</div>
<div class="flex flex-row items-center gap-2">
    <span class="material-symbols-rounded text-24-blue">done</span> Maler, Elektriker, Fliesenleger & andere Handwerker
</div>
<div class="flex flex-row items-center gap-2">
    <span class="material-symbols-rounded text-24-blue">done</span> Bequem und sicher online zahlen
</div>

<form on:submit|preventDefault={() => {goto('/search/' + plz)}}>
    <span class="block mt-3 mb-2 font-bold">In welcher PLZ suchen Sie einen Handwerker?</span>
    <Textfield
        variant="filled"
        label="Ihre PLZ"
        class="w-full"
        required
        input$maxlength="10"
        bind:value={plz} 
    >
        <span class="material-symbols-rounded ml-[16px] mr-[8px] text-xl" slot="leadingIcon">home_pin</span>
    </Textfield>

    <div class="w-full flex justify-center">
        <Button touch variant="raised" class="mt-2 p-5 w-40 rounded-full overflow-hidden">
            <span class="material-symbols-rounded mr-1 text-xl">search</span>
            <Label>Suchen</Label>
        </Button>
    </div>
</form>