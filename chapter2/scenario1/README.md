## Code for Chapter 2

### What does this repo contain?

#### This repo contains two projects - TCP server/client and HTTP Server.

- For both projects, clone this repo and cd into the directory. This is the workspace root folder.

### How to test and run the TCP server/client project from workspace root.

1. From one terminal window, run:

```
cargo run -p tcpserver
```

2. From another terminal window, run:

```
cargo run -p tcpclient
```

You should see the following message printed to your terminal window from where the client is run:

```
Got response from server:"Hello"
```

3. To just run the tests:

```
cargo test -p http
```

You should see the following message printed to your terminal:

```
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### How to test and run the HTTP Server project

1. From a terminal window, run:

```
cargo run -p httpserver
```

2. From a web browser, access the following URIs:

```
localhost:3000
localhost:3000/health
localhost:3000/api/shipping/orders
localhost:3000/invalid-page
```

For the first two URIs, you should be able to see the corresponding HTML pages served.
For the third URI, you should be able to see json data returned from the server.
For the last request, you should see a 404 error page.
