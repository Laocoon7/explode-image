# explode-image

`explode-image` is a command line tool used to split image atlases into individual images.

## Usage
### Defaults:
```
columns: MUST PASS
rows: MUST PASS
padding: (0, 0)
offset: (0, 0)
input: MUST PASS
output: SAME DIRECTORY AS INPUT (will make a sub folder)
fmt: png
```

### Examples:
```
cargo run -- --columns 16 --rows 16 --padding '0, 0' --offset '0, 0' --input ./assets/terminal8x8.png --output ./assets/output --fmt png
```
or
```
cargo run -- -c16 -r16 --padding '0, 0' --offset '0, 0' -i ./assets/terminal8x8.png -o ./assets/output -f png
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
BlueOak-1.0.0