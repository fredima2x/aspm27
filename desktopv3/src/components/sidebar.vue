<script setup>
import { ref, onMounted } from "vue";
import Chat from "../components/chat.vue";
import get_chats from "../api/requests/get_chats.js";
import { deleteChat, removeChatFromLocalStorage } from "../api/requests/deleteChat.js";
import { generateUuid } from "../api/client.js";

const chats = ref([]);
const selectedChatId = ref(null);

onMounted(async () => {
  try {
    const payload = await get_chats();
    chats.value = Array.isArray(payload)
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
});

function chooseChat(chatId) {
  selectedChatId.value = chatId;
}

async function testRemoveChat(chatId = chats.value[0]?.chatId) {
  if (!chatId) {
    console.warn("testRemoveChat: no chat is available to remove");
    return;
  }

  try {
    console.log(`testRemoveChat: attempting to remove ${chatId}`);

    await deleteChat(chatId);
    removeChatFromLocalStorage(chatId);

    chats.value = chats.value.filter((chat) => chat.chatId !== chatId);
    localStorage.setItem("chats", JSON.stringify(chats.value));

    if (selectedChatId.value === chatId) {
      selectedChatId.value = null;
    }

    console.log(`testRemoveChat: removed ${chatId}; remaining=${chats.value.length}`);
  } catch (error) {
    console.error("testRemoveChat: failed to remove chat", error);
  }
}

testRemoveChat('')
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
          @select="testRemoveChat(chatId)"
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
