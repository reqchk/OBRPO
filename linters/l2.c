
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <locale.h>

int main() {
    setlocale(LC_ALL, "ru_RU.UTF-8");
    srand(time(NULL));
    int secret = rand() % 100 + 1;
    int yourNumber;
    int attempts = 0;
    int* history = NULL;  // массив для хранения попыток
    int maxAttempts = 10;
    printf("Угадай (1-100)\n\n");
    printf("У тебя %d попыток\n\n", maxAttempts);
    while (attempts < maxAttempts) {
        printf("Попытка %d: ", attempts + 1);
        scanf("%d", &yourNumber);
        history = (int*)realloc(history, (attempts + 1) * sizeof(int));
        // int* newHistory = (int*)realloc(history, (attempts + 1) * sizeof(int));
        // if (newHistory == NULL) {
        //     printf("Ошибка выделения памяти\n");
        //     free(history);
        //     return 1;
        // }
        // history = newHistory;
        history[attempts] = yourNumber;
        attempts++;
        if (yourNumber == secret) {
            printf("Поздравляю! Ты угадал число за %d попыток\n", attempts);
            printf("\nТвои попытки: ");
            for (int i = 0; i < attempts; i++) {
                printf("%d ", history[i]);
            }
            printf("\n");
            break;
        } else if (yourNumber < secret) {
            printf("Загаданное число больше введенного\n\n");
        } else {
            printf("Загаданное число меньше введенного\n\n");
        }
    }
    if (yourNumber != secret) {
        printf("\nТы проиграл! Загаданное число: %d\n", secret);
    }
    return 0;
}
