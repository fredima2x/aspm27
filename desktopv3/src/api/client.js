// api/client.js
const BASE_URL = 'http://localhost:3000' // deine Axum-Server-Adresse

export async function apiFetch(endpoint, options = {}) {
  const res = await fetch(`${BASE_URL}${endpoint}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options.headers
    }
  })

  if (!res.ok) {
    const error = await res.json().catch(() => ({ message: 'Unbekannter Fehler' }))
    throw new Error(error.message || `HTTP ${res.status}`)
  }

  return res.json()
}
