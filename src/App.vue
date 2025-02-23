<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="flex flex-col justify-center items-center w-full bg-gray-700 h-screen space-y-4">
    <h1 class="text-4xl text-white underline">Welcome to Tauri + Vue</h1>

    <div class="flex h-16">
      <a href="https://vitejs.dev" target="_blank" class="h-full">
        <img src="/vite.svg" class="h-full" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank" class="h-full">
        <img src="/tauri.svg" class="h-full" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank" class="h-full">
        <img src="./assets/vue.svg" class="h-full " alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit" class="">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </main>
</template>

<style>
@import "tailwindcss";

</style>