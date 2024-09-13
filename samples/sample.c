// sample_c.c
#include "sample_c.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

void initPerson(Person* p, const char* name, int age) {
    strncpy(p->name, name, sizeof(p->name) - 1);
    p->name[sizeof(p->name) - 1] = '\0';  // Ensure null-termination
    p->age = age;
}

void introducePerson(const Person* p) {
    printf("Hi, I'm %s and I'm %d years old.\n", p->name, p->age);
}

int add(int a, int b) {
    return a + b;
}

void generateFibonacci(int* fib, size_t n) {
    if (fib == NULL) return;
    
    for (size_t i = 0; i < n; ++i) {
        if (i < 2) fib[i] = i;
        else fib[i] = fib[i-1] + fib[i-2];
    }
}

const char* colorToString(Color c) {
    switch(c) {
        case RED: return "Red";
        case GREEN: return "Green";
        case BLUE: return "Blue";
        default: return "Unknown";
    }
}

int main() {
    // Using the Person struct
    Person person;
    initPerson(&person, "Bob", 25);
    introducePerson(&person);

    // Using the add function
    printf("5 + 3 = %d\n", add(5, 3));

    // Using the Fibonacci function
    int fib[10];
    generateFibonacci(fib, 10);
    printf("First 10 Fibonacci numbers: ");
    for (int i = 0; i < 10; ++i) {
        printf("%d ", fib[i]);
    }
    printf("\n");

    // Using the Color enum
    Color c = BLUE;
    printf("Color: %s\n", colorToString(c));

    // Using function pointers
    int (*funcPtr)(int, int) = &add;
    printf("3 + 4 = %d (using function pointer)\n", funcPtr(3, 4));

    // Using the inline function
    for (int i = 0; i < 5; ++i) {
        printf("%d is %s\n", i, isEven(i) ? "even" : "odd");
    }

    // Dynamic memory allocation
    int* dynamicArray = (int*)malloc(5 * sizeof(int));
    if (dynamicArray != NULL) {
        for (int i = 0; i < 5; ++i) {
            dynamicArray[i] = i * 2;
        }
        for (int i = 0; i < 5; ++i) {
            printf("%d ", dynamicArray[i]);
        }
        printf("\n");
        free(dynamicArray);
    }

    return 0;
}
