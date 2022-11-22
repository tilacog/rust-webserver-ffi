# rust-webserver-ffi
POC for running a rust webserver in Node

## build

```bash
$ npm install
```

## run

This will spin a HTTP server at `https://localhost:3000`.

```bash
$ node
> const rustServer = require(".")
> rustServer.serve()
```

It accepts a `GET` requests at `/` and `POST` requests at `/user`:
```
❯ curlie :3000
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 13
date: Tue, 22 Nov 2022 21:18:20 GMT

Hello, World!%
```

```
$ curlie :3000/users username=ferris
HTTP/1.1 201 Created
content-type: application/json
content-length: 31
date: Tue, 22 Nov 2022 21:18:33 GMT

{
    "id": 1337,
    "username": "ferris"
}
```
