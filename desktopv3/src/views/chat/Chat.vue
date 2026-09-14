<script setup>
import sidebar from "../../components/sidebar.vue";
import contentPanel from "../../components/contentPanel.vue";
import { ref } from "vue";

const selected_chat_id = ref("")

function handle_select_chat(chatId) {
  selected_chat_id.value = chatId;
}

</script>

<template>
  <div class="app-background">
    <sidebar @select_chat="handle_select_chat"/>
    <contentPanel :selected_chat_id="selected_chat_id"/>
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
