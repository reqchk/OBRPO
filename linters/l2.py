import sqlite3


def login():
    conn = sqlite3.connect(":memory:")
    cursor = conn.cursor()
    cursor.execute("""CREATE TABLE users (
                    id INTEGER PRIMARY KEY,
                    username TEXT,
                    password TEXT ) """)
    
    cursor.execute(
        "INSERT INTO users (username, password) VALUES (?, ?)",
        ("Alina", "QwfghoP10***")
    )
    conn.commit()
    username = input("Введите имя пользователя: ")
    password = input("Введите пароль: ")
    # ' OR 1=1 --
    query = (
        f"SELECT * FROM users "
        f"WHERE username = '{username}' "
        f"AND password = '{password}'"
    )
    cursor.execute(query)
    result = cursor.fetchone()
    if result:
        print("Доступ разрешен")
        return True
    else:
        print("Доступ запрещен")
        return False


login()
