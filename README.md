# Time Series library

[![Build Status](https://travis-ci.org/klangner/timeseries.rs.svg?branch=master)](https://travis-ci.org/klangner/timeseries.rs)
[![Crates.io](https://img.shields.io/crates/v/timeseries.svg)](https://crates.io/crates/timeseries) 
[![Crates.io](https://img.shields.io/crates/l/timeseries.svg)](https://github.com/klangner/timeseries/blob/master/LICENSE-MIT) 
[![docs.rs](https://docs.rs/timeseries/badge.svg)](https://docs.rs/timeseries/)

**timeseries is an early-stage open-source project**. It means that API can change at any time.
If you think that this library can help you, then let me know. We can discuss future direction and try to stabilize the API.

The folder [examples](https://github.com/klangner/timeseries.rs/tree/master/examples) contains demo programs 
which shows how to use this library.


# Features
   
* Basic functionality
    * [ ] Slicing series
    * [ ] Map, fold and filter
    * [ ] Integration
    * [ ] Differentiation
    * [ ] groupBy
    * [ ] Rolling window
    * [ ] Resampling 
    * [ ] join and merge
  * Calculate statistics
    * [ ] min, max
    * [ ] mean, variance and standard deviation
    * [ ] covariance and correlation
    * [ ] normalization
  * IO
    * [ ] Read data to/from CSV string
  * Generators
    * [ ] Constant series
    * [ ] Random noise
    * [ ] Random Walk
    * [ ] periodic pattern
  * Metrics
    * [ ] MSE between 2 series
    * [ ] MAE between 2 series
  * ARIMA
    * [ ] Check is series is stationary
    * [ ] AR(p) - Autoregressive
    * [ ] I(d) - Integrate
    * [ ] MA(q) - Moving average
  * Advanced functionality
    * [ ] Finding sessions (periods of activity)  
  
# License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.


**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
