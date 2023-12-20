<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import Test from "./components/Test.vue";
</script>

<template>
  <div class="container">
    <Test/>
    <h1>Welcome to Tauri+Photon!</h1>
    <p>The following image formats are supported:
PNG
JPEG
BMP
ICO
TIFF</p>
    <Greet />
  </div>
</template>

<style scoped>

</style>
