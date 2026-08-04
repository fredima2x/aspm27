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




token = (await login("fredima2x", "12341234")).token_string;
console.log(chats)
