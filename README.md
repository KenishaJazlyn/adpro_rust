**Commit 1**

`handle_connection` function in Rust is designed for processing `TCP connections`, specifically to read and display HTTP 
request headers. It uses `TcpStream` for the connection and `BufReader` for efficient reading. The function reads lines 
from the connection until it hits the end of the HTTP headers, then collects and prints these headers. 

**Commit 2**

- I've added a new HTML file named oops.html to act as the 404 Not Found page.
- Then, I modified the `handle_connection` function in our HTTP server. 
- This function now examines the initial line of the client's HTTP request to check if it's a `GET/HTTP/1.1` request, indicating a request for the root path (/). 
- If this is the case, it responds with the status line `HTTP/1.1 200 OK` and serves the content of the `hello.html` file. 
- Otherwise, it issues a `HTTP/1.1 404 NOT FOUND` status line and serves the content of the `oops.html` file.
![Commit 2 screen capture](img.png)


**Commit 3**
![img_2.png](img_2.png)