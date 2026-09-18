<script setup>
import { nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import Message from "./message.vue";
import get_messages from "../api/requests/get_messages.js";
import { get_cache, set_cache } from "../stores/cache.js";
import send_message from "../api/requests/send_message.js";
import { get_user } from "../api/requests/get_user.js";

const messages = ref([]);
const message_input = ref("");
const message_list = ref("");
const userCache = ref({});

const props = defineProps({
  current_user: Object,
  selected_chat_id: String,
});

let interval;

onMounted(async () => {
  if (props.selected_chat_id) {
    messages.value = get_cache(`messages?chatID=${props.selected_chat_id}`);
    await update_messages();
  }

  interval = setInterval(update_messages, 2000);
});

onUnmounted(() => {
  clearInterval(interval);
});

watch(
  () => props.selected_chat_id,
  async () => {
    await update_messages();
    await scrollToBottom();
  },
);


watch(
  messages,
  async (newMessages) => {
    for (const message of newMessages) {
      await get_message_owner(message.owner_id);
    }
  },
  { immediate: true }
);

async function update_messages() {
  if (!props.selected_chat_id) {
    messages.value = [];
  }

  try {
    const res = await get_messages(props.selected_chat_id, 100, 0);
    if (!res.raw.ok) {
      console.log("Error fetching Messages!", res);
    }
    messages.value = res.body;
  } catch (error) {
    console.log("Error fetching Messages!", error);
    messages.value = get_cache(`messages?chatID=${props.selected_chat_id}`);
  } 
}

async function sendMessage() {
  if (!message_input.value) {
    return;
  }

  const res = await send_message(props.selected_chat_id, message_input.value);

  message_input.value = "";

  if (res.raw.ok) {
    messages.value.push(res.body);
  } else {
    console.error("Failed to send Message!", res);
  }
  await scrollToBottom();
}

async function scrollToBottom() {
  await nextTick();

  if (message_list.value) {
    message_list.value.scrollTop = message_list.value.scrollHeight;
  }
}

async function get_message_owner(id) {
  if (get_cache(`user?id=${id}`)) {
    return get_cache(`user?id=${id}`);
  }

  const res = await get_user(id);

  set_cache(`user?id=${id}`, res.body.username);

  return res.body.username;
}

</script>

<template>
  <div class="content-panel">
    <div class="profile-panel widget"></div>

    <div class="message-panel widget">
      <template v-if="selected_chat_id">
        <div class="message-list" ref="message_list">
          <Message
            v-for="(message, index) in messages"
            :key="message.id"
            :message="message.content"
            :message_owner="get_cache(`user?id=${message.owner_id}`) || '...'"
            :own="String(message.owner_id) === String(current_user?.id)"
            :close="index > 0 && String(message.owner_id) === String(messages[index - 1].owner_id)"
          />

          <div class="message-placeholder"></div>
        </div>

        <div class="message-input">
          <input
            v-model="message_input"
            @keydown.enter="sendMessage"
            type="text"
            placeholder="Enter Message..."
          />
          <button @click="sendMessage" class="send-button">
            Send
          </button>
        </div>
      </template>

      <template v-else>
        <div class="chat-placeholder">
          <h1 class="chat-placeholder-text">
            You have no Chat selected!
          </h1>
          <img src="" alt="logo" class="chat-placeholder-img" />
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
@import "../assets/styles/general.css";
@import "../assets/styles/root.css";

.content-panel {
  display: flex;
  grid-column: 2;
  flex-direction: column;
  min-width: 0;
  min-height: 0;
  width: 100%;
  height: 100%;
  margin: 0;
}

.message-panel {
  flex: 1 1 auto;
  min-width: 0;
  min-height: 0;
}

.message-list {
  flex: 1;
  min-width: 0;
  min-height: 0;

  overflow-y: auto;
  overflow-x: hidden;

  background-color: var(--theme-gray);
  border-radius: 10px;
}

.message-list {
  /* Scrollbar verstecken */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* Internet Explorer */
}

.message-list::-webkit-scrollbar {
  display: none; /* Chrome, Edge, Safari */
}

.widget {
  display: flex;
  flex-direction: column;
  margin: 10px;
  padding: 10px;
  background-color: var(--theme-gray);
  border-radius: 10px;
}

.message-input {
  display: flex;
  flex-shrink: 0;
  gap: 10px;
  padding-top: 10px;
  border-top: 3px solid var(--theme-primary);
}

.message-input input {
  flex: 1;
  margin: 0;
  border: 1px solid var(--theme-light-gray);
}

.send-button {
  width: 80px;
  margin: 0;
}

.message-placeholder {
  width: 100%;
  height: 10px;
}

.chat-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

@media (max-width: 500px) {
  .content-panel {
    grid-column: 1;
  }
}

.profile-panel {
  flex: 0 0 100px;
  min-width: 0;
  width: auto;
  margin-bottom: 0;
}
</style>
