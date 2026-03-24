import sqlite3


def login():
    # Создаем тестовую БД
    conn = sqlite3.connect(":memory:")
    cursor = conn.cursor()
    cursor.execute("""
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            username TEXT,
            password TEXT
        )
    """)
    # Добавляем тестового пользователя
    cursor.execute(
        "INSERT INTO users (username, password) VALUES (?, ?)",
        ("alina", "parol")
    )
    conn.commit()
    username = input("Введите имя пользователя: ")
    password = input("Введите пароль: ")
    query = (
        f"SELECT * FROM users "
        f"WHERE username = '{username}' "
        f"AND password = '{password}'"
    )
    cursor.execute(query)
    # query = ("""SELECT * FROM users
    #          WHERE username = ?
    #          AND password = ?"""
    #          )
    # cursor.execute(query, (username, password))
    result = cursor.fetchone()
    if result:
        print("Доступ разрешен")
        return True
    else:
        print("Доступ запрещен")
        return False


login()
