<script>
    import Textfield from '@smui/textfield';

    let query = "";
</script>

<h1 class="text-2xl"><b>Handwerker</b> - Profis in Ihrer Nähe</h1>
<span class="text-[#008300]">Kostenlos Angebote innerhalb weniger Stunden erhalten.</span>

<div class="flex flex-row items-center gap-2">
    <span class="material-symbols-rounded text-24-blue">done</span> Professionelle Handwerkerleistungen für Ihr Projekt
</div>
<div class="flex flex-row items-center gap-2">
    <span class="material-symbols-rounded text-24-blue">done</span> Maler, Elektriker, Fliesenleger & andere Handwerker
</div>
<div class="flex flex-row items-center gap-2">
    <span class="material-symbols-rounded text-24-blue">done</span> Bequem und sicher online zahlen
</div>

<span class="block mt-3 mb-2">In welcher PLZ suchen Sie einen Handwerker?</span>
<Textfield
    variant="filled"
    label="Ihre PLZ"
    class="w-full"
    bind:value={query} 
>
    <span class="material-symbols-rounded ml-[16px] mr-[8px] text-xl" slot="leadingIcon">home_pin</span>
</Textfield>
<!-- Button to redirect to new page -->