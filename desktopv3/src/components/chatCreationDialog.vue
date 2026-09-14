<script setup>
import { ref } from 'vue';

const emit = defineEmits(['close', 'create']);
const chatName = ref('');
const chatDesc = ref('');
const warning = ref('');

function closeDialog() {
  warning.value = '';
  emit('close');
}

function submitCreateChat() {
  const trimmedName = chatName.value.trim();
  const trimmedDesc = chatDesc.value.trim();

  if (!trimmedName) {
    warning.value = 'Chat name cannot be empty.';
    return;
  }

  if (trimmedName.length < 3) {
    warning.value = 'Chat name must be at least 3 characters.';
    return;
  }

  warning.value = '';
  emit('create', {
    chatName: trimmedName,
    chatDesc: trimmedDesc,
  });
}
</script>

<template>
  <div class="backdrop">
    <div class="chatCreationDialog">
      <form @submit.prevent="submitCreateChat">
        <p>Enter chat name:</p>
        <input v-model="chatName" maxlength="24" type="text" placeholder="e.g. 'Class-chat'" />
        <p>Enter chat description:</p>
        <textarea v-model="chatDesc" maxlength="60" placeholder="Optional" />
        <p v-show="warning" class="warn-text">{{ warning }}</p>
        <button type="submit">Create chat</button>
        <p @click="closeDialog" class="close-button">x</p>
      </form>
    </div>
  </div>
</template>

<style scoped>
  @import "../assets/styles/general.css";
  @import "../assets/styles/root.css";

  textarea {
    height: 70px;
  }

  .backdrop {
    backdrop-filter: blur(7px);
    top: 0;
    bottom: 0;
    right: 0;
    left: 0;
    height: 100%;
    position: fixed;
    align-content: center;
    justify-items: center;
    z-index: 10;
  }

  .chatCreationDialog {
    background-color: var(--theme-light-gray);
    border-radius: var(--theme-round-edges);
    padding: 10px;
    box-shadow: 0 0 5px var(--theme-light-gray);
    position: relative;
  }

  .close-button {
    position: absolute;
    top: -5px;
    right: 10px;
    cursor: pointer;
    font-size: 1em;
    font-weight: bold;
  }

  .warn-text {
    color: var(--theme-red);
  }
</style>
