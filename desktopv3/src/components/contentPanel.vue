<script setup>

</script>

<template>
  <div class="content-panel">
    <div class="message-panel widget">
      <div class="message-list">
        <message message="Hello!"/>
      </div>
      <div class="message-input">
        <input type="text" placeholder="Enter Message...">
        <button class="send-button">Send</button>
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