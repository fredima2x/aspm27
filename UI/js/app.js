async function loadUsers() {
  try {
    console.log("Start");

    const response = await fetch("http://127.0.0.1:3000/users");

    console.log("Status:", response.status);

    const users = await response.json();

    console.log("Users:", users);

    const container = document.querySelector(".contacts");

    if (!container) {
      console.error("Contacts container not found");
      return;
    }

    users.forEach((user) => {
      const contactButton = document.createElement("button");

      contactButton.className = "contact";
      contactButton.type = "button";

      contactButton.innerHTML = `
                <div class="contact-picture-cont">
                    <img src="imgs/kreise.pdf" class="contact-picture" />
                </div>
                <div class="contact-info">
                    <p class="contact-heading">
                        ${user.username}
                    </p>

                    <p class="contact-id">
                        ID: ${user.id}
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
