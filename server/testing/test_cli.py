#!/usr/bin/env python3
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
    except Exception:
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
        print(f"  ✓ Token gespeichert!")


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


# ─── Menü ────────────────────────────────────────────────────────────────────

MENU = {
    "1": ("Registrieren",          register),
    "2": ("Login",                 login),
    "3": ("Alle User anzeigen",    get_all_users),
    "4": ("User by ID",            get_user),
    "5": ("User löschen",          delete_user),
    "6": ("Meine Chats",           get_my_chats),
    "7": ("Chat erstellen",        create_chat),
    "8": ("Chat by ID",            get_chat),
    "0": ("Beenden",               None),
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
