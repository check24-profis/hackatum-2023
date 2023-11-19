<script>
    export let data;

    import Button from '@smui/button';
    import Textfield from '@smui/textfield';
    import { update_craftsman } from '$lib/api_utils.js';
    import { admin_mode } from '$lib/stores.js';

    let maxDrivingDistance = null;
    let profilePictureScore = null;
    let profileDescriptionScore = null;
</script>

<form on:submit|preventDefault={() => {
    if(typeof window !== 'undefined') {
        update_craftsman(window.location.origin.split(":").slice(0, 2).join(':'), data.id, maxDrivingDistance, profilePictureScore, profileDescriptionScore)
    }
}} class="flex flex-col gap-1">
    <Textfield
        variant="filled"
        label="Maximale Fahrdistanz"
        class="w-full"
        input$type=number 
        input$step=0.01
        input$disabled={!$admin_mode}
        bind:value={maxDrivingDistance} 
    >
        <span class="material-symbols-rounded ml-[16px] mr-[8px] text-xl" slot="leadingIcon">directions_car</span>
    </Textfield>
    <Textfield
        variant="filled"
        label="Profilbild Wertung"
        class="w-full mt-2"
        input$type=number 
        input$step=0.01
        input$disabled={!$admin_mode}
        bind:value={profilePictureScore} 
    >
        <span class="material-symbols-rounded ml-[16px] mr-[8px] text-xl" slot="leadingIcon">account_circle</span>
    </Textfield>
    <Textfield
        variant="filled"
        label="Profilbeschreibung Wertung"
        class="w-full mt-2"
        input$type=number 
        input$step=0.01
        input$disabled={!$admin_mode}
        bind:value={profileDescriptionScore} 
    >
        <span class="material-symbols-rounded ml-[16px] mr-[8px] text-xl" slot="leadingIcon">badge</span>
    </Textfield>
    <Button 
        touch 
        variant="raised" 
        class="bg-24-blue mt-3 p-5 rounded-full overflow-hidden"
        disabled={!$admin_mode}
    >
        Aktualisieren
    </Button>
</form>