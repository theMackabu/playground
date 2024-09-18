-- Create tables
CREATE TABLE Customers (
	 CustomerID INT PRIMARY KEY,
	 FirstName VARCHAR(50),
	 LastName VARCHAR(50),
	 Email VARCHAR(100),
	 Phone VARCHAR(20)
);

CREATE TABLE Products (
	 ProductID INT PRIMARY KEY,
	 ProductName VARCHAR(100),
	 Price DECIMAL(10, 2),
	 Stock INT
);

CREATE TABLE Orders (
	 OrderID INT PRIMARY KEY,
	 CustomerID INT,
	 OrderDate DATE,
	 TotalAmount DECIMAL(10, 2),
	 FOREIGN KEY (CustomerID) REFERENCES Customers(CustomerID)
);

CREATE TABLE OrderDetails (
	 OrderDetailID INT PRIMARY KEY,
	 OrderID INT,
	 ProductID INT,
	 Quantity INT,
	 FOREIGN KEY (OrderID) REFERENCES Orders(OrderID),
	 FOREIGN KEY (ProductID) REFERENCES Products(ProductID)
);

-- Insert sample data
INSERT INTO Customers (CustomerID, FirstName, LastName, Email, Phone)
VALUES 
(1, 'John', 'Doe', 'john.doe@email.com', '555-1234'),
(2, 'Jane', 'Smith', 'jane.smith@email.com', '555-5678');

INSERT INTO Products (ProductID, ProductName, Price, Stock)
VALUES 
(1, 'Laptop', 999.99, 50),
(2, 'Smartphone', 599.99, 100);

INSERT INTO Orders (OrderID, CustomerID, OrderDate, TotalAmount)
VALUES 
(1, 1, '2023-01-15', 999.99),
(2, 2, '2023-01-16', 1199.98);

INSERT INTO OrderDetails (OrderDetailID, OrderID, ProductID, Quantity)
VALUES 
(1, 1, 1, 1),
(2, 2, 2, 2);

-- Sample SELECT queries
SELECT * FROM Customers;

SELECT ProductName, Price FROM Products WHERE Stock > 0;

SELECT 
	 O.OrderID, 
	 C.FirstName, 
	 C.LastName, 
	 O.OrderDate, 
	 O.TotalAmount
FROM 
	 Orders O
JOIN 
	 Customers C ON O.CustomerID = C.CustomerID;

-- Sample UPDATE
UPDATE Products SET Price = 949.99 WHERE ProductID = 1;

-- Sample DELETE
DELETE FROM OrderDetails WHERE OrderID = 2;
DELETE FROM Orders WHERE OrderID = 2;

-- Sample complex query
SELECT 
	 C.CustomerID,
	 C.FirstName,
	 C.LastName,
	 COUNT(O.OrderID) AS TotalOrders,
	 SUM(O.TotalAmount) AS TotalSpent
FROM 
	 Customers C
LEFT JOIN 
	 Orders O ON C.CustomerID = O.CustomerID
GROUP BY 
	 C.CustomerID, C.FirstName, C.LastName
HAVING 
	 COUNT(O.OrderID) > 0
ORDER BY 
	 TotalSpent DESC;