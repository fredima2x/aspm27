import { apiFetch } from '../client.js';

export class Chat {
  constructor(chatId, chatName, chatDesc) {
    this.chatId = chatId;
    this.chatName = chatName;
    this.chatDesc = chatDesc;
  }

  async getThisChat() {
    return await apiFetch(`/chats/${this.chatId}`, {
      method: 'GET'
    });
  }

  async deleteThisChat() {
    return await apiFetch(`/chats/${this.chatId}`, {
      method: 'DELETE'
    });
  }
}