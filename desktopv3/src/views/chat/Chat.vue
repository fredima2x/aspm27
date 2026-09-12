<script setup>
import sidebar from "../../components/sidebar.vue";
import contentPanel from "../../components/contentPanel.vue";
import Settings from "../settings/Settings.vue";
</script>

<template>
  <div class="app-background">
    <sidebar />
    <contentPanel />
  </div>
</template>

<style scoped>
* {
  box-sizing: border-box;
}

.app-background {
  display: flex;
  width: 100vw;
  height: 100vh;
  margin: 0;
  padding: 0;
  overflow: hidden;
}
</style>
