export function deleteChat(chat, chatList) {
  const targetId = chat.chatId ? chat.chatId : '';
  const index = chatList.findIndex(chat => chat.chatId === targetId);

  if (index >= 0) {
    const removed = chatList[index];
    chatList.splice(index, 1);

    if (removed?.deleteThisChat) {
      removed.deleteThisChat();
    }

    removeChatFromLocalStorage(targetId);
  }
}

function removeChatFromLocalStorage(chatId) {
  const raw = localStorage.getItem("chats");
  const chats = raw ? JSON.parse(raw) : [];

  const updated = chats.filter(chat => chat.chatId !== chatId);

  localStorage.setItem("chats", JSON.stringify(updated));
}