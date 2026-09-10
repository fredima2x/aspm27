import os
import sys
import shutil
import subprocess


PROJECT_DIR = "desktopv3"


def run_command(command, cwd=None):
    try:
        subprocess.run(command, cwd=cwd, check=True)
    except FileNotFoundError:
        print(f"Fehler: '{command[0]}' wurde nicht gefunden.")
        sys.exit(1)
    except subprocess.CalledProcessError as e:
        print(f"Fehler beim Ausführen von {' '.join(command)}")
        print(f"Exit-Code: {e.returncode}")
        sys.exit(e.returncode)


def delete_contents(directory, except_names=None):
    if except_names is None:
        except_names = set()

    for name in os.listdir(directory):
        if name in except_names:
            continue

        path = os.path.join(directory, name)

        if os.path.isdir(path) and not os.path.islink(path):
            shutil.rmtree(path)
        else:
            os.remove(path)


def move_contents(source, destination):
    for name in os.listdir(source):
        source_path = os.path.join(source, name)
        destination_path = os.path.join(destination, name)

        shutil.move(source_path, destination_path)


def main():
    if not os.path.isdir(PROJECT_DIR):
        print(f"Fehler: '{PROJECT_DIR}' wurde nicht gefunden.")
        print("Bitte führe dieses Script im Repository-Root aus.")
        sys.exit(1)

    project_path = os.path.abspath(PROJECT_DIR)
    root_path = os.getcwd()
    dist_path = os.path.join(project_path, "dist")

    print("Prüfe npm...")
    run_command(["npm", "--version"])

    print()
    print("Dieses Script wird das Repository umstrukturieren.")
    print("Dabei werden Dateien im Repository-Root gelöscht.")
    print()
    print("Nach dem Build wird der Inhalt von:")
    print(f"  {dist_path}")
    print()
    print("direkt in den Repository-Root verschoben.")
    print()

    answer = input("Fortfahren? [y/N] ")

    if answer.lower() != "y":
        print("Abgebrochen.")
        return

    print("\n>>> npm install")
    run_command(["npm", "install"], cwd=project_path)

    print("\n>>> npm run build")
    run_command(["npm", "run", "build"], cwd=project_path)

    if not os.path.isdir(dist_path):
        print()
        print("Fehler: Der Build-Ordner 'dist' wurde nicht gefunden.")
        sys.exit(1)

    # Sicherheitsabfrage
    print()
    print("Build erfolgreich.")
    print("Jetzt wird der Repository-Root geleert.")

    answer = input("Wirklich fortfahren? [y/N] ")

    if answer.lower() != "y":
        print("Abgebrochen.")
        return

    temp_path = os.path.join(root_path, ".deploy_tmp")

    if os.path.exists(temp_path):
        shutil.rmtree(temp_path)

    shutil.move(dist_path, temp_path)

    try:
        delete_contents(
            root_path,
            except_names={".git", ".deploy_tmp"}
        )

        move_contents(temp_path, root_path)

    finally:
        if os.path.exists(temp_path):
            shutil.rmtree(temp_path)

    print()
    print("================================")
    print("Deployment erfolgreich!")
    print("================================")


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\nAbgebrochen.")
        sys.exit(130)
