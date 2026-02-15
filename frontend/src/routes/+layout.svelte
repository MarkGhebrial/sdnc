
<script lang="ts">  
    import { website_title } from '$lib/config';

    import _ from '$lib/style.scss';

    let navbar_items = [
        { name: "Test", url: "/test" },
        { name: "Rules", url: "TODO" },
    ]

    let { children } = $props();
</script>

<svelte:head>
    <title>{ website_title }</title>
</svelte:head>

<header>
    <!-- Navbar -->
    <nav class="navbar navbar-expand-sm bg-body-tertiary">
        <div class="container-fluid">
        <a class="navbar-brand" href="/">San Diego Nerf Club</a>
        
        <a class="btn btn-primary" href="/join">Join</a>

        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
        </button>

        <!-- Collapsible navbar section -->
        <div class="collapse navbar-collapse" id="navbarSupportedContent">
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            <!-- {# Fill in all the nav links specified in config.toml #} -->

            {#each navbar_items as navbar_item }
                <li class="nav-item">
                <a class="nav-link active" aria-current="page" href="{ navbar_item.url }">{ navbar_item.name }</a>
                </li>
            {/each}
            </ul>
        </div>
        </div>
    </nav>
</header>

<div id="content">
    {@render children()}
</div>
