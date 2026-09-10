```html
<template>
    <div class="app-background">
        <div class="sidebar">
            <div class="profile-panel">

            </div>
            <div class="chat-panel">
                <input type="search" placeholder="Search Chats...">
                <div class="chat-list"></div>
            </div>
        </div>
        <div class="content-panel">
            <div class="message-panel">
                <div class="message-list"></div>
                <div class="message-input">
                    <input type="text" placeholder="Enter Message...">
                    <button class="send-button">Send</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
@import '/src/assets/styles/general.css';

.app-background {
    margin: 0;
    padding: 0;
    display: flex;
    gap: 0;
    height: 100%;
    width: 100vw;
    flex-direction: row;
}

.sidebar {
    display: flex;
    flex-direction: column;
    flex: 1;
    height: 100%;
}

.chat-panel, .message-panel, .profile-panel {
    display: flex;
    flex-direction: column;
    background-color: rgb(30,30,30);
    height: calc(100% - 40px);
    border-radius: 10px;
    margin: 10px;
    padding: 10px;
}
.chat-panel {
    flex: 2;
    margin-right: 0;
}

.content-panel {
    flex: 2;
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
}

.message-list, .chat-list {
    height: 100%;
    overflow-y: auto;
    background-color: var(--theme-gray);
    border: 4px solid var(--theme-primary);
    border-radius: 10px;
}

.message-list {
    flex: 1;
}
.chat-list {
    flex: 2;
}

.message-input {
    display: flex;
    flex-direction: row;
    gap: 10px;
    padding-top: 10px;
}

.message-input input {
    margin: 0;
    flex: 1;
}

.send-button {
    margin: 0;
    width: 80px;
}

.profile-panel {
    flex: 1;
    margin-right: 0;
    margin-bottom: 0;
}
</style>
