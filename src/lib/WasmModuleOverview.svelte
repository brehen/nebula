<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let funcResult = "";
  let files: string[] = [];
  let runHistory: {
    file: string;
    result: string;
  }[] = [];

  async function run_module(file: string) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    funcResult = await invoke("run_module", { modulePath: `./${file}` });
    runHistory = [...runHistory, { file, result: funcResult }];
  }

  async function get_wats() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    files = await invoke("get_wats");
  }

  onMount(async () => {
    await get_wats();
  });
</script>

<div>
  <p>{JSON.stringify(funcResult)}</p>
  <button on:click={get_wats}>GG</button>
  <ol>
    {#each files as file}
      <li>
        <button on:click={() => run_module(file)}>{file}</button>
      </li>
    {/each}
  </ol>
  <ol>
    {#each runHistory as entry}
      <li><span>{entry.result}</span></li>
    {/each}
  </ol>
</div>
