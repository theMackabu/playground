import scala.collection.mutable
import scala.util.{Try, Success, Failure}

// Define a case class for representing a book
case class Book(title: String, author: String, year: Int)

// Define a trait for a library catalog
trait LibraryCatalog {
  def addBook(book: Book): Unit
  def removeBook(title: String): Option[Book]
  def findBook(title: String): Option[Book]
  def listBooks(): List[Book]
}

// Implement the LibraryCatalog trait
class Library extends LibraryCatalog {
  private val books: mutable.Map[String, Book] = mutable.Map.empty

  def addBook(book: Book): Unit = {
	 books += (book.title -> book)
  }

  def removeBook(title: String): Option[Book] = {
	 books.remove(title)
  }

  def findBook(title: String): Option[Book] = {
	 books.get(title)
  }

  def listBooks(): List[Book] = {
	 books.values.toList
  }
}

// Define an object for library operations
object LibraryOperations {
  def printBookInfo(book: Book): Unit = {
	 println(s"Title: ${book.title}, Author: ${book.author}, Year: ${book.year}")
  }

  def tryAddBook(library: LibraryCatalog, book: Book): Try[Unit] = {
	 Try {
		library.addBook(book)
	 }
  }
}

// Main object to run the program
object LibraryApp extends App {
  // Create a new library
  val library = new Library()

  // Add some books
  val books = List(
	 Book("To Kill a Mockingbird", "Harper Lee", 1960),
	 Book("1984", "George Orwell", 1949),
	 Book("Pride and Prejudice", "Jane Austen", 1813)
  )

  books.foreach { book =>
	 LibraryOperations.tryAddBook(library, book) match {
		case Success(_) => println(s"Added book: ${book.title}")
		case Failure(e) => println(s"Failed to add book ${book.title}: ${e.getMessage}")
	 }
  }

  // Find and print a book
  library.findBook("1984") match {
	 case Some(book) => LibraryOperations.printBookInfo(book)
	 case None => println("Book not found")
  }

  // Remove a book
  library.removeBook("Pride and Prejudice") match {
	 case Some(book) => println(s"Removed book: ${book.title}")
	 case None => println("Book not found")
  }

  // List all books
  println("\nLibrary catalog:")
  library.listBooks().foreach(LibraryOperations.printBookInfo)

  // Demonstrate pattern matching
  def categorizeBook(book: Book): String = book.year match {
	 case year if year < 1900 => "Classic"
	 case year if year < 2000 => "Modern"
	 case _ => "Contemporary"
  }

  println("\nBook categories:")
  library.listBooks().foreach(book => println(s"${book.title}: ${categorizeBook(book)}"))

  // Demonstrate higher-order function
  def processBooks(books: List[Book])(f: Book => Unit): Unit = {
	 books.foreach(f)
  }

  println("\nProcessing books:")
  processBooks(library.listBooks()) { book =>
	 println(s"Processing: ${book.title}")
  }
}