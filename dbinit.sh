#!/bin/bash

rm -f demo.db
sqlite3 demo.db < schema.sql
sqlite3 demo.db "INSERT INTO users (name, password) VALUES ('hans', 'franz');"
sqlite3 demo.db "INSERT INTO users (name, password) VALUES ('bob', '123');"
echo "db created"
