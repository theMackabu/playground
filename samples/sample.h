// sample_c.h
#ifndef SAMPLE_C_H
#define SAMPLE_C_H

#include <stddef.h>

// Macro definition
#define MAX_SIZE 100

// Type definitions
typedef struct {
    char name[50];
    int age;
} Person;

typedef enum {
    RED,
    GREEN,
    BLUE
} Color;

// Function declarations
void initPerson(Person* p, const char* name, int age);
void introducePerson(const Person* p);
int add(int a, int b);
void generateFibonacci(int* fib, size_t n);
const char* colorToString(Color c);

// Inline function
static inline int isEven(int n) { return n % 2 == 0; }

#endif // SAMPLE_C_H
