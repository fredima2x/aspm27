<template>
    <div class="contacts-window">
    <div class="your-profile">
        <div class="contact-picture-cont">
        <!--img src="assets/James_E._Webb,_official_NASA_photo,_1966.jpg" class="your-profile-picture">-->
        </div>
        <div class="contact-info">
        <p class="your-profile-heading">Loading...</p>
        <p class="your-profile-number"></p>
        <p class="your-profile-name"></p>
        </div>
    </div>
    <div class="top-contact-bar">
        <input type="search" class="contact-search-bar" placeholder="Search Chats">
    </div>
    <div class="contacts">
        <p class="no-chats-placeholder">No chats found</p>
        <div class="add-contact-button-cont">
        <button class="add-contact-button">+</button>
        </div>
    </div>
    </div>
    <div class="chat-window">

    <div class="opened-contact">
        <div class="opened-contact-picture-cont">
        <!---<img src="https://picsum.photos/300/300" class="opened-contact-picture">-->
        </div>
        <div class="contact-info">
        <p class="opened-contact-heading"></p>
        <p class="opened-contact-number"></p>
        <p class="opened-contact-name"></p>
        </div>
    </div>

    <div class="chat-area">
    </div>
    <div class="message-bar">
        <input type="text" class="message-text-bar" placeholder="Message">
        <button class="send-button">Send</button>
        <button class="delete-button"> - </button>
    </div>
    </div>
</template>

<style scoped>
.chat-window {
    background-color: rgb(15, 15, 15);
    flex: 1;
    min-height: 0;
    min-width: 500px;
    border-radius: 10px;
    display: grid;
    grid-template-rows: auto 1fr;
    gap: 10px;
    padding: 10px;
    box-sizing: border-box;
}

.opened-contact {
    background-color: rgb(0, 219, 124);
    border-radius: 10px;
    padding: 10px;
    display: inline-flex;
    justify-content: center;
    align-items: center;
    gap: 10px;
    justify-self: right;
    align-self: start;
}

.opened-contact-heading {
    margin: 0px;
    font-weight: bold;
    color: rgb(46, 46, 46);
}

.opened-contact-number,
.opened-contact-name {
    font-size: 13px;
    color: rgb(90, 90, 90);
    margin: 2px;
}

.opened-contact-picture-cont {
    margin-right: 10px;
}

.opened-contact-picture {
    width: 50px;
    height: 50px;
    border-radius: 25px;
}

.chat-area {
    background-color: rgb(27, 27, 27);
    width: 100%;
    border-radius: 10px;
    box-shadow: 0 0 10px rgb(30, 30, 30);
    overflow-y: auto;
    max-height: 100%;
}

.message-them,
.message-me {
    display: flex;
    flex-direction: column;
    padding: 10px;
}

.message-them {
    align-items: flex-start;
}

.message-me {
    align-items: flex-end;
}

.sender {
    font-size: 12px;
    color: rgb(150, 150, 150);
    margin: 0px 3px 3px 3px;
}

.message-cont {
    border-radius: 10px;
    display: inline-block;
    max-width: 400px;
    padding: 10px;
}

.message-them .message-cont {
    background-color: rgb(67, 67, 67);
    color: rgb(209, 209, 209);
}

.message-me .message-cont {
    background-color: rgb(0, 219, 124);
}

.message {
    font-size: 14px;
    margin: 0px;
    overflow-wrap: anywhere;
}

.message-bar {
    background-color: rgb(27, 27, 27);
    padding: 10px;
    border-radius: 10px;
    display: flex;
    max-height: 40px;
    align-self: flex-end;
    box-shadow: 0 0 10px rgb(30, 30, 30);
}

.send-button,
.delete-button {
    margin-left: 10px;
    padding: 10px;
    background-color: rgb(0, 219, 124);
}

.delete-button {
    padding-left: 15px;
    padding-right: 15px;
}
.contacts-window {
    background-color: rgb(15, 15, 15);
    width: 320px;
    height: 100%;
    border-radius: 10px;
    display: grid;
    grid-template-rows: auto auto 1fr;
    padding: 10px;
    box-sizing: border-box;
}

.top-contact-bar {
    background-color: rgb(27, 27, 27);
    border-radius: 10px;
    box-shadow: 0 0 10px rgb(30, 30, 30);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 10px;
    margin-bottom: 10px;
    height: 53px;
}

.contacts {
    background-color: rgb(27, 27, 27);
    border-radius: 10px;
    box-shadow: 0 0 10px rgb(30, 30, 30);
    padding: 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    min-height: 0;
    overflow-y: auto;
}

.your-profile,
.contact {
    border-radius: 10px;
    padding: 10px;
    margin-bottom: 10px;
    display: flex;
    text-align: left;
    background-color: rgb(67, 67, 67);
    min-width: 280px;
    color: inherit;
    font: inherit;
}

.contact.selected {
    background-color: rgb(0, 219, 124);
    color: rgb(46, 46, 46);
}

.your-profile {
    background-color: rgb(0, 219, 124);
    color: rgb(46, 46, 46);
    align-items: center;
    gap: 10px;
}

.your-profile-heading,
.contact-heading {
    margin: 0px;
    font-weight: bold;
    color: rgb(209, 209, 209);
}

.your-profile-heading {
    color: rgb(46, 46, 46);
}

.settings-cont {
    display: flex;
    align-items: center;
    margin-left: auto;
}

.settings-button {
    background-color: transparent;
    border: none;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: transform 0.5s ease;
}

.settings-button:hover .settings-picture {
    transform: rotate(90deg);
}

.settings-picture {
    width: 50px;
    transition: transform 0.5s ease;
}

.contact-number,
.contact-name,
.contact-id {
    font-size: 13px;
    color: rgb(150, 150, 150);
    margin: 2px;
    overflow: hidden;
}

.your-profile-name,
.your-profile-number {
    font-size: 13px;
    margin: 2px;
    color: rgb(90, 90, 90);
}

.contact-picture-cont {
    margin-right: 10px;
}

.your-profile-picture,
.contact-picture {
    width: 50px;
    height: 50px;
    border-radius: 25px;
    object-fit: cover;
}

.no-chats-placeholder {
    color: rgb(112, 112, 112);
    font-size: 15px;
}

.add-contact-button-cont {
    display: flex;
    align-items: center;
    justify-content: center;
}

.add-contact-button {
    border-radius: 25px;
    background-color: rgb(67, 67, 67);
    color: rgb(209, 209, 209);
    height: 50px;
    width: 50px;
    font-size: 30px;
}
html,
body {
    height: 100vh;
    width: 100vw;
    margin: 0;
}

body {
    background-color: black;
    padding: 10px;
    display: flex;
    gap: 10px;
    box-sizing: border-box;
}

p {
    font-family: Arial;
}

button {
    transition: opacity 0.1s;
    border: none;
    border-radius: 25px;
    cursor: pointer;
}

button:hover {
    opacity: 0.8;
}

button:active {
    opacity: 0.6;
}

input {
    border-radius: 20px;
    border: none;
    flex: 1;
    padding: 10px;
    background-color: rgb(27, 27, 27);
    color: rgb(200, 200, 200);
    font-size: 14px;
}

input:focus {
    outline: none;
}

</style>
