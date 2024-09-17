# frozen_string_literal: true

require 'date'

# Module for utility methods
module Utilities
  def self.generate_id
	 "ID-#{rand(1000...9999)}"
  end
end

# Custom error class
class InvalidAgeError < StandardError; end

# Person class to represent individuals
class Person
  attr_reader :id, :name, :age

  def initialize(name, age)
	 @id = Utilities.generate_id
	 @name = name
	 @age = age
	 validate_age
  end

  def to_s
	 "#{@name} (ID: #{@id}, Age: #{@age})"
  end

  private

  def validate_age
	 raise InvalidAgeError, "Age must be between 0 and 120" unless (0..120).cover?(@age)
  end
end

# Employee class inheriting from Person
class Employee < Person
  attr_reader :position, :hire_date

  def initialize(name, age, position)
	 super(name, age)
	 @position = position
	 @hire_date = Date.today
  end

  def to_s
	 "#{super} - #{@position}, Hired: #{@hire_date}"
  end

  def years_employed
	 (Date.today - @hire_date).to_i / 365
  end
end

# Company class to manage employees
class Company
  def initialize(name)
	 @name = name
	 @employees = []
  end

  def add_employee(employee)
	 @employees << employee
  end

  def list_employees
	 puts "Employees of #{@name}:"
	 @employees.each { |emp| puts emp }
  end

  def find_employee_by_id(id)
	 @employees.find { |emp| emp.id == id }
  end
end

# Main program execution
if __FILE__ == $PROGRAM_NAME
  # Create a company
  acme_corp = Company.new("ACME Corporation")

  # Add employees
  begin
	 acme_corp.add_employee(Employee.new("Alice Smith", 30, "Developer"))
	 acme_corp.add_employee(Employee.new("Bob Johnson", 45, "Manager"))
	 acme_corp.add_employee(Employee.new("Charlie Brown", 22, "Intern"))
	 acme_corp.add_employee(Employee.new("David Lee", 150, "CEO"))  # This will raise an error
  rescue InvalidAgeError => e
	 puts "Error adding employee: #{e.message}"
  end

  # List all employees
  acme_corp.list_employees

  # Find an employee by ID
  puts "\nFinding employee:"
  employee = acme_corp.find_employee_by_id("ID-1234")
  if employee
	 puts "Found: #{employee}"
	 puts "Years employed: #{employee.years_employed}"
  else
	 puts "Employee not found"
  end

  # Demonstrate some Ruby features
  puts "\nDemonstrating Ruby features:"
  numbers = [1, 2, 3, 4, 5]
  squared = numbers.map { |n| n**2 }
  puts "Original numbers: #{numbers}"
  puts "Squared numbers: #{squared}"

  puts "\nUsing a Range:"
  (1..5).each { |i| puts "Number: #{i}" }

  puts "\nString interpolation and symbols:"
  status = :active
  puts "The status is #{status.upcase}"

  puts "\nUsing a Hash:"
  person = { name: "Eve", age: 25, city: "New York" }
  person.each { |key, value| puts "#{key}: #{value}" }

  puts "\nUsing a Proc:"
  greeter = ->(name) { puts "Hello, #{name}!" }
  greeter.call("Ruby Developer")
end