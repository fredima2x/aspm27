<script setup>
import { ref, onMounted } from "vue";
import sidebarChat from "./sidebarChat.vue";
import chatCreationDialog from "./chatCreationDialog.vue";
import { Chat } from "../api/requests/modelChat.js";
import get_chats from "../api/requests/get_chats.js";
import { get_cache, set_cache } from "../stores/cache.js";
import { delete_chat } from "../api/requests/delete_chat.js";
import { create_chat } from "../api/requests/create_chat.js";
import { update_chat } from "../api/requests/update_chat.js";
import { generateUuid } from "../api/client.js";

const chat1 = new Chat(generateUuid(), "test", "test chat");
const chats = ref([chat1]);
const selectedChatId = ref('');
const displayChatCreationDialog = ref(false);

const props = defineProps({
  current_user_id: String,
});

const emit = defineEmits([
  "select_chat"
]);

onMounted(async () => {
  chats.value = get_cache("chats");
  selectedChatId.value = get_cache('lastSelectedChat') ? get_cache('lastSelectedChat') : '';
  console.debug("Getting Chats FROM cache", chats.value);
  setInterval(async () => { await updateChat() }, 5000);
});

async function updateChat() {
  try {
    const payload = await get_chats();
    chats.value = Array.isArray(payload.body)
      ? payload.body.map((chat) => ({
          chatId: chat.id,
          chatName: chat.chat_name,
          chatDesc: chat.chat_desc,
          createdAt: chat.created_at,
        }))
      : [];

    set_cache("chats", chats.value);
  } catch (error) {
    console.error("Failed to load chats", error);

    chats.value = get_cache("chats"); 
  }
}

function chooseChat(chatId) {
  selectedChatId.value = chatId ? chatId : get_cache('lastSelectedChat');
  set_cache("lastSelectedChat", selectedChatId.value);
  emit("select_chat", chatId)
}

function openChatCreationDialog() {
  displayChatCreationDialog.value = true;
}

async function createChat(chatName, chatDesc) {
  try {
    const response = await create_chat(chatName, chatDesc);
    if (!response?.raw?.ok) {
      console.error('Failed to create chat on server.', response);
      return;
    }
    await updateChat();
  } catch (error) {
    console.error('Network error while creating chat.', error);
  } finally {
    displayChatCreationDialog.value = false;
  }
}

async function editChat(chat) {
  try {
    const response = await update_chat(chat.chatId, chat.chatName, chat.chatDesc);
    if (!response?.raw?.ok) {
      console.error('Failed to Edit chat on server.', response);
      return;
    }
    await updateChat();
  } catch(error) {
    console.error('Network error while saving changes.', error);
  }
}

async function deleteChat(chat) {
  const targetId = chat.chatId ? chat.chatId : '';

  try {
    await delete_chat(targetId);
    await updateChat();
  } catch(error) {
    console.error("Network Error while deleting Chats.", error);
  } finally {
    emit("select_chat", '');
  }
}

</script>

<template>
  <div class="sidebar">
    
    <div class="chat-panel widget">
      <div class="search-bar">
        <input type="search" placeholder="Search Chats..." />
      </div>
      <div class="chat-list">
        <template v-if="!chats.length">
          <div class="chats-placeholder-text-wrapper">
            <p class="chats-placeholder-text">No chats were found</p>
          </div>
        </template>
        
        <sidebarChat
          v-for="chat in chats"
          :key="chat.chatId"
          :chatName="chat.chatName"
          :chatDesc="chat.chatDesc"
          :createdAt="chat.createdAt"
          :chatId="chat.chatId"
          :selected="chat.chatId === selectedChatId"
          @select="chooseChat(chat.chatId)"
          @edit="editChat"
          @contextmenu.prevent="deleteChat(chat, chats)"
        />
        <chatCreationDialog
          v-if="displayChatCreationDialog"
          @close="displayChatCreationDialog = false"
          @create="createChat($event.chatName, $event.chatDesc)"
        />
        <div class="add-chat-button-wrapper">
          <div
            class="add-chat-button"
            @click="openChatCreationDialog">
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
  grid-column: 1;
  min-width: 0;
  width: 100%;
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

.chats-placeholder-text-wrapper {
  display: flex;
  align-items: flex-start;
  justify-content: center;
  margin-bottom: 10px;
}

.chats-placeholder-text {
  color: var(--theme-light-gray);
  margin: 0;
  text-align: center;
  font-size: 0.9em;
}

@media (max-width: 500px) {
  .sidebar {
    display: none;
  }
}
</style>