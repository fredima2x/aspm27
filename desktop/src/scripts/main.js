////Libs
import { baseURL } from "./config.js";
//// Configuration

const messageInput = document.querySelector(".message-text-bar");
const sendMessageButton = document.querySelector(".send-button");
const deleteMessageButton = document.querySelector(".delete-button")

const yourProfileDisplayName = document.querySelector(".your-profile-heading")
const yourProfileName = document.querySelector(".your-profile-number");
const yourProfileID = document.querySelector(".your-profile-name");

const openedContact = document.querySelector(".opened-contact");
const openedContactDisplayName = document.querySelector(".opened-contact-heading");
const openedContactName = document.querySelector(".opened-contact-number");
const openedContactID = document.querySelector(".opened-contact-name");


const chatArea = document.querySelector(".chat-area");
const storageKey = "messages";



//// Runtime Globals
let displayed_messages = null;
let selected_chat = null;
let profile = null;

let auth_token = null;
console.log("Trying to load Auth Token...");
if (localStorage.getItem("auth_token") !== null) {
  auth_token = localStorage.getItem("auth_token");
  console.log("Token loaded:", auth_token);
} else {
  console.error("No auth token available!");
  window.location.href = "/login.html";
}

// Depreciated
// let messages = JSON.parse(localStorage.getItem(storageKey) || "[]");
// function saveMessages() {
//   localStorage.setItem(storageKey, JSON.stringify(messages));
// }


//// API Communcation Functions
// API Header Helper

export function header() {
  return {
    "Authorization": `Bearer ${auth_token}`,
    "Content-Type": "application/json",
  };
}

async function send_message(chat_id, content) {
  try {
    const response = await fetch(`${baseURL}/chats/${chat_id}/messages`, {
      method: "POST",
      body: JSON.stringify({ content }),
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
    const response = await fetch(`${baseURL}/chats/${chat_id}/get_messages`, {
      method: "POST",
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

        for (const chat of chats) {
            const contactButton = document.createElement("button");

            contactButton.className = "contact";
            contactButton.type = "button";
            contactButton.dataset.chatId = chat.id;

            // Markiere ausgewählten Chat
            if (chat.id === selected_chat) {
                contactButton.classList.add("selected");
            }

            contactButton.innerHTML = `
                <div class="contact-picture-cont">
                    <!--img src="https://picsum.photos/300/200" class="contact-picture">-->
                </div>

                <div class="contact-info">
                    <p class="contact-heading">${chat.chat_name}</p>
                    <p class="contact-id">${chat.chat_desc}</p>
                </div>
            `;

            contactButton.addEventListener("click", async () => {
                // Bereits ausgewählt -> nichts tun
              if (selected_chat === chat.id) return;

              selected_chat = chat.id;

              openedContactDisplayName.textContent = chat.chat_name;
              openedContactName.textContent = chat.chat_desc;
              openedContactID.textContent = `#${chat.id}`;

              await updateChats();
              await updateMessages();
            });

            container.appendChild(contactButton);
        }


        if (selected_chat === null) {
          openedContact.style.display = "none";
        } else {
          openedContact.style.display = "block";
        }


        // "+"-Button
        const addContactContainer = document.createElement("div");
        addContactContainer.className = "add-contact-button-cont";

        addContactContainer.innerHTML = `
            <button class="add-contact-button">+</button>
        `;

        container.appendChild(addContactContainer);

    } catch (error) {
        console.error("Fetch error:", error);
    }
}

async function load_messages() {
  if (selected_chat === null) {return}
  displayed_messages = await get_chat_messages(
    selected_chat,
    100,
    0
  );
  console.log("Loaded messages", displayed_messages);
}

async function updateMessages() {
  await load_messages();

  chatArea.innerHTML = "";

  if (displayed_messages === null) {
    console.debug("Displayed Messages null return");
    return;
  }

  // Alle benötigten User einmalig und parallel laden
  const uniqueOwnerIds = [...new Set(displayed_messages.map(m => m.owner_id))];
  const userEntries = await Promise.all(
    uniqueOwnerIds.map(async (id) => [id, await getUser(id)])
  );
  const users = Object.fromEntries(userEntries); // { ownerId: userObj }

  displayed_messages.forEach((entry) => {
    console.debug("Printing Message", entry);

    function get_display_name(owner) {
      if (owner.id === profile.user.id) return "You";
      return owner.display_name;
    }

    function get_class(owner) {
      if (owner.id === profile.user.id) return "message-me";
      return "message-them";
    }

    const owner = users[entry.owner_id];

    if (owner === null || owner === undefined) {
      console.error("Owner is null/undefined, skipping message", entry);
      return;
    }

    const messageWrapper = document.createElement("div");
    messageWrapper.className = get_class(owner);
    messageWrapper.innerHTML = `
      <p class="sender">${get_display_name(owner)}</p>
      <div class="message-cont">
        <p class="message"></p>
      </div>
    `;

    messageWrapper.querySelector(".message").textContent = entry.content;
    chatArea.appendChild(messageWrapper);
  });

  chatArea.scrollTop = chatArea.scrollHeight;
}

async function update_profile() {
  profile = await get_profile();
  console.log("profile:", profile);
  if (!profile) {return}

  yourProfileDisplayName.textContent = profile.user.display_name;
  yourProfileName.textContent = `@${profile.user.username}`;
  yourProfileID.textContent = `#${profile.user.id}`;
}


// Handler
async function handler_sendButton() {
  const content = messageInput.value;
  if (!content) { return }
  messageInput.value = "";
  await send_message(selected_chat, content);
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
    update_profile(),
  ]);
}

async function setup() {

  sendMessageButton.addEventListener("click", handler_sendButton);
  messageInput.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      handler_sendButton();
    }
  });
  deleteMessageButton.addEventListener("click", handler_deleteButton);

  // Initial load, so the UI isn't empty for the first 5 seconds
  await update_ui().catch(console.error);

  // Setting Updates
  setInterval(() => { update_ui().catch(console.error); }, 60000);
}

setup();
