import { baseURL } from './config.js';
import { header } from './main.js';

const addContactButton = document.querySelector('.add-contact-button');

addContactButton.addEventListener('click', () =>{
  console.log('Add contact button clicked');

  const chatName = prompt('Enter the name of the new chat:');
  createNewChat(chatName);
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
