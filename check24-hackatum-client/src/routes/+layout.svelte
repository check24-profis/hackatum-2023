<script>
    import "../app.css";
</script>

<!-- Navbar -->
<nav class="bg-24-blue text-white p-3 m-3 lg:p-4 lg:m-4 rounded-full flex justify-center">
    <a href="/">
        <img src="/img/craft24_logo.svg"
            alt="Craft24 Logo"
            class="h-6">
    </a>
</nav>

<!-- Content -->
<main class="py-6 px-6 lg:px-[15rem] flex-1">
    <slot></slot>
</main>

<footer class="bg-gray-300 p-4 text-xs mt-auto">
    © 2023 CRAFT24 Vergleichsportal Profis GmbH
</footer>