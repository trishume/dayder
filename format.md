# Binary Time Series Format

## Record format, all 32 bit integers:

- `N`: number of data points
- `L`: length of the name in **bytes**
- `T`: start time int64 in unix epoch seconds (including negative times for before 1970)
- `D`: int32 epoch seconds between data points
- `X#`: `F-3` extra integers padding out to the size of the record header specified in the file header.
- `S`: `L` bytes of a string representing the name of the record, in UTF-8
- `P`: `N` int32 representing the data points (`MIN_INT` represents missing data point)

## File format:

Designated extension: `.btsf`

- A header containing:
  - `V` a version number (currently 1)
  - `F` the number of **bytes** in the file header (currently 4*4)
  - `H` the number of **bytes** in a record header (currently `3*4+1*8`)
  - `R` the number of records in the file
- `R` concatenated records

## Pro tips for using the format

When reading, store an integer corresponding to `H` and add that to the byte offset of a record to get the index of the name.
Skip over an additional `L` bytes to get to the data.
Skip over `N*4+L+H` bytes to get to the next record in the file.

When reading the header, read in the numbers and then start at index `F` in the file for reading records.
