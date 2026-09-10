<script setup>
import { sidebar } from '../../components/sidebar.vue';
import { contentPanel } from '../../components/contentPanel.vue'
</script>

<template>
	<div class="app-background">
		<sidebar/>
		<contentPanel/>
	</div>
</template>

<style scoped>
@import '/src/assets/styles/general.css';

* {
  box-sizing: border-box;
}

.app-background {
	display: flex;
	width: 100vw;
	height: 100%;
	margin: 0;
	padding: 0;
}
</style>
