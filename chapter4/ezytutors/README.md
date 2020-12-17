## Code for Chapter 3

### What does this repo contain?

#### This repo contains two projects - basic-server and tutor-service.

- For both projects, clone this repo and cd into the directory. This is the workspace root folder.

### How to test and run the basic-server

1. From one terminal window, run from workspace root:

```
cargo run -p tutor-nodb --bin basic-server
```

2. From a web browser access the following URI:

```
http://localhost:3000/health
```

You should see a web page displayed with the following message:

```
"Hello. EzyTutors is alive and kicking"
```

### How to test and run the tutor-service project

1. From a terminal window, run from workspace root:

```
cargo run -p tutor-nodb --bin tutor-service
```

2. From a new terminal window, run the following command to post a few courses:

```
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my first course !"}'
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my second course !"}'
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{"tutor_id":1, "course_name":"Hello , my third course !"}'

```

2. From a web browser, access the following URIs:

```
localhost:3000/health
localhost:3000/courses/1
localhost:3000/courses/1/1
```

For the first URI, you should see a message similar to this. The visitor count will increment everytime you refresh the browser screen

```
"I'm good. You've already asked me  1 times"
```

For the second URI, you should see the following:

```
[{"tutor_id":1,"course_id":1,"course_name":"Hello , my first course !","posted_time":"2020-10-30T10:24:16.985954"},{"tutor_id":1,"course_id":2,"course_name":"Hello , my second course !","posted_time":"2020-10-30T10:24:17.013067"},{"tutor_id":1,"course_id":3,"course_name":"Hello , my third course !","posted_time":"2020-10-30T10:24:17.907214"}]
```

For the third URI, you should see the following:

```
{"tutor_id":1,"course_id":1,"course_name":"Hello , my first course !","posted_time":"2020-10-30T10:24:16.985954"}
```

3. To just run the tests for tutor-service, from the workspace root:

```
cargo test -p tutor-nodb
```

You should see the following message displayed:

```
running 3 tests
test handlers::tests::get_all_courses_success ... ok
test handlers::tests::get_one_course_success ... ok
test handlers::tests::post_course_test ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
