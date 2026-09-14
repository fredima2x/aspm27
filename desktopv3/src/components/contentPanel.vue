<script setup>
import { onMounted, ref } from 'vue';
import Message from './message.vue';
import get_messages from '../api/requests/get_messages.js';
import { get_cache } from '../stores/cache.js';
import send_message from '../api/requests/send_message.js';

const messages = ref([]);
const message_input = ref('');

const props = defineProps({
	current_user_id: String,
	selected_chat_id: String,
});

onMounted(async () => {
	messages.value = get_cache(`messages?chatID=${props.selected_chat_id}`)
	setInterval(update_messages, 2000);
});

async function update_messages() {
	try {
		const res = await get_messages(props.selected_chat_id, 100, 0);
		if (!res.raw.ok) {
			console.log("Error fetching Messages!", res);
		}
		messages.value = res.body;
	} catch {
		console.log("Error fetching Messages!", res);
		messages.value = get_cache(`messages?chatID=${props.selected_chat_id}`)
	}
}

async function sendMessage() {
	const res = await send_message(props.selected_chat_id, message_input.value)
	if (res.raw.ok) {
		messages.value.push(res.body);
	} else {
		console.error("Failed to send Message!", res);
	}
}

</script>

<template>
  <div class="content-panel">
    <div class="message-panel widget">
      <div class="message-list">
        <Message v-for="message in messages" :message="message.content" :own="message.owner_id === current_user_id"/>
      </div>
      <div class="message-input">
        <input v-model="message_input" @keydown.enter="sendMessage" type="text" placeholder="Enter Message...">
        <button @click="sendMessage" class="send-button">Send</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
@import '../assets/styles/general.css';
@import '../assets/styles/root.css';

.content-panel {
	display: flex;
	flex: 2;
	height: 100%;
	margin: 0;
}

.widget {
	display: flex;
	flex-direction: column;
	margin: 10px;
	padding: 10px;
	background-color: var(--theme-gray);
	border-radius: 10px;
}

.message-panel {
	flex: 1;
}

.message-list {
	flex: 1;
	min-height: 0;
	overflow-y: auto;
	background-color: var(--theme-gray);
	border-radius: 10px;
}

.message-input {
	display: flex;
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
</style>