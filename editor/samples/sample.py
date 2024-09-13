#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys
import os
from typing import List, Dict, Tuple, Optional
from dataclasses import dataclass
from abc import ABC, abstractmethod

# Constants
PI = 3.14159
MAX_VALUE = 100

# Variables
x: int = 10
y: float = 20.5
name: str = "Python"

# Lists
numbers: List[int] = [1, 2, 3, 4, 5]
mixed: List = [1, "two", 3.0, [4, 5]]

# Tuples
coordinates: Tuple[int, int] = (10, 20)

# Dictionaries
person: Dict[str, Union[str, int]] = {"name": "Alice", "age": 30}

# Sets
unique_numbers: set = {1, 2, 3, 4, 5}

# Functions
def greet(name: str) -> str:
	 """
	 A simple greeting function.
	 
	 Args:
		  name (str): The name to greet.
	 
	 Returns:
		  str: The greeting message.
	 """
	 return f"Hello, {name}!"

# Lambda functions
square = lambda x: x ** 2

# Classes
class Animal:
	 def __init__(self, name: str):
		  self.name = name
	 
	 def speak(self) -> str:
		  pass

class Dog(Animal):
	 def speak(self) -> str:
		  return f"{self.name} says Woof!"

# Abstract class
class Shape(ABC):
	 @abstractmethod
	 def area(self) -> float:
		  pass

# Dataclass
@dataclass
class Point:
	 x: float
	 y: float

# Exception handling
try:
	 result = 10 / 0
except ZeroDivisionError as e:
	 print(f"Error: {e}")
finally:
	 print("This always executes")

# Context managers
with open("example.txt", "w") as f:
	 f.write("Hello, Python!")

# Generators
def countdown(n: int):
	 while n > 0:
		  yield n
		  n -= 1

# List comprehensions
squares = [x**2 for x in range(10)]

# Dictionary comprehensions
square_dict = {x: x**2 for x in range(5)}

# Decorators
def timer(func):
	 def wrapper(*args, **kwargs):
		  import time
		  start = time.time()
		  result = func(*args, **kwargs)
		  end = time.time()
		  print(f"{func.__name__} took {end - start} seconds")
		  return result
	 return wrapper

@timer
def slow_function():
	 import time
	 time.sleep(1)

# Type hints
def add_numbers(a: int, b: int) -> int:
	 return a + b

# Optional type
def print_if_string(value: Optional[str]) -> None:
	 if value is not None:
		  print(value)

# Async functions
import asyncio

async def fetch_data(url: str) -> str:
	 print(f"Fetching data from {url}")
	 await asyncio.sleep(1)  # Simulate I/O operation
	 return f"Data from {url}"

async def main():
	 urls = ["http://example.com", "http://example.org", "http://example.net"]
	 tasks = [fetch_data(url) for url in urls]
	 results = await asyncio.gather(*tasks)
	 for result in results:
		  print(result)

# F-strings
name = "Alice"
age = 30
print(f"{name} is {age} years old")

# Walrus operator (Python 3.8+)
if (n := len(numbers)) > 5:
	 print(f"List is long ({n} elements)")

# Match statement (Python 3.10+)
def check_status(status):
	 match status:
		  case 400:
				return "Bad request"
		  case 404:
				return "Not found"
		  case 418:
				return "I'm a teapot"
		  case _:
				return "Something's wrong with the internet"

if __name__ == "__main__":
	 print(greet("Python"))
	 dog = Dog("Buddy")
	 print(dog.speak())
	 slow_function()
	 asyncio.run(main())