-- MySQL example database 
-- Change rust_test_data to real name when needed
DROP DATABASE if EXISTS rust_test_data;
CREATE DATABASE rust_test_data;
USE rust_test_data;
CREATE TABLE basic(
id INT AUTO_INCREMENT PRIMARY KEY,
DATA TEXT NOT NULL);
INSERT INTO basic VALUES (NULL, "Este valor es el que se deberia ver desde la base de datos");