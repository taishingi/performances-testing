use performances::performances::Performances;
use performances::{f32, f64, micros, millis, nanos, secs};
use std::collections::HashMap;
use std::process::{exit, ExitCode};
use std::thread::sleep;
use std::time::Duration;

fn live() {
    let t = Duration::from_secs_f32(7.0f32);
    sleep(t);
}

fn forgive() {
    let t = Duration::from_secs_f32(8.0f32);
    sleep(t);
}

fn life() {
    let t = Duration::from_secs_f64(7.0f64);
    sleep(t);
}

fn forever() {
    let t = Duration::from_secs_f64(8.0f64);
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

fn me() {
    let t = Duration::from_nanos(9_000_000);
    sleep(t);
}

fn knife() {
    let t = Duration::from_millis(7_000);
    sleep(t);
}

fn you() {
    let t = Duration::from_millis(8_000);
    sleep(t);
}

fn chipper() {
    let t = Duration::from_secs(1);
    sleep(t);
}

fn www() {
    let t = Duration::from_secs(2);
    sleep(t);
}

fn main() -> ExitCode {
    let mut callback_f32: HashMap<fn(), f32> = HashMap::new();
    let mut callback_f64: HashMap<fn(), f64> = HashMap::new();
    let mut callback_nanos: HashMap<fn(), u128> = HashMap::new();
    let mut callback_millis: HashMap<fn(), u128> = HashMap::new();
    let mut callback_micros: HashMap<fn(), u128> = HashMap::new();
    let mut callback_secs: HashMap<fn(), u64> = HashMap::new();

    callback_f32.insert(live, 8.0f32);
    callback_f32.insert(forgive, 9.0f32);
    callback_f64.insert(life, 8.0f64);
    callback_f64.insert(forever, 9.0f64);
    callback_nanos.insert(wife, 8_000_0000);
    callback_nanos.insert(me, 10_000_0000);
    callback_millis.insert(knife, 8_000);
    callback_millis.insert(you, 9_000);
    callback_micros.insert(like, 8_000);
    callback_micros.insert(like, 8_000);
    callback_secs.insert(chipper, 2);
    callback_secs.insert(www, 3);

    f32!(callback_f32);
    f64!(callback_f64);
    nanos!(callback_nanos);
    millis!(callback_millis);
    micros!(callback_micros);
    secs!(callback_secs);

    exit(0);
}
