import requests
import json

BASE_URL = "http://127.0.0.1:3000"
token = None  # wird nach Login gesetzt


def header():
    return {"Authorization": f"Bearer {token}"} if token else {}


def print_response(r):
    print(f"  Status: {r.status_code}")
    try:
        print(f"  Body:   {json.dumps(r.json(), indent=2, ensure_ascii=False)}")
    except:
        print(f"  Body:   {r.text}")

# ─── User ────────────────────────────────────────────────────────────────────

def register():
    username = input("  Username: ")
    password = input("  Passwort: ")
    r = requests.post(f"{BASE_URL}/users", json={"username": username, "password": password})
    print_response(r)


def login():
    global token
    username = input("  Username: ")
    password = input("  Passwort: ")
    r = requests.post(f"{BASE_URL}/login", json={"username": username, "password": password})
    print_response(r)
    if r.status_code == 200:
        token = r.json().get("token_string")
        print("  ✓ Token gespeichert!")


def get_all_users():
    r = requests.get(f"{BASE_URL}/users")
    print_response(r)


def get_user():
    id = input("  User ID: ")
    r = requests.get(f"{BASE_URL}/users/{id}")
    print_response(r)


def delete_user():
    id = input("  User ID: ")
    r = requests.delete(f"{BASE_URL}/users/{id}")
    print_response(r)


# ─── Chat ────────────────────────────────────────────────────────────────────

def get_my_chats():
    r = requests.get(f"{BASE_URL}/chats", headers=header())
    print_response(r)


def create_chat():
    name = input("  Chat Name: ")
    r = requests.post(f"{BASE_URL}/chats", json={"chat_name": name}, headers=header())
    print_response(r)


def get_chat():
    id = input("  Chat ID: ")
    r = requests.get(f"{BASE_URL}/chats/{id}")
    print_response(r)

def delete_chat():
    id = input("  Chat ID: ")
    r = requests.delete(f"{BASE_URL}/chats/{id}", headers=header())
    print_response(r)

def get_chat_members():
    id = input("  Chat ID: ")
    r = requests.get(f"{BASE_URL}/chats/{id}/users", headers=header())
    print_response(r)

def is_user_in_chat():
    chat_id = input("  Chat ID: ")
    user_id = input("  user ID: ")
    r = requests.get(f"{BASE_URL}/chats/{chat_id}/users/{user_id}", headers=header())
    print_response(r)

def add_chat_member():
    chat_id = input("  Chat ID: ")
    user_id = input("  user ID: ")
    r = requests.post(f"{BASE_URL}/chats/{chat_id}/users/{user_id}", headers=header())
    print_response(r)

def remove_chat_member():
    chat_id = input("  Chat ID: ")
    user_id = input("  user ID: ")
    r = requests.delete(f"{BASE_URL}/chats/{chat_id}/users/{user_id}", headers=header())
    print_response(r)

def send_message():
    chat_id = input("   Chat ID: ")
    content = input("   Message Content: ")
    r = requests.post(f"{BASE_URL}/chats/{chat_id}/messages", json={"content": content},headers=header())
    print_response(r)

def delete_message():
    message_id = input("   Message ID: ")
    r = requests.delete(f"{BASE_URL}/messages/{message_id}", headers=header())
    print_response(r)

def get_message():
    message_id = input("   Message ID: ")
    r = requests.get(f"{BASE_URL}/messages/{message_id}", headers=header())
    print_response(r)

def get_chat_messenges():
    chat_id = input("   Chat ID: ")
    limit = int(input("   Limit: "))
    offset = int(input("   Offset: "))
    r = requests.get(f"{BASE_URL}/chats/{chat_id}/messages", json={"limit": limit, "offset": offset},headers=header())
    print_response(r)

def update_user_profile():
    username = input("   Username: ")
    display_name = input("   Displayname: ")
    password = input("   Password: ")
    r = requests.put(
        f"{BASE_URL}/profile",
        json={"user": {
            "username": username,
            "password": password,
            "display_name": display_name,
        }},
        headers=header(),
    )
    print_response(r)

def update_chat():
    chat_id = input("   Chat ID: ")
    chat_name = input("   Chat Name: ")
    chat_desc = input("   Chat Description: ")
    r = requests.put(
        f"{BASE_URL}/chats/{chat_id}",
        json={"chat_name": chat_name, "chat_desc": chat_desc},
        headers=header(),
    )
    print_response(r)

def get_user_profile():
    r = requests.get(f"{BASE_URL}/profile", headers=header())
    print_response(r)

# ─── Menü ────────────────────────────────────────────────────────────────────

MENU = {
    "1": ("Registrieren", register),
    "2": ("Login", login),
    "3": ("Alle User anzeigen", get_all_users),
    "4": ("User by ID", get_user),
    "5": ("User löschen", delete_user),
    "6": ("Meine Chats", get_my_chats),
    "7": ("Chat erstellen", create_chat),
    "8": ("Chat by ID", get_chat),
    "9": ("Chat Löschen", delete_chat),
    "10": ("Alle Chat Member anzeigen", get_chat_members),
    "11": ("Chat Member hinzufügen", add_chat_member),
    "12": ("Chat Member entfernen", remove_chat_member),
    "13": ("Ist User in Chat?", is_user_in_chat),
    "14": ("Send Message", send_message),
    "15": ("Delete Message", delete_message),
    "16": ("Get Message", get_message),
    "17": ("Get Chat Messenges", get_chat_messenges),
    "18": ("Update User Profile", update_user_profile),
    "19": ("Get User Profile", get_user_profile),
    "20": ("Update Chat", update_chat),
    "0": ("Beenden", None),
}

def main():
    print("╔══════════════════════════════╗")
    print("║     Backend Test CLI         ║")
    print("╚══════════════════════════════╝")
    while True:
        print(f"\n  Token: {'✓ gesetzt' if token else '✗ nicht gesetzt'}")
        print()
        for key, (label, _) in MENU.items():
            print(f"  [{key}] {label}")
        print()
        choice = input("  Auswahl: ").strip()
        if choice not in MENU:
            print("  Ungültige Auswahl.")
            continue
        label, fn = MENU[choice]
        if fn is None:
            print("  Tschüss!")
            break
        print(f"\n── {label} ──")
        try:
            fn()
        except requests.exceptions.ConnectionError:
            print("  ✗ Server nicht erreichbar! Läuft dein Backend?")


if __name__ == "__main__":
    main()
