// This is a sample JavaScript file to test syntax highlighting

// Importing modules
import { useState, useEffect } from 'react';
import axios from 'axios';

// Constants
const PI = 3.14159;
const COLORS = ['red', 'green', 'blue'];

// Variables
let counter = 0;
var oldVar = 'I\'m an old var';
const newConst = `I'm a new const using template literals ${PI}`;

// Functions
function greet(name) {
    return `Hello, ${name}!`;
}

const arrowFunc = (x, y) => x + y;

// Classes
class Animal {
    constructor(name) {
        this.name = name;
    }

    speak() {
        console.log(`${this.name} makes a sound.`);
    }
}

class Dog extends Animal {
    speak() {
        console.log(`${this.name} barks.`);
    }
}

// Async function and Promises
async function fetchData() {
    try {
        const response = await axios.get('https://api.example.com/data');
        return response.data;
    } catch (error) {
        console.error('Error fetching data:', error);
    }
}

// Conditional statements
if (counter > 0) {
    console.log('Counter is positive');
} else if (counter < 0) {
    console.log('Counter is negative');
} else {
    console.log('Counter is zero');
}

// Switch statement
switch (COLORS[0]) {
    case 'red':
        console.log('Color is red');
        break;
    case 'green':
        console.log('Color is green');
        break;
    default:
        console.log('Unknown color');
}

// Loops
for (let i = 0; i < COLORS.length; i++) {
    console.log(COLORS[i]);
}

COLORS.forEach(color => console.log(color));

// Object
const person = {
    name: 'John Doe',
    age: 30,
    hobbies: ['reading', 'gaming'],
    greet() {
        console.log(`Hello, my name is ${this.name}`);
    }
};

// Destructuring
const { name, age } = person;

// Spread operator
const newColors = [...COLORS, 'yellow'];

// Rest parameter
function sum(...numbers) {
    return numbers.reduce((acc, num) => acc + num, 0);
}

// Regular expression
const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

// Error handling
try {
    throw new Error('This is a custom error');
} catch (error) {
    console.error(error.message);
} finally {
    console.log('This always runs');
}

// ES6 Module export
export { greet, Animal, fetchData };
export default person;
