#!/bin/bash

rm -f demo.db
sqlite3 demo.db < schema.sql
sqlite3 demo.db "INSERT INTO users (name, password) VALUES ('hans', 'password123');"
sqlite3 demo.db "INSERT INTO users (name, password) VALUES ('bob', '123password');"
echo "db created"
