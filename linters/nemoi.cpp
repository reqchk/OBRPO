#include <iostream>
#include <cstring>

using namespace std;

struct Node {
        int value;
        Node* next;
    };
    
int main() {
    Node* first = new Node;
    first->value = 343434344;
    first->next = nullptr;
    
    Node* second = new Node;
    second->value = 129939393;
    second->next = nullptr;

    Node* third = new Node;
    third ->value = 2334;
    second->next = nullptr;

    first->next = second;

    second->next = third;

    third ->next = nullptr;

    cout << first->value << endl;
    cout << second->value << endl;

    cout << endl;
    
    // Ошибка1 переполнение буфера 
    char numberName[10];
    strcpy(numberName, "сто двадцать три миллиона");  // можно использовать string
    cout << "Название числа: " << numberName << endl;
    
   // Утечка памяти 
    int* numbers = new int[1000];  // numbers указывает на начало выделенного массива
    for (int i = 0; i < 1000; i++) {
        numbers[i] = i * 2; 
    }
    cout << "Создан массив из 1000 чисел" << endl;
    
    // Ошибка3 выход за границы массива 
    int primes[5] = {2, 3, 5, 7, 11};
    cout << endl << "Простые числа:" << endl;
    for (int i = 0; i <= 5; i++) {  // ошибка: должно быть i < 5
        cout << "  " << primes[i] << endl;
    }
    // Ошибка4 неинициализированная переменная
    // Программа вычисляет сумму чисел
    int sum;
    for (int i = 0; i < 1000; i++) {
        sum += numbers[i];  // sum не инициализирована — начинается с мусора
    }
    cout << endl << "Сумма всех чисел: " << sum << endl;

    // Ошибка5 переполнения int
    int product = 1;
    for (int i = 1; i <= 10; i++) {  
        product = product * numbers[i];
    }
    cout << "Произведение первых 10 чисел: " << product << endl;
   
    int count = 0;
    int average = sum / count;  // деление на ноль!
    cout << "Среднее арифметическое: " << average << endl;
    // не освобождаем память 

    return 0;
}
