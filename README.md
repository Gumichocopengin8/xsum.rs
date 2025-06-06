# Xsum in Rust

This crate implments xsum algorithm by Radford M. Neal (https://arxiv.org/abs/1505.05571).

xsum is able to calculate fast exact summation

⚠️ Currently, xsum supports `f64` calculation only.

## Usage

### `add_list()` to take vector or array

Calculates the sum of a small-sized vector or array.

```rs
use xsum::{Xsum, XsumSmall};

let mut xsmall = XsumSmall::new();
xsmall.add_list(&vec![1.0, 2.0, 3.0]);
assert_eq!(xsmall.sum(), 6.0);
```

Calculates the sum of a large-sized vector or array (more than 1,000 elements).

```rs
use xsum::{Xsum, XsumLarge};

let mut xlarge = XsumLarge::new();
xlarge.add_list(&vec![1.0; 1_000]);
assert_eq!(xlarge.sum(), 1000.0);
```

Calculates the sum of a unknown-sized vector or array.

```rs
use xsum::{Xsum, XsumAuto};

let mut xauto = XsumAuto::new();
xauto.add_list(&vec![1.0; 1_000]);
assert_eq!(xauto.sum(), 1000.0);
```

### `add()` to take a floating point number

```rs
use xsum::{Xsum, XsumSmall};

let mut xsmall = XsumSmall::new();
let vec = vec![1.0, 2.0, 3.0];
for v in vec {
    xsmall.add(v);
}
assert_eq!(xsmall.sum(), 6.0);
```

### Chaining Method

```rs
use xsum::{Xsum, XsumExt};

let vec = vec![1.0, 2.0, 3.0];
assert_eq!(vec.xsum(), 6.0);
```

## Documentation

The doc can be found on [docs.rs](https://docs.rs/xsum/).
