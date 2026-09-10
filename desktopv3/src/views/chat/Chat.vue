<script setup>
import message from '../../components/message.vue';
import chat from '../../components/chat.vue';
import { get_chats } from '../../api/requests/get_chats';
import { ref } from 'vue';
import { onMounted, onUnmounted } from 'vue';

const chats = ref('');

onMounted(async () => {
  await get_chats()

  interval = setInterval(async () => {
      await getChats()
  }, 5000)
})

onUnmounted(() => {
  clearInterval(interval)
})

async function getChats() {
  try {
    const data = await get_chats();
    chats.value = data;
  } catch {
    console.error("Failed to get Chats!", data);
  }
}
</script>

<template>
    <div class="app-background">
        <div class="sidebar">
            <div class="profile-panel widget">
            </div>
            <div class="chat-panel widget">
                <input type="search" placeholder="Search Chats...">
                <div class="chat-list">
                    <chat v-for="chat in chats" :chat_name="chat.chat_name" :chat_desc="chat.chat_desc" :chat_id="chat.chat_id" />
                </div>
            </div>
        </div>
        <div class="content-panel">
            <div class="message-panel widget">
                <div class="message-list">
                    <message message="Hello!"/>
                </div>
                <div class="message-input">
                    <input type="text" placeholder="Enter Message...">
                    <button class="send-button">Send</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
@import '/src/assets/styles/general.css';

* {
    box-sizing: border-box;
}

.app-background {
    display: flex;
    width: 100vw;
    height: 100%;
    margin: 0;
    padding: 0;
}

.sidebar {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
    height: 100%;
}

.content-panel {
    display: flex;
    flex: 2;
    min-width: 0;
    height: 100%;
}

/* Common widget styling */
.widget {
    display: flex;
    flex-direction: column;
    margin: 10px;
    padding: 10px;
    background-color: rgb(30, 30, 30);
    border-radius: 10px;
}

/* Sidebar */
.profile-panel {
    flex: 0 0 100px;
    margin-right: 0;
    margin-bottom: 0;
}

.chat-panel {
    flex: 1;
    margin-right: 0;
}

/* Messages */
.message-panel {
    flex: 1;
}

.message-list,
.chat-list {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    background-color: var(--theme-gray);
    border: 4px solid var(--theme-primary);
    border-radius: 10px;
}

.message-input {
    display: flex;
    gap: 10px;
    padding-top: 10px;
}

.message-input input {
    flex: 1;
    margin: 0;
}

.send-button {
    width: 80px;
    margin: 0;
}
</style>
