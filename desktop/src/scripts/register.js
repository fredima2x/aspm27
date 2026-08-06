export async function register(username, password) {
  
  try {
    const response = await fetch(`${baseURL}/users`, {
      method: 'POST',
      headers: header(token),
      body: JSON.stringify({ username, password })
    });
    return await response.json();

  } catch(error) {
    console.error(`Fetch error: ${error}`);
  }
}