#![allow(non_snake_case)]
use stocks::*;

#[test]
fn test_PriceDifference_calculate() {
    let signal = PriceDifference {};
    assert_eq!(signal.calculate(&[]), None);
    assert_eq!(signal.calculate(&[1.0]), Some((0.0, 0.0)));
    assert_eq!(signal.calculate(&[1.0, 0.0]), Some((-1.0, -1.0)));
    assert_eq!(
        signal.calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]),
        Some((8.0, 4.0))
    );
    assert_eq!(
        signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]),
        Some((1.0, 1.0))
    );
}

#[test]
fn test_MinPrice_calculate() {
    let signal = MinPrice {};
    assert_eq!(signal.calculate(&[]), None);
    assert_eq!(signal.calculate(&[1.0]), Some(1.0));
    assert_eq!(signal.calculate(&[1.0, 0.0]), Some(0.0));
    assert_eq!(
        signal.calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]),
        Some(1.0)
    );
    assert_eq!(
        signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]),
        Some(0.0)
    );
}

#[test]
fn test_MaxPrice_calculate() {
    let signal = MaxPrice {};
    assert_eq!(signal.calculate(&[]), None);
    assert_eq!(signal.calculate(&[1.0]), Some(1.0));
    assert_eq!(signal.calculate(&[1.0, 0.0]), Some(1.0));
    assert_eq!(
        signal.calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0]),
        Some(10.0)
    );
    assert_eq!(
        signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]),
        Some(6.0)
    );
}

#[test]
fn test_WindowedSMA_calculate() {
    let series = vec![2.0, 4.5, 5.3, 6.5, 4.7];

    let signal = WindowedSMA { window_size: 3 };
    assert_eq!(
        signal.calculate(&series),
        Some(vec![3.9333333333333336, 5.433333333333334, 5.5])
    );

    let signal = WindowedSMA { window_size: 5 };
    assert_eq!(signal.calculate(&series), Some(vec![4.6]));

    let signal = WindowedSMA { window_size: 10 };
    assert_eq!(signal.calculate(&series), Some(vec![]));
}
