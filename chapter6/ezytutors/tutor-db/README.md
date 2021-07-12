#### Steps to run the server

1. Create the ```database``` and ```user``` with password, as described in the chapter
2. Grant privileges for the user on the database
3. Check if the ```.env``` file has been configured with the correct ```DATABASE_URL``` and ```HOST_PORT``` environment variables.
4. Execute the database scripts for ```tutor``` and ```course``` table creation and seed data upload, in the ```postgres``` database.

5. From the root folder of __tutor-db__, run:

[source,shell]
----
cargo check
cargo run --bin iter5
----

6. Run the automated tests with: 

[source,shell]
----
cargo test
----

7. From a browser:  

[source,shell]
----
http://localhost:3000/tutors/       // To get the list of all tutors        
http://localhost:3000/tutors/2      // To get details of a single tutor
http://localhost:3000/courses/1     // To get all courses for a tutor
http://localhost:3000/courses/1/2   // To get details of a course by a tutor
----

8. For further instructions on creating, updating and deleting tutors and courses, check instructions within the chapter.

#### Debugging tip
In case of difficulty in building the server due to missing/inconsistent dependencies, delete the ```cargo.lock``` file and ```target``` folder and rebuild.
