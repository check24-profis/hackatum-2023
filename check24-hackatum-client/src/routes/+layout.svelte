<script>
    import "../app.css";
    import Switch from '@smui/switch';
    import { admin_mode } from '$lib/stores.js';
</script>

<!-- Navbar -->
<nav class="bg-24-blue text-white p-3 m-3 lg:p-4 lg:m-4 rounded-full grid grid-cols-3 place-items-center">
    <a href="/" class="col-[2]">
        <img src="/img/craft24_logo.svg"
            alt="Craft24 Logo"
            class="h-6">
    </a>
    <div class="w-full h-full flex flex-row justify-end items-center">
        <span class="text-gray-400 hidden sm:inline-block">Admin</span><Switch bind:checked={$admin_mode} icons={false} class="col-[3]"/>
    </div>
</nav>

<!-- Content -->
<main class="py-6 px-6 lg:px-[15rem] flex-1">
    <slot></slot>
</main>

<footer class="bg-gray-300 p-4 text-xs mt-auto">
    © 2023 CRAFT24 Vergleichsportal Profis GmbH
</footer>