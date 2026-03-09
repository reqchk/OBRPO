#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <locale.h>
#include <string.h>
#include "four.h"

// Функция для вычисления суммы нечетных цифр числа
int Sum_nech() {

    int sum = 0;    // Переменная для накопления суммы
    int number;     // Переменная для хранения введенного числа

    printf("Введите число: ");
    scanf("%d", &number);  // Считываем целое число с клавиатуры

    // Обрабатываем каждую цифру числа пока есть цифры (number > 0)
    while (number > 0) {
        // Проверяем, является ли последняя цифра нечетной
        if (((number % 10) % 2) != 0) {
            // Если цифра нечетная, добавляем ее к сумме
            sum = sum + (number % 10);
        }
        number = number / 10;  // Убираем последнюю цифру числа
    }

    return sum;  // Возвращаем полученную сумму
}

void run_four(void) {
    int res = Sum_nech();  // Вызываем функцию и сохраняем результат

    // Выводим результат
    printf("\nСумма нечетных цифр числа: ");
    printf("%d", res);

    int ch;
    while ((ch = getchar()) != '\n' && ch != EOF);
}