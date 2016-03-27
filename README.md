# Dayder

## Inspiration

Inspired by the Spurious Correlations book and website. We thought of making a modern fast reactive site for finding spurious correlations in lots of time series data sets. We started with the data set of various causes of death over time, because it was the funniest in a morbid sort of way.

## What it does

Finds correlations between different causes of death over time and charts them. Includes super fast as-you-type filtering of thousands of data sets.

## How we built it

We designed a custom binary time series data format (btsf!) that allows bandwidth and memory efficient processing of large amounts of time series data.

This is processed using typed arrays in JS on the client and using Rust on the server. All the DOM and canvas rendering is efficient custom code so that it can render thousands of graphs in milliseconds.