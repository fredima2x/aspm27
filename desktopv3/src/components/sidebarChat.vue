<script setup>
import { ref } from 'vue';
import chatSettingsDialog from './chatSettingsDialog.vue';

const props = defineProps({
  chatId: String,
  chatName: String,
  chatDesc: String,
  selected: Boolean,
});

const emit = defineEmits(['select', 'edit']);

const displayChatSettingsDialog = ref(false);

function openChatEditDialog() {
  displayChatSettingsDialog.value = true;
}

function closeChatEditDialog() {
  displayChatSettingsDialog.value = false;
}

function submitChatEdit(chat) {
  emit('edit', {
    chatId: props.chatId,
    chatName: chat.chatName,
    chatDesc: chat.chatDesc,
  });
  closeChatEditDialog();
}

</script>

<template>
  <div class="chat" :class="{ selected: props.selected }" @click="emit('select', props.chatId)">
    <h2 class="chat-name">{{ props.chatName }}</h2>
    <p class="chat-desc">{{ props.chatDesc }}</p>
    <p class="chat-id">{{ props.chatId }}</p>
    <img src="../assets/kebab-menu-svgrepo-com.svg" alt="menu" class="menu" @click.stop="openChatEditDialog">
    <chatSettingsDialog
      v-if="displayChatSettingsDialog"
      :chat-name="props.chatName"
      :chat-desc="props.chatDesc"
      @close="closeChatEditDialog"
      @edit="submitChatEdit"
    />
  </div>
</template>

<style scoped>
@import "../assets/styles/general.css";
@import "../assets/styles/root.css";
@import "../assets/styles/utils/animations.css";

.chat {
  padding: 10px;
  padding-right: 44px;
  margin-bottom: 10px;
  border-radius: var(--theme-round-edges);
  background-color: var(--theme-light-gray);
	color: var(--theme-white);
  cursor: pointer;
  animation: fadeInTop 0.5s ease-in-out;
  position: relative;
}

.chat.selected {
  background-color: var(--theme-primary);
}

.chat-name {
  margin-bottom: 20px;
	margin-top: 0;
}

.chat-desc,
.chat-id {
  margin-bottom: 2px;
}

.menu {
  position: absolute;
  top: 10px;
  right: 10px;
  width: 24px;
  height: 24px;
  display: block;
  cursor: pointer;
  color: var(--theme-gray);
}
</style>
