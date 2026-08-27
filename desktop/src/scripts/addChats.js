import { baseURL } from './config.js';

const addContactButton = document.querySelector('.add-contact-button');

addContactButton.addEventListener('click', () => {
  createNewChat();
});

async function createNewChat(chatName) {
  const contactContainer = document.querySelector('.contacts');

  const chat = await fetch(`${baseURL}/chats`, {
    headers: header(),
    method: 'POST',
    body: JSON.stringify({ name: chatName })
  });

  const HTML = `
    <div class="contact">
      <div class="contact-picture-cont">
        <!--img src="assets/James_E._Webb,_official_NASA_photo,_1966.jpg" class="your-profile-picture">-->
      </div>
      <div class="contact-info">
        <p class="your-profile-heading">${chat.chatName}</p>
        <p class="your-profile-number"></p>
        <p class="your-profile-name"></p>
      </div>
    </div>
  `;

  document.removeChild(document.querySelector('.no-chats-placeholder'));
  contactContainer.innerHTML = HTML;
}
