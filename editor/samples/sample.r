#!/usr/bin/env Rscript

# This is a sample R script demonstrating various R features and syntax

# Load required libraries
library(ggplot2)
library(dplyr)

# Basic variable assignment
x <- 10
y = 20  # = can also be used, but <- is more common in R

# Data types
numeric_var <- 3.14
integer_var <- 42L
character_var <- "Hello, R!"
logical_var <- TRUE
complex_var <- 3 + 4i

# Vectors
numeric_vector <- c(1, 2, 3, 4, 5)
character_vector <- c("apple", "banana", "cherry")
logical_vector <- c(TRUE, FALSE, TRUE)

# Sequences
seq_vector <- 1:10
seq_by_2 <- seq(1, 10, by = 2)

# Matrices
matrix_data <- matrix(1:9, nrow = 3, ncol = 3)

# Lists
my_list <- list(
  numbers = c(1, 2, 3),
  fruits = c("apple", "banana"),
  matrix = matrix(1:4, 2, 2)
)

# Data frames
df <- data.frame(
  name = c("Alice", "Bob", "Charlie"),
  age = c(25, 30, 35),
  height = c(165, 180, 175)
)

# Factors
gender <- factor(c("male", "female", "male", "female"))

# Basic operations
sum_result <- x + y
product_result <- x * y

# Conditional statements
if (x > y) {
  print("x is greater than y")
} else if (x < y) {
  print("x is less than y")
} else {
  print("x is equal to y")
}

# Loops
for (i in 1:5) {
  print(paste("Iteration:", i))
}

while (x < 15) {
  print(paste("x is now:", x))
  x <- x + 1
}

# Functions
square <- function(n) {
  return(n^2)
}

# Apply family of functions
lapply(1:5, square)
sapply(1:5, square)

# Anonymous functions
(function(x) x^3)(4)

# String manipulation
paste("Hello", "R", "World")
sprintf("Pi is approximately %.2f", pi)

# Regular expressions
grepl("^H", c("Hello", "World", "Hi"))
sub("a", "A", "banana")

# Date and time
current_date <- Sys.Date()
current_time <- Sys.time()

# Statistical functions
mean_value <- mean(c(1, 2, 3, 4, 5))
median_value <- median(c(1, 2, 3, 4, 5))
sd_value <- sd(c(1, 2, 3, 4, 5))

# Random number generation
set.seed(123)
random_numbers <- rnorm(100, mean = 0, sd = 1)

# Data manipulation with dplyr
df_summarized <- df %>%
  group_by(name) %>%
  summarize(mean_age = mean(age))

# Plotting with ggplot2
ggplot(df, aes(x = age, y = height)) +
  geom_point() +
  geom_smooth(method = "lm") +
  labs(title = "Age vs Height", x = "Age", y = "Height")

# Error handling
tryCatch(
  {
	 result <- 10 / 0
  },
  error = function(e) {
	 print(paste("An error occurred:", e$message))
  },
  warning = function(w) {
	 print(paste("A warning occurred:", w$message))
  },
  finally = {
	 print("This is always executed")
  }
)

# Class and methods
setClass("Person",
  slots = c(
	 name = "character",
	 age = "numeric"
  )
)

john <- new("Person", name = "John", age = 30)

setGeneric("introduce", function(x) standardGeneric("introduce"))
setMethod("introduce", "Person", function(x) {
  cat(paste("Hi, I'm", x@name, "and I'm", x@age, "years old.\n"))
})

introduce(john)

# Closures
make_counter <- function() {
  count <- 0
  function() {
	 count <<- count + 1
	 count
  }
}

counter <- make_counter()
counter()
counter()

# Saving and loading data
saveRDS(df, file = "data.rds")
loaded_df <- readRDS("data.rds")

# Ending the script
print("R script execution completed")