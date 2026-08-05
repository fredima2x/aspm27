const baseURL = "http://127.0.0.1:3000";
let token = null;

function header() {
  return {
    "Authorization": `Bearer ${token}`,
    "Content-Type": "application/json",
  };
}

async function register(username, password) {
  
  try {
    const response = await fetch(`${baseURL}/users`, {
      method: 'POST',
      headers: header(),
      body: JSON.stringify({ username, password })
    });
    return await response.json();

  } catch(error) {
    console.error("Fetch error:", error);
  }
}

async function login(username, password) {

  try {
    const response = await fetch(`${baseURL}/login`, {
      method: "POST",
      headers: header(),
      body: JSON.stringify({ username, password }),
    });
    return await response.json();

  } catch(error) {
    console.error("Fetch error:", error);
  }
}

async function get_chats() {

  try {
    const response = await fetch(`${base_url}/chats`, {
      headers: header(),
    });
    return await response.json();

  } catch(error) {
    console.error("Fetch error:", error);
  }
}


async function getUser(id) {
  try {
    response = await fetch(`${BASE_URL}/users/${id}`, {
      headers: header()
    });
    return await response.json();

  } catch(error) {
    console.error("Fetch error:", error);
  }
}

async function loadChats() {
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
    console.error("Fetch error:", error);
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

function deleteLastMessage() {
  if (messages.length === 0) {
    return;
  }

  messages.pop();
  saveMessages();
  renderMessages();
}

renderMessages();
loadChats();
