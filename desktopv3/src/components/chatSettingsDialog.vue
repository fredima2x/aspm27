<script setup>
import { ref } from 'vue';

const props = defineProps({
  chatName: {
    type: String,
    default: '',
  },
  chatDesc: {
    type: String,
    default: '',
  },
});

const emit = defineEmits(['close', 'edit']);
const chatName = ref('');
const chatDesc = ref('');
const warning = ref('');

chatName.value = props.chatName;
chatDesc.value = props.chatDesc;

function closeDialog() {
  warning.value = '';
  emit('close');
}

function submitEditChat() {
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
  emit('edit', {
    chatName: trimmedName,
    chatDesc: trimmedDesc,
  });
}

function handle_input() {
  warning.value = '';
}
</script>

<template>
  <div class="backdrop" @click.stop>
    <div class="chatSettingsDialog">
      <form @submit.prevent="submitEditChat">
        <p>Enter new Chat name:</p>
        <input
          @input="handle_input"
          v-model="chatName"
          maxlength="24"
          type="text"
          placeholder="e.g. 'Goon-Corner'"
        />
        <p>Enter new Chat description:</p>
        <textarea
          @input="handle_input"
          v-model="chatDesc"
          maxlength="25"
          placeholder="A short Description of your Chat! (optional)"
          
        />
        <p v-show="warning" class="warn-text">{{ warning }}</p>
        <button type="submit">Save changes</button>
        <p @click="closeDialog" class="close-button">x</p>
      </form>
    </div>
  </div>
</template>

<style scoped>
  @import "../assets/styles/general.css";
  @import "../assets/styles/root.css";

  textarea {
    height: 100px;
    width: 300px;
    resize: none;
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

  .chatSettingsDialog {
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
