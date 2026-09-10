<script setup>

</script>

<template>
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
</template>

<style scoped>
.sidebar {
	display: flex;
	flex-direction: column;
	flex: 1;
	min-width: 0;
	height: 100%;
}

.widget {
	display: flex;
	flex-direction: column;
	margin: 10px;
	padding: 10px;
	background-color: rgb(30, 30, 30);
	border-radius: 10px;
}

.profile-panel {
	flex: 0 0 100px;
	margin-right: 0;
	margin-bottom: 0;
}

.chat-panel {
	flex: 1;
	margin-right: 0;
}

.chat-list {
	flex: 1;
	min-height: 0;
	overflow-y: auto;
	background-color: var(--theme-gray);
	border: 4px solid var(--theme-primary);
	border-radius: 10px;
}
</style>