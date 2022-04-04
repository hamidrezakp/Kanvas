## Run
First install rustup with instructions on [Rustup](https://rustup.rs/).

Then run it:

`cargo run`

## Endpoints
There is two endpoints, one for getting canvas and the other for writing
to canvas:
1. GET `http://localhost:8000/`

In this endpoint you will get canvas as raw binary bytes with
each byte representing a color. (currently we have only white
and black)

2. POST `http://localhost:8000/`

```json
{
    "width": 0,
    "height": 0,
    "color": "White"
}
```

## Examining binary result
You can use `hexdump` and `curl` to check output:

`curl localhost:8000/ --output - --silent | hexdump`

