## Chapter 8 - Working with templates for tutor registration

From root folder of __tutor-web-app-ssr__, run:
```
cargo run --bin iter5-ssr
```

From root folder of __tutor-web-service__, run:
```
cargo run --bin iter5
```

From a browser, access the following URL to see the home screen of the tutor web app. Change the port number to the value set in the ```.env``` file.
```
localhost:8080/
```

Further instructions can be found within the chapter.  

#### Debugging tips
1. In case of difficulty in building any of these two servers due to missing/inconsistent dependencies, delete the ```cargo.lock``` file and ```target``` folder in that project folder, and rebuild.  
2. In case of difficulty in running the servers or getting back expected responses, check to ensure that there are separate ```.env``` files, one each for __tutor-web-app-ssr__ and __tutor-web-service__. The ```.env``` file in each project should contain the respective values for ```HOST_PORT``` and ```DATABASE_URL```.  
3. In case of errors in retrieving data from the database, check if the database scripts for table creation in postgres have been executed.