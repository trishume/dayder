# Binary Time Series Format

All numbers are little-endian.

## Record format, all 32 bit integers:

- `N`: uint32 number of data points
- `L`: uint32 length of the name in **bytes**
- `X#`: `H-8` extra bytes padding out to the size of the record header specified in the file header.
- `S`: `L` bytes of a string representing the name of the record, in UTF-8
- `P`: `N` data points
  - `T`: int32 time in unix epoch
  - `D`: float32 data point

## File format:

Designated extension: `.btsf`

- A header containing:
  - `V` uint32 version number (currently 1)
  - `F` uint32 the number of **bytes** in the file header (currently `4*4`)
  - `H` uint32 the number of **bytes** in a record header (currently `2*4`)
  - `R` uint32 the number of records in the file
- `R` concatenated records

## Pro tips for using the format

When reading, store an integer corresponding to `H` and add that to the byte offset of a record to get the index of the name.
Skip over an additional `L` bytes to get to the data.
Skip over `N*8+L+H` bytes to get to the next record in the file.

When reading the header, read in the numbers and then start at index `F` in the file for reading records.
