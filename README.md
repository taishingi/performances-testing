# Installation

```bash
cargo add performances-testing
```

![build status](https://raw.githubusercontent.com/taishingi/performances-testing/master/badges/build.svg) ![tests status](https://raw.githubusercontent.com/taishingi/performances-testing/master/badges/tests.svg) ![clippy status](https://raw.githubusercontent.com/taishingi/performances-testing/master/badges/clippy.svg) ![documentation build](https://raw.githubusercontent.com/taishingi/performances-testing/master/badges/documentation.svg) ![check status](https://raw.githubusercontent.com/taishingi/performances-testing/master/badges/check.svg)





<img title="performances testing output" src="https://raw.githubusercontent.com/taishingi/performances-testing/master/perf-testing-take.gif" alt="performances-testing output" data-align="inline">

## Usage

```rust
use performances::performances::Performances;
use std::process::{exit, ExitCode};
use std::{collections::HashMap, thread::sleep, time::Duration};

fn live() {
    let t = Duration::from_secs_f32(7.0f32);
    sleep(t);
}

fn life() {
    let t = Duration::from_secs_f64(7.0f64);
    sleep(t);
}

fn like() {
    let t = Duration::from_micros(7_000);
    sleep(t);
}

fn wife() {
    let t = Duration::from_nanos(7_000_000);
    sleep(t);
}

fn knife() {
    let t = Duration::from_millis(7_000);
    sleep(t);
}

fn chipper() {
    let t = Duration::from_secs(1);
    sleep(t);
}

pub fn main() -> ExitCode {
    let mut callback_f32: HashMap<fn(), f32> = HashMap::new();
    let mut callback_f64: HashMap<fn(), f64> = HashMap::new();
    let mut callback_nanos: HashMap<fn(), u128> = HashMap::new();
    let mut callback_millis: HashMap<fn(), u128> = HashMap::new();
    let mut callback_micros: HashMap<fn(), u128> = HashMap::new();
    let mut callback_secs: HashMap<fn(), u64> = HashMap::new();

    callback_f32.insert(live, 8.0f32);
    callback_f64.insert(life, 8.0f64);
    callback_nanos.insert(wife, 8_000_0000);
    callback_millis.insert(knife, 8_000);
    callback_micros.insert(like, 8_000);
    callback_micros.insert(like, 8_000);
    callback_secs.insert(chipper, 8);

    let mut p = Performances::default();
    p.f32(callback_f32);
    p.f64(callback_f64);
    p.nanos(callback_nanos);
    p.millis(callback_millis);
    p.micros(callback_micros);
    p.secs(callback_secs);
    assert!(p.end().is_ok());
    exit(0);
}
```

### Run test

```bash
cargo run     
```
