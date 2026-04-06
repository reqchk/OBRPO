#include <stdio.h>
#include <stdlib.h>

struct list {
    int data;
    struct list* next;
};

struct list* new_node(int data) {
    struct list* node = (struct list*)malloc(sizeof(struct list));
    if (!node) return NULL;
    node->data = data;
    node->next = NULL;
    return node;
}

void print_list(struct list* head) {
    while (head != NULL) {
        printf("%d ", head->data);
        head = head->next;
    }
    printf("\n");
}

int get_sum(struct list* head) {
    int sum = 0;
    while (head != NULL) {
        sum += head->data;
        head = head->next;
    }
    return sum;
}

void free_list(struct list* head) {
    while (head != NULL) {
        struct list* temp = head;
        head = head->next;
        free(temp);
    }
}

struct list* find_max(struct list* head) {
    struct list* max = head;
    while (head != NULL) {
        if (head->data > max->data) {  
            max = head;
        }
        head = head->next;
    }
    return max;
}

int main() {
    struct list* list1 = new_node(13);
    list1->next = new_node(25);
    list1->next->next = new_node(3);
    
    struct list* list2 = NULL;
    
    printf("Первый список: ");
    print_list(list1);
    
    printf("Второй список: ");
    print_list(list2);
    
    int sum1 = get_sum(list1);
    int sum2 = get_sum(list2);
    printf("Сумма первого: %d\n", sum1);
    printf("Сумма второго: %d\n", sum2);
    
   
    struct list* max1 = find_max(list1);
    struct list* max2 = find_max(list2); // разыменование NULL
    printf("Максимум в первом: %d\n", max1->data);
    printf("Максимум во втором: %d\n", max2->data);
    
    // утечка памяти
    int* num = (int*)malloc(sizeof(int));
    *num = 42;

    free_list(list1);
    free_list(list2);
    return 0;
}
