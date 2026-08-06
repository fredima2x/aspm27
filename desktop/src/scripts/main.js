////Libs


//// Configuration
const baseURL = "http://127.0.0.1:3000";
const messageInput = document.querySelector(".message-text-bar");
const sendMessageButton = document.querySelector(".send-button");
const deleteMessageButton = document.querySelector(".delete-button")
const chatArea = document.querySelector(".chat-area");
const storageKey = "messages";

//// Runtime Globals
let displayed_messages = null;
let auth_token = null;

// Depreciated
// let messages = JSON.parse(localStorage.getItem(storageKey) || "[]");
// function saveMessages() {
//   localStorage.setItem(storageKey, JSON.stringify(messages));
// }


//// API Communcation Functions
// API Header Helper

function header() {
  return {
    "Authorization": `Bearer ${auth_token}`,
    "Content-Type": "application/json",
  };
}

async function get_profile() {
  try {
    const response = await fetch(`${baseURL}/profile`, {
      method: "GET",
      headers: header(),
    });
    if (!response.ok) {
      console.error(await response.text());
      return null;
    }
    return await response.json();
  } catch (error) {
    console.error("Fetch error:", error);
  }
}

async function get_chat_messages(chat_id, limit, offset) {
  try {
    const response = await fetch(`${baseURL}/chats/${chat_id}/messages`, {
      method: "GET",
      body: JSON.stringify({ limit, offset }),
      headers: header(),
    })
    if (!response.ok) {
      console.error(await response.text());
      return null;
    }
    return await response.json();
  } catch(error) {
    console.error("Fetch error:", error);
  }
}

async function register(username, password) {

  try {
    const response = await fetch(`${baseURL}/users`, {
      method: 'POST',
      headers: header(),
      body: JSON.stringify({ username, password })
    });
    if (!response.ok) {
      console.error(await response.text());
      return null;
    }
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
    const data = await response.json();
    if (!response.ok) {
      console.error("Login failed:", data);
      return null;
    }
    return data;

  } catch(error) {
    console.error("Fetch error:", error);
    return null;
  }
}

async function get_chats() {
  try {
    const response = await fetch(`${baseURL}/chats`, {
      headers: header(auth_token),
    });

    if (!response.ok) {
      console.error(await response.text());
      return [];
    }

    return await response.json();
  } catch (error) {
    console.error("Fetch error:", error);
    return [];
  }
}


async function getUser(id) {
  try {
    const response = await fetch(`${baseURL}/users/${id}`, {
      headers: header(),
    });
    if (!response.ok) {
      console.error(await response.text());
      return null;
    }
    return await response.json();
  } catch (error) {
    console.error("Fetch error:", error);
    return null;
  }
}

//// UI functions
async function updateChats() {
  try {
  const chats = await get_chats();

  if (!Array.isArray(chats)) {
    console.error("Chats is not an array:", chats);
    return;
  }
  const container = document.querySelector(".contacts");
  if (!container) return;

  container.innerHTML = "";

    chats.forEach((chat) => {
      const contactButton = document.createElement("button");

      contactButton.className = "contact";
      contactButton.type = "button";

      contactButton.innerHTML = `
                <div class="contact-picture-cont">
                    <img src="https://picsum.photos/300/200" class="contact-picture" />
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
    console.error(`Fetch error: ${error}`);
  }
}

async function updateMessages() {
  chatArea.innerHTML = "";

  if (!displayed_messages) {return}

  displayed_messages.forEach((entry) => {
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

// Handler
async function handler_sendButton() {
  // TODO API-Call
  await update_ui();
}
async function handler_deleteButton() {
  // TODO API-Call
  await update_ui();
}

//// System
async function update_ui() {
  await Promise.all([
    updateChats(),
    updateMessages(),
  ]);
}

async function setup() {

  // Depreciated: Initial Login Attempt
  const loginResponse = (await login("fredima2x", "12341234"));
  if (!loginResponse || !loginResponse.token_string) {
    console.error("Login response invalid", loginResponse);
    return;
  }
  auth_token = loginResponse.token_string;

  sendMessageButton.addEventListener("click", handler_sendButton);
  deleteMessageButton.addEventListener("click", handler_deleteButton);

  // Setting Updates
  setInterval(() => { updateChats().catch(console.error); }, 5000);
}

setup();
