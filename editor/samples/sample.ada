-- File: library_system.adb

with Ada.Text_IO;         use Ada.Text_IO;
with Ada.Integer_Text_IO; use Ada.Integer_Text_IO;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;

procedure Library_System is

	-- Custom exceptions
	Invalid_Book_ID : exception;

	-- Record type for Book
	type Book is record
		ID     : Positive;
		Title  : Unbounded_String;
		Author : Unbounded_String;
		Available : Boolean := True;
	end record;

	-- Array type for Book collection
	type Book_Array is array (Positive range <>) of Book;

	-- Package for Library operations
	package Library is
		procedure Add_Book (Title, Author : in String);
		procedure Display_Books;
		procedure Borrow_Book (Book_ID : in Positive);
		procedure Return_Book (Book_ID : in Positive);
	private
		Max_Books : constant := 100;
		Books : Book_Array (1 .. Max_Books);
		Book_Count : Natural := 0;
	end Library;

	-- Package body implementation
	package body Library is
		procedure Add_Book (Title, Author : in String) is
		begin
			if Book_Count < Max_Books then
				Book_Count := Book_Count + 1;
				Books(Book_Count) := (ID => Book_Count,
											 Title => To_Unbounded_String(Title),
											 Author => To_Unbounded_String(Author),
											 Available => True);
				Put_Line("Book added successfully. ID: " & Integer'Image(Book_Count));
			else
				Put_Line("Error: Library is full.");
			end if;
		end Add_Book;

		procedure Display_Books is
		begin
			if Book_Count = 0 then
				Put_Line("The library is empty.");
			else
				for I in 1 .. Book_Count loop
					Put("ID: ");
					Put(Books(I).ID, Width => 3);
					Put(" | Title: ");
					Put(To_String(Books(I).Title));
					Put(" | Author: ");
					Put(To_String(Books(I).Author));
					Put(" | Available: ");
					if Books(I).Available then
						Put_Line("Yes");
					else
						Put_Line("No");
					end if;
				end loop;
			end if;
		end Display_Books;

		procedure Borrow_Book (Book_ID : in Positive) is
		begin
			if Book_ID > Book_Count then
				raise Invalid_Book_ID;
			elsif not Books(Book_ID).Available then
				Put_Line("Error: Book is already borrowed.");
			else
				Books(Book_ID).Available := False;
				Put_Line("Book borrowed successfully.");
			end if;
		end Borrow_Book;

		procedure Return_Book (Book_ID : in Positive) is
		begin
			if Book_ID > Book_Count then
				raise Invalid_Book_ID;
			elsif Books(Book_ID).Available then
				Put_Line("Error: Book is not borrowed.");
			else
				Books(Book_ID).Available := True;
				Put_Line("Book returned successfully.");
			end if;
		end Return_Book;
	end Library;

	-- Main procedure
	Choice : Integer;
	Book_ID : Positive;
	Title, Author : String(1..50);
	Last : Natural;

begin
	loop
		Put_Line("Library Management System");
		Put_Line("1. Add a book");
		Put_Line("2. Display all books");
		Put_Line("3. Borrow a book");
		Put_Line("4. Return a book");
		Put_Line("5. Exit");
		Put("Enter your choice: ");
		Get(Choice);

		case Choice is
			when 1 =>
				Put("Enter book title: ");
				Get_Line(Title, Last);
				Put("Enter book author: ");
				Get_Line(Author, Last);
				Library.Add_Book(Title(1..Last), Author(1..Last));

			when 2 =>
				Library.Display_Books;

			when 3 =>
				Put("Enter book ID to borrow: ");
				Get(Book_ID);
				Library.Borrow_Book(Book_ID);

			when 4 =>
				Put("Enter book ID to return: ");
				Get(Book_ID);
				Library.Return_Book(Book_ID);

			when 5 =>
				Put_Line("Thank you for using the Library Management System.");
				exit;

			when others =>
				Put_Line("Invalid choice. Please try again.");
		end case;

		New_Line;
	end loop;

exception
	when Invalid_Book_ID =>
		Put_Line("Error: Invalid book ID.");
	when Data_Error =>
		Put_Line("Error: Invalid input. Please enter a number.");
	when others =>
		Put_Line("An unexpected error occurred.");
end Library_System;