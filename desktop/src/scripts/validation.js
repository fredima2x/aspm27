import { bannedPasswords, bannedUsernames } from './config.js';

export function getValidationErrors(password, username) {
  const errors = {
    password: null,
    username: null,
  };

  if (password.length < 8) {
    errors.password = 'Password must be at least 8 characters long!';
  } else if (password.length > 64) {
    errors.password = "Password can't be longer than 64 characters!";
  } else if (bannedPasswords.includes(password)) {
    errors.password = "Don't even try...";
  }

  if (username.length < 3) {
    errors.username = 'Username must be at least 3 characters long!';
  } else if (username.length > 24) {
    errors.username = "Username can't be longer than 24 characters!";
  } else if (bannedUsernames.includes(username)) {
    errors.username = "I'm disappointed...";
  }

  return errors;
}

export function applyValidationMessage(element, message) {
  if (!element) {
    return;
  }

  if (message) {
    element.innerText = message;
    element.classList.add('password-warn-text');
  } else {
    element.innerText = '';
    element.classList.remove('password-warn-text');
  }
}
