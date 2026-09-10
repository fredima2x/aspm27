<script setup>
const isTauri = !!window.__TAURI_INTERNALS__;

import { ref } from 'vue';
import fullscreenIcon from "../assets/fullscreen-icon.svg";
import quitIcon from "../assets/quit-icon.svg";

let currentWindow;
if (isTauri) {
    const { getCurrentWindow } = await import('@tauri-apps/api/window');
    currentWindow = getCurrentWindow();
}

const reactiveIsTauri = ref(isTauri);

function toggleFullscreen() {
    if (!isTauri) {return}
    currentWindow.setFullscreen(!currentWindow.isFullscreen());
}

function quit() {
    if (!isTauri) {return}
    currentWindow.close();
}

function startDragging() {
    if (!isTauri) {return}
    currentWindow.startDragging();
}

</script>

<template>
  <header @mousedown="startDragging" v-if="reactiveIsTauri">
    <div class="app-controls">
      <button
        @click="toggleFullscreen"
        @mousedown.stop
        class="app-control-fullscreen"
      >
        <img :src="fullscreenIcon" class="icon" alt="Fullscreen" />
      </button>
      <button @click="quit" @mousedown.stop class="app-control-quit">
        <img :src="quitIcon" class="icon" alt="Quit" />
      </button>
    </div>
  </header>
</template>

<style scoped>
header {
  background-color: var(--theme-gray);
  display: fixed;
  padding: 5px;
  margin: 0;
  right: 0;
  left: 0;
  top: 0;
  height: 20px;
  border-bottom: solid 2px var(--theme-primary);
}

.app-controls {
  display: flex;
  align-items: center;
  justify-content: flex-start;
}

header .icon {
  width: 16px;
  filter: brightness(0) invert(1);
}

header button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  margin: 0;
}
</style>
