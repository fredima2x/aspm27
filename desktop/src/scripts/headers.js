export function header(token) {
  return {
    "Authorization": `Bearer ${token}`,
    "Content-Type": "application/json",
  };
}