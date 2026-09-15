<script setup>
import sidebar from "../../components/sidebar.vue";
import contentPanel from "../../components/contentPanel.vue";
import { onMounted, ref } from "vue";
import get_profile from "../../api/requests/get_profile.js";

const selected_chat_id = ref("")
const current_user = ref("")

function handle_select_chat(chatId) {
  selected_chat_id.value = chatId;
}

async function get_current_user() {
  const res = await get_profile()
  if (!res.raw.ok) { return };
  current_user.value = res.body.user;
}

onMounted(async () => {
  await get_current_user();
})

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
