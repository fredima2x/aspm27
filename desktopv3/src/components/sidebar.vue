<script setup>
import { ref } from "vue";
import chat from "../components/chat.vue";
import get_chats from '../api/requests/get_chats.js';

const chats = get_chats();

const selectedChatId = ref(67);

function chooseChat(chatId) {
  selectedChatId.value = chatId;
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
        <chat
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
