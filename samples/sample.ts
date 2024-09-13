// Basic Types
let isDone: boolean = false;
let decimal: number = 6;
let color: string = "blue";
let list: number[] = [1, 2, 3];
let x: [string, number] = ["hello", 10]; // Tuple

// Enum
enum Color {Red, Green, Blue}
let c: Color = Color.Green;

// Any
let notSure: any = 4;
notSure = "maybe a string instead";
notSure = false; // okay, definitely a boolean

// Void
function warnUser(): void {
    console.log("This is my warning message");
}

// Null and Undefined
let u: undefined = undefined;
let n: null = null;

// Never
function error(message: string): never {
    throw new Error(message);
}

// Object
let obj: object = { key: "value" };

// Type assertions
let someValue: any = "this is a string";
let strLength: number = (someValue as string).length;

// Functions
function add(x: number, y: number): number {
    return x + y;
}
let myAdd = function(x: number, y: number): number { return x + y; };

// Optional and Default Parameters
function buildName(firstName: string, lastName?: string) {
    if (lastName)
        return firstName + " " + lastName;
    else
        return firstName;
}

function buildName2(firstName: string, lastName = "Smith") {
    return firstName + " " + lastName;
}

// Rest Parameters
function buildName3(firstName: string, ...restOfName: string[]) {
    return firstName + " " + restOfName.join(" ");
}

// Interfaces
interface LabeledValue {
    label: string;
}

function printLabel(labeledObj: LabeledValue) {
    console.log(labeledObj.label);
}

// Classes
class Greeter {
    greeting: string;
    constructor(message: string) {
        this.greeting = message;
    }
    greet() {
        return "Hello, " + this.greeting;
    }
}

let greeter = new Greeter("world");

// Inheritance
class Animal {
    name: string;
    constructor(theName: string) { this.name = theName; }
    move(distanceInMeters: number = 0) {
        console.log(`${this.name} moved ${distanceInMeters}m.`);
    }
}

class Snake extends Animal {
    constructor(name: string) { super(name); }
    move(distanceInMeters = 5) {
        console.log("Slithering...");
        super.move(distanceInMeters);
    }
}

// Generics
function identity<T>(arg: T): T {
    return arg;
}

let output = identity<string>("myString");

// Decorators
function sealed(constructor: Function) {
    Object.seal(constructor);
    Object.seal(constructor.prototype);
}

@sealed
class BugReport {
    type = "report";
    title: string;

    constructor(t: string) {
        this.title = t;
    }
}

// Modules
export interface StringValidator {
    isAcceptable(s: string): boolean;
}

// Async/Await
async function fetchData(url: string): Promise<any> {
    const response = await fetch(url);
    return response.json();
}

// Type Inference
let a = 3; // a is inferred to be of type number

// Union Types
let multiType: number | boolean;
multiType = 20; // OK
multiType = true; // OK

// Intersection Types
interface ErrorHandling {
    success: boolean;
    error?: { message: string };
}

interface ArtworksData {
    artworks: { title: string }[];
}

type ArtworksResponse = ErrorHandling & ArtworksData;

// Literal Types
type Easing = "ease-in" | "ease-out" | "ease-in-out";

// Utility Types
interface Todo {
    title: string;
    description: string;
}

type TodoPreview = Pick<Todo, "title">;
type ReadonlyTodo = Readonly<Todo>;

export { Animal, Snake, identity, fetchData };
