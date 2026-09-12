<script setup>
const props = defineProps({
  chatId: Number,
  chatName: String,
  chatDesc: String,
  selected: Boolean,
});

const emit = defineEmits(["select"]);
</script>
<template>
  <div class="chat" :class="{ selected: props.selected }" @click="emit('select', props.chatId)">
    <h2 class="chat-name">{{ props.chatName }}</h2>
    <p class="chat-desc">{{ props.chatDesc }}</p>
    <p class="chat-id">{{ props.chatId }}</p>
  </div>
</template>
<style scoped>
@import "../assets/styles/general.css";
@import "../assets/styles/root.css";

.chat {
  margin: 10px 0;
  padding: 10px;
  border-radius: var(--theme-round-edges);
  background-color: var(--theme-light-gray);
	color: var(--theme-white);
  cursor: pointer;
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
</style>
