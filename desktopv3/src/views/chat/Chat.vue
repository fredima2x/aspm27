<template>
    <div class="background">
        <div class="contacts-window">
            <div class="your-profile">
                <div class="contact-picture-cont">
                     <img src="https://picsum.photos/300/300" class="opened-contact-picture">
                </div>
                <div class="contact-info">
                    <p class="your-profile-heading"></p>
                    <p class="your-profile-number"></p>
                    <p class="your-profile-name"></p>
                </div>
            </div>
            <div class="top-contact-bar">
                <input type="search" class="contact-search-bar" placeholder="Search Chats...">
            </div>
            <div class="contacts">
                <p class="no-chats-placeholder">No chats found</p>
                <div class="add-contact-button-cont">
                    <button class="add-contact-button">+</button>
                </div>
            </div>
        </div>
    </div>
    <div class="chat-window">
        <div class="opened-contact">
            <div class="opened-contact-picture-cont">
                <img src="https://picsum.photos/300/300" class="opened-contact-picture">
            </div>
            <div class="contact-info">
                <p class="opened-contact-heading"></p>
                <p class="opened-contact-number"></p>
                <p class="opened-contact-name"></p>
            </div>
        </div>

        <div class="chat-area"></div>
        <div class="message-bar">
            <input type="text" class="message-text-bar" placeholder="Message">
            <button class="send-button">Send</button>
            <button class="delete-button"> - </button>
        </div>
    </div>
</template>

<style scoped>

</style>
