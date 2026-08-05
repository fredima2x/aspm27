const base_url = "http://127.0.0.1:3000";
let token = null;

function header() {
    return {
      "Authorization": `Bearer ${token}`,
      "Content-Type": "application/json",
    };
  }

async function login(username, password) {
  const response = await fetch(`${base_url}/login`, {
    method: "POST",
    headers: header(),
    body: JSON.stringify({ username, password }),
  });
  return await response.json();
}

async function get_chats() {
  const response = await fetch(`${base_url}/chats`, {
    method: "GET",
    headers: header(),
  })
  return await response.json();
}

async function loadUsers() {
  try {

    token = (await login("fredima2x", "12341234")).token_string;

    const chats = await get_chats();

    const container = document.querySelector(".contacts");
    if (!container) {
      console.error("Contacts container not found");
      return;
    }

    chats.forEach((chat) => {
      const contactButton = document.createElement("button");

      contactButton.className = "contact";
      contactButton.type = "button";

      contactButton.innerHTML = `
                <div class="contact-picture-cont">
                    <img src="imgs/kreise.pdf" class="contact-picture" />
                </div>
                <div class="contact-info">
                    <p class="contact-heading">
                        ${chat.chat_name}
                    </p>

                    <p class="contact-id">
                        ${chat.chat_desc}
                    </p>
                </div>
            `;

      container.appendChild(contactButton);
    });

    const addContactButton = document.createElement("div");

    addContactButton.className = "add-contact-button-cont";
    addContactButton.type = "div";

    addContactButton.innerHTML = `
                <button class="add-contact-button">+</button>
            `;

    container.appendChild(addContactButton);

  } catch (error) {
    console.error("Fetch Fehler:", error);
  }
}

const messageInput = document.querySelector(".message-text-bar");
const sendButton = document.querySelector(".send-button");
const chatArea = document.querySelector(".chat-area");
const storageKey = "messages";
let messages = JSON.parse(localStorage.getItem(storageKey) || "[]");

function saveMessages() {
  localStorage.setItem(storageKey, JSON.stringify(messages));
}

function renderMessages() {
  chatArea.innerHTML = "";

  messages.forEach((entry) => {
    const messageWrapper = document.createElement("div");
    messageWrapper.className = "message-me";
    messageWrapper.innerHTML = `
      <p class="sender">You</p>
      <div class="message-cont">
        <p class="message"></p>
      </div>
    `;

    messageWrapper.querySelector(".message").textContent = entry.message;
    chatArea.appendChild(messageWrapper);
  });

  chatArea.scrollTop = chatArea.scrollHeight;
}

function displayMessage() {
  const text = messageInput.value.trim();

  if (!text) {
    return;
  }

  messages.push({ message: text });
  saveMessages();
  renderMessages();
  messageInput.value = "";
}

sendButton.addEventListener("click", displayMessage);

messageInput.addEventListener("keydown", (event) => {
  if (event.key === "Enter") {
    event.preventDefault();
    displayMessage();
  }
});

const actionsButton = document.querySelector(".actions-button");

function deleteMessage() {
  if (messages.length === 0) {
    return;
  }

  messages.pop();
  saveMessages();
  renderMessages();
}

if (actionsButton) {
  actionsButton.addEventListener("click", deleteMessage);
}

renderMessages();
loadUsers();
