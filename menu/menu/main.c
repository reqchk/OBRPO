#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>
#include <locale.h>
#include "first.h"
#include "second.h"
#include "third.h"

int main() {
    setlocale(LC_ALL, "ru");
    int choice;

    do {
        system("cls"); 

        printf("ГЛАВНОЕ МЕНЮ\n\n");
        printf("1. Восьмеричный калькулятор\n");
        printf("2. Сортировка трех чисел\n");
        printf("3. Сумма четных цифр числа\n");
        printf("0. Выход\n\n");
        printf("Ваш выбор: ");

        scanf("%d", &choice);

        // ОЧИЩАЕМ БУФЕР ПОСЛЕ SCANF
        int ch;
        while ((ch = getchar()) != '\n' && ch != EOF);

        switch (choice) {
        case 1:
            run_first();
            break;
        case 2:
            run_second();
            break;
        case 3:
            run_third();
            break;
        case 0:
            printf("\nПрограмма завершена.\n");
            break;
        default:
            printf("\nОшибка: такого пункта нет!\n");
        }

        if (choice != 0) {
            printf("\nНажмите Enter, чтобы продолжить...");
            // Убираем while(getchar() != '\n'); - буфер уже чист!
            getchar(); // просто ждем Enter
        }

    } while (choice != 0);

    return 0;
}