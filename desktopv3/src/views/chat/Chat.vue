<script setup>
import sidebar from "../../components/sidebar.vue";
import contentPanel from "../../components/contentPanel.vue";
import { onMounted, ref } from "vue";
import get_profile from "../../api/requests/get_profile.js";
import { get_cache, set_cache } from "../../stores/cache.js";

const selected_chat_id = ref("")
const current_user = ref("")

function handle_select_chat(chatId) {
  selected_chat_id.value = chatId ? chatId : get_cache('lastSelectedChat');
  set_cache("lastSelectedChat", selected_chat_id.value);
}

async function get_current_user() {
  const res = await get_profile()
  if (!res.raw.ok) { return };
  current_user.value = res.body.user;
}

onMounted(async () => {
  selected_chat_id.value = get_cache('lastSelectedChat') ? get_cache('lastSelectedChat') : '';
  await get_current_user();
})

</script>

<template>
  <div class="app-background">
    <sidebar @select_chat="handle_select_chat"/>
    <contentPanel :current_user="current_user" :selected_chat_id="selected_chat_id"/>
  </div>
</template>

<style scoped>
* {
  box-sizing: border-box;
}

.app-background {
  display: grid;
  grid-template-columns: clamp(200px, 35vw, 350px) minmax(0, 1fr);
  grid-template-rows: minmax(0, 1fr);
  width: 100vw;
  height: 100vh;
  margin: 0;
  padding: 0;
  overflow: hidden;
}

@media (max-width: 500px) {
  .app-background {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
