# Dayder

## Inspiration

Inspired by the Spurious Correlations book and website. We thought of making a modern fast reactive site for finding spurious correlations in lots of time series data sets. We started with the data set of various causes of death over time, because it was the funniest in a morbid sort of way.

After the hackathon we added Canadian GDP from various sectors, and the full set of over 300,000 time series from [FRED](https://research.stlouisfed.org/). This required making a whole bunch more improvements to speed and usability.

## What it does

Finds correlations between different causes of death over time and charts them. Includes super fast as-you-type filtering of thousands of data sets.

## How we built it

We designed a custom binary time series data format (btsf!) that allows bandwidth and memory efficient processing of large amounts of time series data.

This is processed using typed arrays in JS on the client and using Rust on the server. All the DOM and canvas rendering is efficient custom code so that it can render thousands of graphs in milliseconds.

Later we optimized it further. It now does as-you-type filtering on the server side so it doesn't have to send 400MB of data to the client. In order to do as-you-type filtering I optimized the filtering method using efficient data layout, lazy processing, caching and incremental sorting to maintain sub 50ms response times on every request.

## License

This project is released under the MIT license, see the `LICENSE` file for details.
