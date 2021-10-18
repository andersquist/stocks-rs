use chrono::prelude::*;
use std::io::{Error, ErrorKind};
use yahoo_finance_api as yahoo;

///
/// A trait to provide a common interface for all signal calculations.
///
pub trait AsyncStockSignal {
    ///
    /// The signal's data type.
    ///
    type SignalType;

    ///
    /// Calculate the signal on the provided series.
    ///
    /// # Returns
    ///
    /// The signal (using the provided type) or `None` on error/invalid data.
    ///
    fn calculate(&self, series: &[f64]) -> Option<Self::SignalType>;
}

pub struct PriceDifference;

impl AsyncStockSignal for PriceDifference {
    type SignalType = (f64, f64);

    fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        price_diff(series)
    }
}

pub struct MinPrice;

impl AsyncStockSignal for MinPrice {
    type SignalType = f64;

    fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        min(series)
    }
}

pub struct MaxPrice;

impl AsyncStockSignal for MaxPrice {
    type SignalType = f64;

    fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        max(series)
    }
}

pub struct WindowedSMA {
    pub window_size: usize,
}

impl AsyncStockSignal for WindowedSMA {
    type SignalType = Vec<f64>;

    fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        n_window_sma(self.window_size, series)
    }
}

// finds the minimum value of a given series
pub fn min(series: &[f64]) -> Option<f64> {
    if !series.is_empty() {
        Some(series.iter().fold(f64::MAX, |acc, q| acc.min(*q)))
    } else {
        None
    }
}

// finds the maximum value of a given series
pub fn max(series: &[f64]) -> Option<f64> {
    if !series.is_empty() {
        Some(series.iter().fold(f64::MIN, |acc, q| acc.max(*q)))
    } else {
        None
    }
}

// calculates the difference between the first and last elements of the series
// and returns the expressed as a tuple of (percentage, absolute difference)
pub fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    if !series.is_empty() {
        let (first, last) = (series.first().unwrap(), series.last().unwrap());
        let abs_diff = last - first;
        let first = if *first == 0.0 { 1.0 } else { *first };
        let rel_diff = abs_diff / first;
        Some((abs_diff, rel_diff))
    } else {
        None
    }
}

// calculates a simple moving average over the series with a window size of n elements
pub fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if !series.is_empty() && n > 1 {
        Some(
            series
                .windows(n)
                .map(|v| v.iter().sum::<f64>() / v.len() as f64)
                .collect(),
        )
    } else {
        None
    }
}

pub async fn fetch_closing_data(
    ticker: &str,
    start: &DateTime<Utc>,
    end: &DateTime<Utc>,
) -> std::io::Result<Vec<f64>> {
    let provider = yahoo::YahooConnector::new();
    let response = provider
        .get_quote_history(ticker, *start, *end)
        .await
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut quotes = response
        .quotes()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    if !quotes.is_empty() {
        quotes.sort_by_cached_key(|q| q.timestamp);
        Ok(quotes.iter().map(|q| q.adjclose as f64).collect())
    } else {
        Ok(vec![])
    }
}
