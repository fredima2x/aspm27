<template>
    <div class="background">
        <div class="left-sidebar">
            <div class="chat-window">
                <input type="search" placeholder="Search Chats...">
                <div class="chat-container"></div>
            </div>
        </div>
        <div class="main-window">
            <div class="message-window">
                <div class="message-container"></div>
                <div class="message-input-bar">
                    <input type="text" placeholder="Enter Message...">
                    <button class="send-button">Send</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
@import '/src/assets/styles/general.css';
:root {
    --theme: rgb(0, 219, 124);
}

.background {
    margin: 0;
    padding: 0;
    display: flex;
    gap: 0;
    height: 100%;
    width: 100vw;
    flex-direction: row;
}
.left-sidebar, .main-window {
    display: flex;
    flex-direction: column;
    background-color: rgb(30,30,30);
    height: calc(100% - 40px);
    border-radius: 10px;
    margin: 10px;
    padding: 10px;
}
.left-sidebar {
    flex: 1;
    margin-right: 0;
}
.main-window {
    flex: 2;
}

.chat-window, .message-window {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
}

.message-container, .chat-container {
    height: 100%;
    overflow-y: auto;
    background-color: black;
    border: 4px solid rgb(0, 219, 124);
    border-radius: 10px;
}
.message-container {
    flex: 1;
}
.chat-container {
    flex: 2;
}

.message-input-bar {
    display: flex;
    flex-direction: row;
    gap: 10px;
    padding-top: 10px;
}

.message-input-bar input {
    margin: 0;
    flex: 1;
}

.send-button {
    margin: 0;
    width: 80px;
}
</style>
