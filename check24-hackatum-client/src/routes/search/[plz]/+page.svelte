<script>
    import Button from '@smui/button';
    import CircularProgress from '@smui/circular-progress';
    import { Label } from '@smui/common';

    export let data;
    let loading = false;

    // Dummy Craftsmen data
    let craftsmen = [
        {
            name: "Bouncy Bob",
            rankingScore: 8
        },
        {
            name: "Bobby Bounce",
            rankingScore: 7
        },
        {
            name: "Peter Lustig",
            rankingScore: 6
        },
        {
            name: "Peter Lustig",
            rankingScore: 5.5
        },
        {
            name: "Peter Lustig",
            rankingScore: 4.4
        },
        {
            name: "Peter Lustig",
            rankingScore: 3.3
        },
        {
            name: "Peter Lustig",
            rankingScore: 2
        },
        {
            name: "Ladadi Ladadei",
            rankingScore: 0
        },
    ];

    function load_more_craftsmen() {
        // for button
    }
</script>

<svelte:head>
    <style>
        body {
            background: transparent;
        }
        body:before {
            content: "";
            position: fixed;
            width: 100%;
            height: 100%;
            background-image: url('/img/bg.png?enhanced');
            background-size: cover;
            z-index: -1;
            filter: blur(10px);
            transform: scale(1.1);
        }
    </style>
</svelte:head>

<h1 
    class="text-2xl text-white font-bold" 
    style="filter: drop-shadow(0px 0px 12px #063773)">Zu Ihrer Postleitzahl ({data.plz}) passende Profis
</h1>
<!-- TODO: show number of *actual* results -->
{#if !loading}
    <span 
        class="text-base text-white mb-3 block drop-shadow-md" 
        style="filter: drop-shadow(0px 0px 10px #063773)">{craftsmen.length} Ergebnis{craftsmen.length > 1 ? "se" : ""}
    </span>
{/if}

<div class="flex flex-col gap-2">
    {#each craftsmen as craftsman}
        <div class="flex flex-col gap-1 bg-white rounded-md p-2">
            <div class="flex flex-col md:flex-row gap-1 sm:max-md:justify-center md:items-center">
                <div class="inline-flex flex-row gap-1 items-center">
                    <span class="py-1 px-2 bg-24-blue text-white rounded-md mr-1">{craftsman.rankingScore.toFixed(1)}</span>
                    
                    <span>{craftsman.name}</span>
                </div>
        
                <div class="inline-grid w-min" style="--rating: {craftsman.rankingScore / 0.08}%;">
                    <div class="text-gray-400 col-[1] row-[1] w-full text-lg">
                        <span>★★★★★</span>
                    </div>
                    <div class="text-24-yellow col-[1] row-[1] overflow-hidden w-[--rating] text-lg">
                        <span>★★★★★</span>
                    </div>
                </div>
            </div>
            <!--<div class="flex flex-row items-center gap-1">
                <span class="material-symbols-rounded text-lg">location_on</span>
                <span class="text-sm">{craftsman.distance} km entfernt</span>
            </div>-->
        </div>
    {:else}
        {#if loading}
        <div class="flex flex-row justify-center mt-5">
            <CircularProgress style="height: 64px; width: 64px;" indeterminate />
        </div>    
        {:else}
            <span class="bg-white rounded-md p-2 w-max">Es wurden leider keine Profis für die PLZ "{data.plz}" gefunden.</span>
        {/if}
    {/each}
</div>
<!-- TODO: Only show if more pages available -->
{#if craftsmen.length > 0}
    <div class="flex justify-center">
        <Button touch variant="raised" class="bg-24-blue mt-3 p-5 rounded-full overflow-hidden" on:click={load_more_craftsmen}>
            <Label class="text-sm">Mehr anzeigen</Label>
        </Button>
    </div>
{/if}