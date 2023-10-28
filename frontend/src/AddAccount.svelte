<script>
  import { UpsertAccount } from "../wailsjs/go/main/App";
  import { writable } from "svelte/store";
  
  export let activePage = "";
  const user = writable({
    id: 0,
    username: "",
    issuer: "",
    secret: "",
    type: "TOTP",
    hash_function: "SHA1",
    period: 30,
    digits: 6,
    note: "",
  })

  function saveAccount() {
    UpsertAccount(JSON.stringify($user))
      .then((response) => {
        if (response === "OK") {
          activePage = "list";
        } else {
          alert(response);
        }
      })
  }

</script>

<div class="title">
  <div class="title-left">
    <p>Add Account</p>
  </div>
  <div class="title-right">
    <button class="btn-nav btn-green"
      on:click={() => {
        saveAccount();
      }}>Save</button
    >
    <button class="btn-nav btn-red"
      on:click={() => {
        activePage = "list";
      }}>Close</button
    >
  </div>
</div>
<div class="add-form">
  <div class="field">
    <label for="username">Username</label>
    <input type="text" name="username" bind:value={$user.username}>
  </div>
  <div class="field">
    <label for="issuer">Account Issuer</label>
    <input type="text" name="issuer" bind:value={$user.issuer}>
  </div>
  <div class="field">
    <label for="secret">Secret</label>
    <input type="text" name="secret" bind:value={$user.secret}>
  </div>
  <div class="field">
    <label for="type">Type</label>
    <select name="type" bind:value={$user.type}>
      <option value="TOTP">TOTP</option>
    </select>
  </div>
  <div class="field">
    <label for="hash">Hash Function</label>
    <select name="hash" bind:value={$user.hash_function}>
      <option value="SHA1">SHA1</option>
    </select>
  </div>
  <div class="field">
    <label for="period">Period (seconds)</label>
    <input type="number" name="period" bind:value={$user.period}>
  </div>
  <div class="field">
    <label for="digit">Digit</label>
    <input type="number" name="digit" bind:value={$user.digits}>
  </div>
  <div class="field">
    <label for="note">Note</label>
    <input type="text" name="note" bind:value={$user.note}>
  </div>
</div>

<style>
  .add-form {
    height: 550px;
    overflow-y: hidden;
    padding: 15px;
  }
  .field {
    width: 100%;
    display: flex;
    flex-direction: column;
  }
</style>
