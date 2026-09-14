import { apiFetch } from '../client.js';

export class Chat {
  constructor(chatId, chatName, chatDesc) {
    this.chatId = chatId;
    this.chatName = chatName;
    this.chatDesc = chatDesc;
  }
}