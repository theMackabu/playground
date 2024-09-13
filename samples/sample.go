package main

import (
	"fmt"
	"log"
	"sync"
	"time"
)

// Constants
const (
	Pi     = 3.14159
	MaxInt = 1<<63 - 1
)

// Custom type definitions
type ID int

// Struct definition
type Person struct {
	Name string
	Age  int
}

// Method for Person struct
func (p Person) SayHello() string {
	return fmt.Sprintf("Hello, my name is %s and I'm %d years old", p.Name, p.Age)
}

// Interface definition
type Shape interface {
	Area() float64
	Perimeter() float64
}

// Circle struct implementing Shape interface
type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return Pi * c.Radius * c.Radius
}

func (c Circle) Perimeter() float64 {
	return 2 * Pi * c.Radius
}

// Function with multiple return values
func divideAndRemainder(a, b int) (int, int, error) {
	if b == 0 {
		return 0, 0, fmt.Errorf("division by zero")
	}
	return a / b, a % b, nil
}

// Variadic function
func sum(nums ...int) int {
	total := 0
	for _, num := range nums {
		total += num
	}
	return total
}

// Goroutine and channel example
func worker(id int, jobs <-chan int, results chan<- int) {
	for j := range jobs {
		fmt.Printf("Worker %d started job %d\n", id, j)
		time.Sleep(time.Second)
		fmt.Printf("Worker %d finished job %d\n", id, j)
		results <- j * 2
	}
}

// Defer example
func deferExample() {
	defer fmt.Println("This will be printed last")
	fmt.Println("This will be printed first")
}

// Error handling
func mayFail() error {
	return fmt.Errorf("something went wrong")
}

// Custom error type
type MyError struct {
	When time.Time
	What string
}

func (e MyError) Error() string {
	return fmt.Sprintf("at %v, %s", e.When, e.What)
}

// Pointer example
func modifyValue(val *int) {
	*val = 42
}

// Closure example
func adder() func(int) int {
	sum := 0
	return func(x int) int {
		sum += x
		return sum
	}
}

func main() {
	// Basic variable declaration
	var x int = 10
	y := 20.5

	// Printing
	fmt.Printf("x = %d, y = %.2f\n", x, y)

	// Using custom types
	var myID ID = 12345
	fmt.Printf("My ID is %d\n", myID)

	// Using structs
	p := Person{Name: "Alice", Age: 30}
	fmt.Println(p.SayHello())

	// Using interfaces
	var s Shape = Circle{Radius: 5}
	fmt.Printf("Area: %.2f, Perimeter: %.2f\n", s.Area(), s.Perimeter())

	// Multiple return values and error handling
	quotient, remainder, err := divideAndRemainder(10, 3)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("10 ÷ 3 = %d remainder %d\n", quotient, remainder)

	// Variadic function
	fmt.Printf("Sum: %d\n", sum(1, 2, 3, 4, 5))

	// Goroutines and channels
	jobs := make(chan int, 100)
	results := make(chan int, 100)

	for w := 1; w <= 3; w++ {
		go worker(w, jobs, results)
	}

	for j := 1; j <= 5; j++ {
		jobs <- j
	}
	close(jobs)

	for a := 1; a <= 5; a++ {
		<-results
	}

	// Defer
	deferExample()

	// Error handling
	if err := mayFail(); err != nil {
		fmt.Printf("Error: %v\n", err)
	}

	// Custom error
	err = MyError{
		When: time.Now(),
		What: "it failed",
	}
	fmt.Printf("MyError: %v\n", err)

	// Pointers
	val := 10
	modifyValue(&val)
	fmt.Printf("Modified value: %d\n", val)

	// Closures
	add := adder()
	for i := 0; i < 5; i++ {
		fmt.Printf("Running total: %d\n", add(i))
	}

	// HTTP server (uncomment to run)
	// http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
	// 	fmt.Fprintf(w, "Hello, World!")
	// })
	// log.Fatal(http.ListenAndServe(":8080", nil))

	// Mutexes
	var mu sync.Mutex
	count := 0

	increment := func() {
		mu.Lock()
		defer mu.Unlock()
		count++
	}

	var wg sync.WaitGroup
	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			increment()
		}()
	}

	wg.Wait()
	fmt.Printf("Count: %d\n", count)
}
