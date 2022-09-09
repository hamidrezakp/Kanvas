## Endpoints
#### GET `http://localhost:8000/`

In this endpoint you will get canvas as raw binary bytes with
each byte representing a color. There is 256 color and you can get the
list of colors with `http://locahost:8000/colors` request.

#### GET `http://localhost:8000/colors`

Get list of all the 256 colors available

Response:
```json
[
  "#000000",
  "#800000",
  "#008000",
  ...
]
```

#### POST `http://localhost:8000/`
Set color to a cell

Request:
```json
{
    "width": 0,
    "height": 0,
    "color": 12
}
```
Note: Color field is the index of color in the colors array received from server.

## Examining binary result
You can use `hexdump` and `curl` to check output:

`curl localhost:8000/ --output - --silent | hexdump`

