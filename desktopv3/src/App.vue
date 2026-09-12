<script setup>
import customHeader from './components/customHeader.vue';
</script>

<template>
    <customHeader />
    <router-view/>
</template>

<style>
@import '/src/assets/styles/root.css';

html,
body,
#app {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;

    background-color: black;
    display: flex;
    gap: 10px;
    box-sizing: border-box;
}

#app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}
</style>
