## Code for Chapter 4

### What does this repo contain?

Code for chapter 4 of the book.

To run:

Go to folder chapter4/ezytutors/tutor-db. This is the project root.

### Iteration 1

To run iteration 1:

```
cargo run --bin iter1
```
Ensure the project root contains .env file that contains database access credentials.
Note: The .env file contained in this repo is for learning purposes only. For production, ensure .env is not checked into git repo (include the file in .gitignore).

### Iteration 2

To check whether iter2 compiles:

```
cargo check --bin iter2
```

To run unit tests for iteration 2: 

```
cargo test --bin iter2
```

### Iteration 3

Run the web service with:

```
cargo run --bin iter3
```
From a browser (or using curl):

To retrieve all courses for tutor-id = 1:

```
http://localhost:3000/courses/1
```

To post a new course :

```
curl -X POST localhost:3000/courses/ \
-H "Content-Type: application/json" \
-d '{"tutor_id":1, "course_id":4, "course_name":"Fourth course"}'
```

For retrieving course details for tutor-id =1 and course-id=4
```
http://localhost:3000/courses/1/2
```

To run automated unit tests:

```
cargo test --bin iter3
```

To repeat the unit tests, run the clean-up script first and then run the unit tests, as shown:
```
psql -U $DATABASE_USER -d ezytutors --password < $PROJECT_ROOT/iter3-test-clean.sql
cargo test --bin iter3

```

