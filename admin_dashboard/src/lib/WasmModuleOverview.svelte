<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  type FunctionResult = {
    function_name: string;
    start: number;
    end: number;
    total_runtime: number;
  };
  let funcResult: FunctionResult;
  let files: string[] = [];
  let runHistory: FunctionResult[] = [];

  async function run_module(file: string) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    funcResult = await invoke<FunctionResult>("run_module", {
      path: `./${file}`,
    });
    console.log(funcResult);
    runHistory = [...runHistory, funcResult];
    console.log({ runHistory });
    console.table(runHistory);
  }

  async function get_wats() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    files = await invoke("list_files_in_dir");
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
      <li><span>{JSON.stringify(entry)}</span></li>
    {/each}
  </ol>
</div>
