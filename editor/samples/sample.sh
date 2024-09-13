#!/bin/bash

# This is a sample shell script demonstrating various features

# Variables
NAME="World"
AGE=25
readonly CONSTANT="This is a constant"

# Functions
greet() {
	 local name=$1
	 echo "Hello, $name!"
}

# Command substitution
current_date=$(date +"%Y-%m-%d")

# Arithmetic operations
result=$((10 + 5))

# Conditional statements
if [ $AGE -ge 18 ]; then
	 echo "You are an adult."
elif [ $AGE -ge 13 ]; then
	 echo "You are a teenager."
else
	 echo "You are a child."
fi

# Case statement
case $AGE in
	 18) echo "You just became an adult." ;;
	 21) echo "You can now drink in the US." ;;
	 *) echo "Nothing special about this age." ;;
esac

# Loops
for i in {1..5}; do
	 echo "Iteration $i"
done

counter=0
while [ $counter -lt 5 ]; do
	 echo "Counter: $counter"
	 ((counter++))
done

# Arrays
fruits=("apple" "banana" "cherry")
echo "Second fruit: ${fruits[1]}"

# Associative arrays (Bash 4.0+)
declare -A car
car[make]="Toyota"
car[model]="Corolla"
car[year]=2022
echo "Car: ${car[make]} ${car[model]} (${car[year]})"

# Reading user input
echo "What's your name?"
read user_name
echo "Nice to meet you, $user_name!"

# File operations
if [ -f "example.txt" ]; then
	 echo "example.txt exists"
else
	 echo "Creating example.txt"
	 echo "This is an example file" > example.txt
fi

# Command line arguments
echo "Script name: $0"
echo "First argument: $1"
echo "All arguments: $@"
echo "Number of arguments: $#"

# String manipulation
string="Hello, World!"
echo "Length of string: ${#string}"
echo "Uppercase: ${string^^}"
echo "Lowercase: ${string,,}"
echo "Replace: ${string/World/Universe}"

# Error handling
set -e  # Exit immediately if a command exits with a non-zero status
trap 'echo "An error occurred"; exit 1' ERR

# Functions with return values
is_even() {
	 if (( $1 % 2 == 0 )); then
		  return 0  # true
	 else
		  return 1  # false
	 fi
}

# Using the function
if is_even 4; then
	 echo "4 is even"
else
	 echo "4 is odd"
fi

# Here document
cat << EOF > config.txt
This is a multi-line
configuration file
created by the script.
EOF

# Process substitution
diff <(ls -l) <(ls -al)

# Subshells
(
	 cd /tmp
	 echo "Current directory: $(pwd)"
)
echo "Back to: $(pwd)"

# Brace expansion
echo {1..5}
echo {a..e}

# Parameter expansion
DEFAULT_VALUE="default"
echo "${UNDEFINED_VARIABLE:-$DEFAULT_VALUE}"

# Parsing options
while getopts ":a:b:" opt; do
	 case $opt in
		  a) echo "Option -a was triggered, Parameter: $OPTARG" ;;
		  b) echo "Option -b was triggered, Parameter: $OPTARG" ;;
		  \?) echo "Invalid option: -$OPTARG" ;;
	 esac
done

# Exit the script
echo "Script completed successfully"
exit 0