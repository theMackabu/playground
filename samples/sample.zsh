#!/usr/bin/env zsh

# This is a sample Zsh script demonstrating various Zsh features and syntax

# Set shell options
setopt extended_glob
setopt null_glob
setopt nocaseglob
setopt rcexpandparam

# Variables and Arrays
name="Alice"
age=30
fruits=(apple banana cherry)
declare -A colors
colors[red]="#FF0000"
colors[green]="#00FF00"
colors[blue]="#0000FF"

# Parameter expansion
echo "Hello, ${name:u}!"  # Uppercase
echo "In 5 years, you'll be $((age + 5)) years old."

# Array operations
echo "First fruit: $fruits[1]"
echo "All fruits: $fruits[@]"
echo "Number of fruits: $#fruits"

# Associative array operations
echo "RGB for red: $colors[red]"
echo "All color names: ${(k)colors[@]}"

# Conditionals
if [[ $age -ge 18 ]]; then
	 echo "You are an adult."
elif [[ $age -ge 13 ]]; then
	 echo "You are a teenager."
else
	 echo "You are a child."
fi

# Case statement
case $name in
	 (Alice|Bob)
		  echo "Hello, $name!"
		  ;;
	 (Charlie)
		  echo "Hey, Charlie!"
		  ;;
	 (*)
		  echo "Nice to meet you, $name!"
		  ;;
esac

# Loops
for fruit in $fruits; do
	 echo "I like $fruit"
done

for i in {1..5}; do
	 echo "Count: $i"
done

# While loop
counter=0
while (( counter < 5 )); do
	 echo "While counter: $counter"
	 (( counter++ ))
done

# Functions
greet() {
	 echo "Hello, $1!"
}

greet "World"

# Arithmetic
result=$(( 10 + 5 * 3 ))
echo "10 + 5 * 3 = $result"

# Command substitution
current_date=$(date +"%Y-%m-%d")
echo "Today's date: $current_date"

# Process substitution
diff <(ls -l) <(ls -al)

# Here document
cat << EOF > config.txt
This is a multi-line
configuration file
created by the script.
EOF

# Globbing
echo "Text files: "*.txt

# Extended globbing
echo "Files not ending with .txt: "^*.txt

# Redirection
ls -l > file_list.txt 2>&1

# Piping
echo "Lines in this script: "
cat $0 | wc -l

# String manipulation
string="Hello, World!"
echo "Uppercase: ${string:u}"
echo "Lowercase: ${string:l}"
echo "Length: ${#string}"
echo "Replace: ${string/World/Universe}"

# Ternary-like operator
(( age >= 18 )) && echo "Adult" || echo "Minor"

# Regular expressions
if [[ "test@example.com" =~ ^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$ ]]; then
	 echo "Valid email format"
fi

# Read user input
echo -n "Enter your name: "
read user_name
echo "Hello, $user_name!"

# Error handling
set -e  # Exit immediately if a command exits with a non-zero status
trap 'echo "An error occurred"; exit 1' ERR

# Zsh-specific array operations
echo "Sorted fruits: ${(o)fruits[@]}"
echo "Unique letters in 'banana': ${(u)=:-banana}"

# Parameter flags
echo "Fruits joined by ',': ${(j:,:)fruits}"

# Modifiers
echo "Script name without extension: ${0:t:r}"
echo "Parent directory: ${0:h}"

# Prompts
PS3="Choose a fruit: "
select fruit in "${fruits[@]}"; do
	 echo "You chose $fruit"
	 break
done

# Completion
autoload -Uz compinit
compinit

# Define a custom completion for a command
compdef _gnu_generic my_command

# Zsh modules
zmodload zsh/datetime
echo "UNIX timestamp: $EPOCHSECONDS"

# Ending the script
echo "Zsh script execution completed"
exit 0