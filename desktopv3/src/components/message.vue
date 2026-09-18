<script setup>
const props = defineProps({
  message: String,
  own: Boolean,
  message_owner: String,
  close: Boolean,
});
</script>

<template>
  <div class="message-wrapper" :class="{ own, close }">
    <div class="message" :class="{ own }">
      <p>{{ message }}</p>
    </div>
    <p class="owner-name" v-if="!own">{{ message_owner }}</p>
  </div>
</template>

<style scoped>
@import "../assets/styles/general.css";
@import "../assets/styles/root.css";

.owner-name {
  color: var(--theme-light-light-gray);
  margin-top: 0;
}

.message-wrapper {
  display: flex;
  width: 100%;
  min-width: 0;
  box-sizing: border-box;
  padding: 0 10px;
  flex-direction: column;
  align-items: flex-start;
}

.message-wrapper.own {
  justify-content: flex-end;
  align-items: flex-end;
}

.message-wrapper.close,
.message-wrapper.own.close {
  margin-top: 0;
  margin-bottom: 0;
}

.message-wrapper.close .message {
  margin-top: 0;
}

.message {
  max-width: 60%;
  min-width: 0;

  padding: 8px 12px;
  margin: 4px 0;

  background-color: var(--theme-light-light-gray);
  border-radius: 12px 12px 12px 5px;

  overflow-wrap: anywhere;
  word-break: break-word;
}

.message.own {
  background-color: var(--theme-primary);
  border-radius: 12px 12px 5px 12px;
}

.message.own p {
  color: var(--theme-gray);
}

.message p {
  margin: 0;
  color: var(--theme-white);
}
</style>
