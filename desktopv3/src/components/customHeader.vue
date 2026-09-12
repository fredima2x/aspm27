<script setup>
import fullscreenIcon from "../assets/fullscreen-icon.svg";
import quitIcon from "../assets/quit-icon.svg";

const isTauri =
  typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__);
let currentWindow = null;

async function attachCurrentWindow() {
  if (!isTauri) {
    return;
  }

  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    currentWindow = getCurrentWindow();
  } catch {
    currentWindow = null;
  }
}

attachCurrentWindow();

function toggleFullscreen() {
  if (!isTauri || !currentWindow) {
    return;
  }
  currentWindow.setFullscreen(!currentWindow.isFullscreen());
}

function quit() {
  if (!isTauri || !currentWindow) {
    return;
  }
  currentWindow.close();
}

function startDragging() {
  if (!isTauri || !currentWindow) {
    return;
  }
  currentWindow.startDragging();
}
</script>

<template>
  <header @mousedown="startDragging">
    <div class="app-controls">
      <button
        v-if="isTauri"
        @click="toggleFullscreen"
        @mousedown.stop
        class="app-control-fullscreen"
      >
        <img :src="fullscreenIcon" class="icon" alt="Fullscreen" />
      </button>
      <button
        v-if="isTauri"
        @click="quit"
        @mousedown.stop
        class="app-control-quit"
      >
        <img :src="quitIcon" class="icon" alt="Quit" />
      </button>
    </div>
  </header>
</template>

<style scoped>
header {
  background-color: var(--theme-gray);
  position: sticky;
  padding: 5px;
  margin: 0;
  right: 0;
  left: 0;
  top: 0;
  height: 30px;
  border-bottom: solid 2px var(--theme-primary);
  z-index: 10;
}

.app-controls {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  min-height: 20px;
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
