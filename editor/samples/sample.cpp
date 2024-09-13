// sample_cpp.cpp
#include "sample_cpp.h"
#include <iostream>
#include <stdexcept>
#include <memory>
#include <algorithm>

namespace sample {

int Person::count = 0;

Person::Person(const std::string& name, int age) : name(name), age(age) {
    ++count;
}

std::string Person::getName() const { return name; }
int Person::getAge() const { return age; }

void Person::introduce() const {
    std::cout << "Hi, I'm " << name << " and I'm " << age << " years old." << std::endl;
}

int Person::getCount() { return count; }

template<typename T>
T add(T a, T b) {
    return a + b;
}

std::vector<int> generateFibonacci(int n) {
    if (n < 0) throw std::invalid_argument("n must be non-negative");
    std::vector<int> fib(n);
    for (int i = 0; i < n; ++i) {
        if (i < 2) fib[i] = i;
        else fib[i] = fib[i-1] + fib[i-2];
    }
    return fib;
}

} // namespace sample

// Main function
int main() {
    using namespace sample;

    // Using the Person class
    auto person = std::make_unique<Person>("Alice", 30);
    person->introduce();

    // Using the add function template
    std::cout << "5 + 3 = " << add(5, 3) << std::endl;
    std::cout << "3.14 + 2.86 = " << add(3.14, 2.86) << std::endl;

    // Using the Fibonacci function
    auto fib = generateFibonacci(10);
    std::cout << "First 10 Fibonacci numbers: ";
    for (int n : fib) std::cout << n << " ";
    std::cout << std::endl;

    // Using the Color enum class
    Color c = Color::Blue;
    switch(c) {
        case Color::Red: std::cout << "Red"; break;
        case Color::Green: std::cout << "Green"; break;
        case Color::Blue: std::cout << "Blue"; break;
    }
    std::cout << std::endl;

    // Using lambda function with algorithm
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    std::for_each(numbers.begin(), numbers.end(), [](int n) {
        std::cout << (isEven(n) ? "Even" : "Odd") << " ";
    });
    std::cout << std::endl;

    return 0;
}
