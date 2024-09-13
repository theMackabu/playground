// sample_cpp.h
#ifndef SAMPLE_CPP_H
#define SAMPLE_CPP_H

#include <string>
#include <vector>

namespace sample {

// Constants
constexpr int MAX_SIZE = 100;

// Class declaration
class Person {
public:
    Person(const std::string& name, int age);
    virtual ~Person() = default;

    std::string getName() const;
    int getAge() const;
    virtual void introduce() const;

    static int getCount();

private:
    std::string name;
    int age;
    static int count;
};

// Function declarations
template<typename T>
T add(T a, T b);

std::vector<int> generateFibonacci(int n);

// Enum class
enum class Color { Red, Green, Blue };

// Inline function
inline bool isEven(int n) { return n % 2 == 0; }

} // namespace sample

#endif // SAMPLE_CPP_H
