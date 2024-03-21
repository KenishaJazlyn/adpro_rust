**Commit 1**

`handle_connection` function in Rust is designed for processing `TCP connections`, specifically to read and display HTTP 
request headers. It uses `TcpStream` for the connection and `BufReader` for efficient reading. The function reads lines 
from the connection until it hits the end of the HTTP headers, then collects and prints these headers. 

**Commit 2** 
