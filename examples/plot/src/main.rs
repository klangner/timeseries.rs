use gnuplot::{Figure, Color};

use timeseries::series::TimeSeries;


fn main() {
    let ts = TimeSeries::from_timestamp(0, 60, vec![1.0, 2.5, 3.2]);

    let mut fg = Figure::new();
    fg.axes2d().lines(&ts.index, &ts.data, &[Color("blue")]);
    fg.show();
}