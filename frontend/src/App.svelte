<script>
  import ListAccount from "./ListAccount.svelte";
  import AddAccount from "./AddAccount.svelte";
  
  let activePage;
</script>

<main>
  {#if activePage === "add"}
    <AddAccount bind:activePage={activePage} />
  {:else}
    <ListAccount bind:activePage={activePage} />
  {/if}
</main>

<style></style>
