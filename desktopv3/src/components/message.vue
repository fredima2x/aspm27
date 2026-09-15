<script setup>
defineProps({
  message: String,
  own: Boolean,
});
</script>

<template>
  <div class="message-wrapper" :class="{ own }">
    <div class="message" :class="{ own }">
      <p>{{ message }}</p>
    </div>
  </div>
</template>

<style scoped>
@import "../assets/styles/general.css";
@import "../assets/styles/root.css";

.message-wrapper {
  display: flex;
  width: 100%;
  min-width: 0;
  box-sizing: border-box;
  padding: 0 10px;
}

.message-wrapper.own {
  justify-content: flex-end;
}

.message {
  max-width: 40%;
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
