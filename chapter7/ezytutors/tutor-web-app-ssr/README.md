## Chapter 7 - Introduction to server-side webapps in Rust

#### Example 1: Serve a static web page

From root folder of __tutor-web-app-ssr__, run:
```
cargo run --bin iter1
```
From a browser:  

```
http://localhost:8080/static/static-web-page.html
```

#### Example 2: Serve a dynamic web page

```
cargo run --bin iter1
```

From a browser:

````
http://localhost:8080/
````

#### Example 3: Accept user input with forms

```
cargo run --bin iter2
```

From a browser: 

```
 http://localhost:8080/
```


#### Example 4: Displaying a list with templates

```
cargo run --bin iter3
```

From a browser: 
```
http://localhost:8080/tutors
```

#### Example 5: Writing unit and integration tests

```
cargo test --bin iter2
```

#### Example 6: Connecting to backend web service

From root folder of __tutor-web-app-ssr__, run:
```
cargo run --bin iter4
```

From root folder of __tutor-web-service__, run:
```
cargo run --bin iter5
```
From a browser: 
```
localhost:8080/tutors
```

#### Debugging tip
In case of difficulty in building the server due to missing/inconsistent dependencies, delete the ```cargo.lock``` file and ```target``` folder and rebuild.