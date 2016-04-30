# Dayder

[Dayder](http://dayder.thume.ca/) is a web app for finding spurious correlations in thousands of data sets. It was originally [created for TerribleHack III](http://devpost.com/software/dayder) where it just worked with 3000+ different causes of death, but we've kept improving it since then. It was made by [Tristan Hume](http://thume.ca/) and [Marc Mailhot](http://mlht.ca/). Since the hackathon we've expanded and optimized it to work with over 390,000 data sets at a time.

Having been originally created for a "Stupid shit no-one needs and terrible ideas" hackathon, we don't even pretend that Dayder is useful. It is mostly a tech demo for making a super fast web app despite dealing with large quantities of data. Dayder uses [a custom binary format](https://github.com/trishume/dayder/blob/master/format.md), custom JS Canvas and DOM rendering, heavily optimized server-side code in Rust, caching and tuned data layouts for excellent performance. It can filter through 390,000+ data sets as you type with sub 50ms server response time and instantaneous rendering of thousands of graphs in the browser. It also can find correlations among all 390,000+ data sets in less than 2 seconds.

![Dayder](http://i.imgur.com/EGyqdb4.png)

## Origins

Inspired by the Spurious Correlations book and website. We thought of making a modern fast reactive site for finding spurious correlations in lots of time series data sets. We started with the data set of various causes of death over time, because it was the funniest in a morbid sort of way.

After the hackathon we added Canadian GDP from various sectors, and the full set of over 300,000 time series from [FRED](https://research.stlouisfed.org/). This required making a whole bunch more improvements to speed and usability.

## How we built it

We designed a custom binary time series data format (btsf!) that allows bandwidth and memory efficient processing of large amounts of time series data.

This is processed using typed arrays in JS on the client and using Rust on the server. All the DOM and canvas rendering is efficient custom code so that it can render thousands of graphs in milliseconds.

Later we optimized it further. It now does as-you-type filtering on the server side so it doesn't have to send 400MB of data to the client. In order to do as-you-type filtering I optimized the filtering method using efficient data layout, lazy processing, caching and incremental sorting to maintain sub 50ms response times on every request.

## License

This project is released under the MIT license, see the `LICENSE` file for details.
