<script setup>
import { ref, onMounted } from "vue";
import sidebarChat from "./sidebarChat.vue";
import { Chat } from "../api/requests/modelChat.js";
import get_chats from "../api/requests/get_chats.js";
import { get_cache, set_cache } from "../stores/cache.js";
import { deleteChat } from "../api/requests/deleteChat.js";
import { generateUuid } from "../api/client.js";

const chat1 = new Chat(generateUuid(), "test", "test chat");
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
    chats.value = Array.isArray(payload.body)
      ? payload.map((chat) => ({
          chatId: chat.id,
          chatName: chat.chat_name,
          chatDesc: chat.chat_desc,
        }))
      : [];

    localStorage.setItem("chats", JSON.stringify(chats.value));
  } catch (error) {
    console.error("Failed to load chats", error);

    const cached = JSON.parse(localStorage.getItem("chats") || "[]");
    chats.value = Array.isArray(cached) ? cached : [];
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
        <sidebarChat
          v-for="chat in chats"
          :key="chat.chatId"
          :chatName="chat.chatName"
          :chatDesc="chat.chatDesc"
          :chatId="chat.chatId"
          :selected="chat.chatId === selectedChatId"
          @select="chooseChat(chat.chatId)"
          @contextmenu.prevent="deleteChat(chat, chats)"
        />
        <div class="add-chat-button-wrapper">
          <div
            class="add-chat-button"
            @click="chats.unshift(new Chat(generateUuid(), 'test', 'test chat'))">
            <p>+</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@import "../assets/styles/general.css";
@import "../assets/styles/root.css";
@import "../assets/styles/utils/animations.css";

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
  padding-top: 10px;
  background-color: var(--theme-gray);
  border-radius: var(--theme-round-edges);
  display: flex;
  flex-direction: column;
  align-items: stretch;
}

.search-bar {
  border-bottom: 3px solid var(--theme-primary);
}

.search-bar input {
  border: 1px solid var(--theme-light-gray);
}

.add-chat-button-wrapper {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding-bottom: 10px;
}

.add-chat-button {
  border-radius: 50%;
  width: 4em;
  height: 4em;
  background-color: var(--theme-light-gray);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  animation: fadeInTop 0.5s ease-in-out;
}

.add-chat-button p {
  font-size: 4em;
}
</style>
