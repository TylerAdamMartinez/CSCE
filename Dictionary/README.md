# Homework 3: Dictionary

```json
  "Name": "Tyler Adam Martinez"
  "Class": "CSCE3110"
  "Date": "4/7/2022"
```

## Objective

For this assignment you need to implement two dictionaries using a hash table, one with Closed
hashing and one with Open hashing. Both dictionaries are going to use an integer as key and
are going to store a string as value. Both dictionaries need to have at least this functions:

* string findItem(int key): this function receives a key and returns the string that was stored for the given key.
* bool insert(int key, string value): this function receives a key , value pair and updates the dictionary so it contains the key and the corresponding value in the correct spot according to the hash function. If the key already exists in the dictionary return false and if the insertion was successful return true.
* bool remove(int key): this function receives a key and removes the key , value pair from the dictionary. If the key does not exist in the dictionary return false and if it exists remove it and return true.

For both dictionaries the hash table size is going to be set as 10k. You can use any hash
function you want as long as the output of the hash function is between 0 and (10k-1). Each
dictionary implementation is worth 50 points.

## Where to find stuff
* For Dictionary data structure implementation check src/lib.rs
* For code examples of the Dictionary check src/main.rs
* For unit tests scroll to bottom of src/lib.rs file

## Cargo functions
### Run program
`cargo +nightly run`

### Run unit tests
`cargo +nightly test`

