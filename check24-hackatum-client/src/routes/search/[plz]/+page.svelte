<!-- FILEPATH: /home/dominik/Documents/Studium/HackaTUM2023/hackatum-2023/check24-hackatum-client/src/routes/search/+page.svelte -->

<script>
  // Dummy Craftsmen data
  export let data;

  let craftsmen = [
    {
      name: 'Malerbetrieb Maier',
      stars: 5,
      amount_ratings: 42,
      number_rating: 6.3,
      is_new: false,
      distance: 2.5,
      orders: 10,
      first_visit_free: false,
      happiness_garantee: true,
    },
    {
      name: 'Jörg Petersson Malerbetrieb',
      stars: 3,
      amount_ratings: 12,
      number_rating: 6.2,
      is_new: true,
      distance: 7.3,
      orders: 0,
      first_visit_free: true,
      happiness_garantee: false,
    },
  ];

  function load_more_craftsmen() {
    // for button
  }

  function number_rating_to_text(number_rating) {
    switch (number_rating) {
      case 7:
        return 'hervorragend';
      case 6:
        return 'sehr gut';
      case 5:
        return 'gut';
      case 4:
        return 'befriedigend';
      case 3:
        return 'ausreichend';
      case 2:
        return 'mangelhaft';
      case 1:
        return 'ungenügend';
      default:
        return 'neu';
    }
  }
</script>

<main>
  <h1><b>Handwerker</b> - Profis in Ihrer Nähe {data.plz}</h1>
  
  <header style="display: flex; justify-content: space-between;">
      <div style="display: flex;">
      <button>Anfrage stornieren</button>
      <button>Details anzeigen</button>
    </div>
  </header>

  <header>
    <h1><b>Zu Ihrer Anfrage passende Profis</b></h1>
    <p>{craftsmen.length} Ergebnisse</p>
  </header>

  {#each craftsmen as craftsman}
    <div>
      <img src="round-picture.jpg" alt="Craftsman Picture">
      <h2><b>{craftsman.name}</b></h2>

      <div>
        {#if craftsman.is_new}
          <p>neu</p>
          <p>bei CHECK24</p>
        {:else}
          <div>
            <div class="star-ratings">
              <div class="fill-ratings" style="width: 50%;">
                <span>★★★★★</span>
              </div>
              <div class="empty-ratings">
                <span>★★★★★</span>
              </div>
            </div>
          </div>
          <p>{craftsman.amount_ratings}</p>
          <p>{craftsman.number_rating}</p>
          <p>{number_rating_to_text(craftsmen.number_rating)}</p>
          {#if craftsman.happiness_garantee}
            <p>HAPPINESS GARANTIE</p>
          {/if}
        {/if}
      </div>

      <span class="material-symbols-rounded"> location_on </span>

      <p>{craftsman.distance} km entfernt</p>

      <div>
      <span class="material-symbols-rounded">
        license
      </span>
      {#if craftsman.is_new}
        <p>kürzlich gebucht</p>
      {:else}
        <p>{craftsman.orders} Buchungen</p>
      {/if}
      </div>
       
      <div>
      {#if craftsman.first_visit_free}
        <p>Kostenlose Besichtigung</p>
      {/if}
      </div>

    </div>
  {/each}

  <button on:click={load_more_craftsmen}>mehr anzeigen</button>
</main>

