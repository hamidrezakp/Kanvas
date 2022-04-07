## Run
First install rustup with instructions on [Rustup](https://rustup.rs/).

Then run it:

`cargo run`

## Endpoints
There is two endpoints, one for getting canvas and the other for writing
to canvas:
1. GET `http://localhost:8000/`

In this endpoint you will get canvas as raw binary bytes with
each byte representing a color. There is 256 color and you can get the
list of colors with `http://locahost:8000/colors` request.
2. POST `http://localhost:8000/`

```json
{
    "width": 0,
    "height": 0,
    "color": 12
}
```

## Examining binary result
You can use `hexdump` and `curl` to check output:

`curl localhost:8000/ --output - --silent | hexdump`

