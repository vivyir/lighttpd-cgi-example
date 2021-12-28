# Lighttpd CGI demo (with rust program)
This is a demo of how you could set up CGI on Lighttpd, it uses rust
as the language for the program inside the CGI directory of www, it
parses an http query string and returns it, for example after running
`start.sh` and compiling the cgi program and letting it start lighttpd
you can send a request using:

```
http://127.0.0.1:8080/cgi/cgi?language=Rust&uuid=214a-fcd4-0af5-778f
```

and you would get a result similar to:

```
[
    (
        "language",
        "Rust",
    ),
    (
        "uuid",
        "214a-fcd4-0af5-778f",
    ),
]
```

Be sure to look at the source code and config file for lighttpd to
understand everything better.

## Running
Because of how lighttpd works you have to edit the 10th line of lighttpd.conf
a.k.a the `server.document-root` to the www directory of your cloned repository

## Running without compiling
There exists a precompiled binary so you can check it without needing
to compile the rust program, just run with:

```
lighttpd -Df ./lighttpd.conf
```
