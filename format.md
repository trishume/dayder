# Binary Time Series Format

btsf<sup>TM</sup> is a revolutionary<sup>[1](#foot1)</sup> advanced data format that utilizes state-of-the-art 0 and 1 technology to communicate time series data in a compact and efficient manner.

All numbers are little-endian.

## Record format, all 32 bit integers:

- `N`: uint32 number of data points
- `L`: uint32 length of the name in **bytes**
- `X1`: proprietary extension space
- `X#`: `H-8` extra bytes padding out to the size of the record header specified in the file header.
- `S`: `L` bytes of a string representing the name of the record, in UTF-8
- `P`: `N` data points
  - `T`: int32 time in unix epoch
  - `D`: float32 data point

## File format:

Designated extension: `.btsf`

- A header containing:
  - `V` uint32 version number (currently 2)
  - `F` uint32 the number of **bytes** in the file header (currently `4*4`)
  - `H` uint32 the number of **bytes** in a record header (currently `3*4`)
  - `R` uint32 the number of records in the file
- `R` concatenated records

## Pro tips for using the format

When reading, store an integer corresponding to `H` and add that to the byte offset of a record to get the index of the name.
Skip over an additional `L` bytes to get to the data.
Skip over `N*8+L+H` bytes to get to the next record in the file.

When reading the header, read in the numbers and then start at index `F` in the file for reading records.

## Usage of proprietary extension field

The X1 field of each record is reserved for proprietary extensions.
Which extension(s) are being used must be communicated out of band, alongside the file.

### Current extensions

| Proposal Number | Title | Description |
| --------------- | ----- | ----------- |
| BTSF-X0116.RV   | btsf<sup>TM</sup> Extension for the Communication of Statistical Correlations | For users of this extension, the X1 field will contain a float32 value specifying the statistical correlation between the record and a predetermined data set

## Governance

The btsf<sup>TM</sup> standard is governed by the btsf<sup>TM</sup> steering committee, the membership of which is selected by contributors on a regular<sup>[2](#foot2)</sup> basis.

Current steering committee members
- Tristan Hume
- Marc Mailhot

<a name="foot1">1</a>: Revolution not guaranteed
<a name="foot2">2</a>: For some definion of regular