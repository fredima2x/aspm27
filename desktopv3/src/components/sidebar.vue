<script setup>
import { ref, onMounted } from "vue";
import Chat from "../components/chat.vue";
import get_chats from "../api/requests/get_chats.js";
import { get_cache, set_cache } from "../stores/cache.js";

const chats = ref([]);
const selectedChatId = ref('');

const props = defineProps({
  current_user_id: String,
});

const emit = defineEmits([
  "select_chat"
]);

onMounted(async () => {
  chats.value = get_cache("chats");
  console.debug("Getting Chats FROM cache", chats.value)
  setInterval(async () => { await updateChat() }, 5000)
});

async function updateChat() {
  try {
    const payload = await get_chats();
    console.debug("Fetched Chats from Server", payload);
    const chats_ = Array.isArray(payload.body)
        ? payload.body.map((chat) => ({
            chatId: chat.id,
            chatName: chat.chat_name,
            chatDesc: chat.chat_desc,
          }))
        : [];
    chats.value = chats_;
    set_cache("chats", chats_);
  } catch {
    console.error("Failed to Fetch Chats from API");
    chats.value = get_cache("chats");
  }
}

function chooseChat(chatId) {
  selectedChatId.value = chatId;
  emit("select_chat", chatId)
}

</script>

<template>
  <div class="sidebar">
    <div class="profile-panel widget"></div>
    <div class="chat-panel widget">
      <div class="search-bar">
        <input type="search" placeholder="Search Chats..." />
      </div>
      <div class="chat-list">
        <Chat
          v-for="chat in chats"
          :key="chat.chatId"
          :chatName="chat.chatName"
          :chatDesc="chat.chatDesc"
          :chatId="chat.chatId"
          :selected="chat.chatId === selectedChatId"
          @select="chooseChat"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
@import "../assets/styles/general.css";
@import "../assets/styles/root.css";

.sidebar {
  display: flex;
  flex-direction: column;
  flex: 1;
  margin: 0;
  height: 100%;
}

.widget {
  display: flex;
  flex-direction: column;
  margin: 10px;
  padding: 10px;
  background-color: var(--theme-gray);
  border-radius: 10px;
}

.profile-panel {
  flex: 0 0 100px;
  margin-right: 0;
  margin-bottom: 0;
}

.chat-panel {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
  margin-right: 0;
}

.chat-list {
  flex: 1;
  min-height: 0;
  max-height: 100%;
  overflow-y: auto;
  background-color: var(--theme-gray);
  border-radius: var(--theme-round-edges);
}

.search-bar {
  border-bottom: 3px solid var(--theme-primary);
}

.search-bar input {
  border: 1px solid var(--theme-light-gray);
}
</style>
