<script setup>
defineProps({
  chat_id: BigInt,
  chat_name: String,
  chat_desc: String,

  selected: Boolean,
});
</script>
<template>
    <div class="chat" :class="selected">
        <p class="chat-name">{{ chat_name }}</p>
        <p class="chat-desc">{{ chat_desc }}</p>
    </div>
</template>
<style scoped>
.chat {
    margin: 10px;
    padding: 10px;
    border-radius: 5px;
    background-color: var(--theme-light-gray);
}
.chat .selected {
    background-color: var(--theme-primary);
}
.chat-name, .chat-desc {
    margin: 0;
}
.chat-name {
    color: var(--theme-white);
}
.chat-desc {
    color: var(--theme-white);
}
</style>
