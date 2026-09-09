<script setup>
// Libraries
import { getCurrentWindow } from '@tauri-apps/api/window';

// Assets
import fullscreenIcon from '/src/assets/fullscreen-icon.svg';
import quitIcon from '/src/assets/quit-icon.svg';

const currentWindow = getCurrentWindow();

function toggleFullscreen() {
    currentWindow.setFullscreen(!currentWindow.isFullscreen());
}
function quit() {
    currentWindow.close();
}
function startDragging() {
    currentWindow.startDragging();
}
</script>

<template>
    <header @mousedown="startDragging">
        <div class="app-controls">
            <button @click="toggleFullscreen" @mousedown.stop class="app-control-fullscreen"><img :src="fullscreenIcon" class="icon" alt="Fullscreen"></button>
            <button @click="quit" @mousedown.stop class="app-control-quit"><img :src="quitIcon" class="icon" alt="Quit"></button>
        </div>
    </header>

    <router-view/>

</template>

<style scoped>
header {
    background-color: black;
    display: flex;
    align-items: center;
    padding: 10px;
    margin: 0;
    height: 15px;
}

.app-controls {
    margin-left: auto;
}

header .icon {
    width: 16px;
    height: 16px;
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


<style>
html,
body,
#app {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
}

#app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}
</style>
