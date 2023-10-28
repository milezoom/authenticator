<script>
  import { onMount } from "svelte";
  import { GetAccounts } from "../wailsjs/go/main/App";
  import { LogDebug, LogError } from "../wailsjs/runtime";
  export let activePage = "";
  $: accounts = [];

  function getAccounts() {
    GetAccounts()
      .then((response) => {
        accounts = [];
        let data = [];
        try {
          data = JSON.parse(response);
        } catch (error) {
          LogError("error when parsing accounts: " + error);
        }
        if (data.length > 0) {
          data.forEach((element) => {
            let acc = {
              issuer: element.issuer,
              username: element.username,
              code: element.code,
            };
            accounts = [...accounts, acc];
          });
        }
      })
      .catch((err) => {
        LogError("error when getting accounts: " + err);
      });
  }
  getAccounts();

  onMount(() => {
    let firstTimeout = (30 - (Math.floor(Date.now() / 1000) % 30)) * 1000;
    setTimeout(() => {
      getAccounts();
      setInterval(() => {
        getAccounts();
      }, 30000);
    }, firstTimeout);
  });
</script>

<div class="title">
  <div class="title-left">
    <p id="app-title">Authenticator</p>
  </div>
  <div class="title-right">
    <button
      class="btn-nav btn-green"
      on:click={() => {
        activePage = "add";
      }}>Add</button
    >
  </div>
</div>
<div class="scrollable">
  {#each accounts as acc}
    <div style="border-bottom: 1px solid #ffffff;padding: 5px 0;">
      <h4>{acc.issuer}</h4>
      <p>{acc.username}</p>
      <p style="font-size: 2.15em;">{acc.code}</p>
    </div>
  {/each}
</div>

<style>
  .scrollable {
    height: 550px;
    overflow-y: auto;
    padding: 0 15px 0 15px;
  }
</style>
