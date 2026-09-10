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
@import '/src/assets/styles/general.css';

.content-panel {
	display: flex;
	flex: 2;
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

.message-panel {
	flex: 1;
}

.message-list {
	flex: 1;
	min-height: 0;
	overflow-y: auto;
	background-color: var(--theme-gray);
	border: 4px solid var(--theme-primary);
	border-radius: 10px;
}

.message-input {
	display: flex;
	gap: 10px;
	padding-top: 10px;
}

.message-input input {
	flex: 1;
	margin: 0;
}

.send-button {
	width: 80px;
	margin: 0;
}
</style>