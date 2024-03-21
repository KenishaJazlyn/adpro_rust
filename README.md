**Commit 1**

`handle_connection` function in Rust is designed for processing `TCP connections`, specifically to read and display HTTP 
request headers. It uses `TcpStream` for the connection and `BufReader` for efficient reading. The function reads lines 
from the connection until it hits the end of the HTTP headers, then collects and prints these headers. 

**Commit 2**

By changing the handle_connection function to show a simple HTML page, I learned a lot about how websites get to a browser. 
Before, the code just looked at the web request and didn't do much with it. 
But now, it actually sends back a real web page that we can see in our browser. 
This taught me how a server talks to a browser by following specific rules, like saying "OK" to tell the browser everything is fine and how big the page is so the browser knows what to expect. 
It also showed me how to use Rust to open a file and send its contents over the internet. 
![Commit 2 screen capture](img.png)


**Commit 3**

- I've added a new HTML file named oops.html to act as the 404 Not Found page.
- Then, I modified the `handle_connection` function in our HTTP server.
- This function now examines the initial line of the client's HTTP request to check if it's a `GET/HTTP/1.1` request, indicating a request for the root path (/).
- If this is the case, it responds with the status line `HTTP/1.1 200 OK` and serves the content of the `hello.html` file.
- Otherwise, it issues a `HTTP/1.1 404 NOT FOUND` status line and serves the content of the `oops.html` file.
![img_2.png](img_2.png)


**Commit 4**

We've modified `handle_connection` function to handle different HTTP GET requests based on the request line.
- When the request line is exactly "GET / HTTP/1.1", it implies the client is requesting the root directory of the server. The server responds by setting the HTTP status line to "HTTP/1.1 200 OK", indicating a successful request, and choosing "hello.html" as the file to return. This behavior is typical for serving a homepage or a primary entry point of a website.
- If the request line matches "GET /sleep HTTP/1.1", the server interprets this as a command to simulate a delay before responding. It does this by pausing execution for 10 seconds, using Rust's thread::sleep(Duration::from_secs(10)). This could be used to test client handling of delayed server responses. After the delay, the server proceeds to respond with "HTTP/1.1 200 OK" and serves the same "hello.html" file. This case demonstrates how to implement artificial delays for specific endpoints.
- For any other request lines that do not match the above two patterns, the server defaults to responding with "HTTP/1.1 404 NOT FOUND" and serves a "404.html" file. This case is used to handle unknown or unhandled requests, indicating to the client that the requested resource could not be found on the server


**Commit 5**

How the ThreadPool works
- `Initialization`: When the server starts, it creates a ThreadPool with a fixed number of worker threads. These threads are immediately waiting for jobs.
- `Incoming Connections`: As clients connect to the server, the main thread accepts the TCP connections. For each connection, it sends a job (the handle_connection function) to the thread pool.
- `Job Execution`: Worker threads pick up these jobs from the queue. Since the job encapsulates handling a TCP connection, each worker effectively processes the HTTP request, generates the response, and sends it back to the client.
- `Concurrency`: By using a thread pool, the server can handle multiple connections concurrently, up to the number of threads in the pool. This improves the server's responsiveness and scalability compared to a single-threaded or a new-thread-per-request model.