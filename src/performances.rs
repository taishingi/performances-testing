pub mod performances {
    use colored_truecolor::Colorize;
    use std::collections::HashMap;
    use std::time::Instant;
    pub struct Performances {}

    impl Default for Performances {
        fn default() -> Self {
            Self::new("Performances testing")
        }
    }

    ///
    /// To run performance test
    ///
    impl Performances {
        ///
        /// Constructor
        ///
        pub fn new(description: &str) -> Performances {
            println!("\n{}\n", description.magenta().bold());
            Self {}
        }

        ///
        /// # Check if all callback's time are less than expected time
        ///
        /// - `callbacks`   The expected callback with the expected time in f32s unit
        ///
        ///
        pub fn f32(&mut self, callbacks: HashMap<fn(), f32>) -> &mut Performances {
            for (&k, &v) in callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: f32 = now.elapsed().as_secs_f32();
                assert!(
                    end < v,
                    "A callback take {} f32s and the expected time is {} f32s",
                    end,
                    v
                );
            }
            self.output()
        }

        ///
        /// # Check if all callback's time are less than expected time
        ///
        /// - `callbacks`   The expected callback with the expected time in f64s unit
        ///
        ///
        pub fn f64(&mut self, callbacks: HashMap<fn(), f64>) -> &mut Performances {
            for (&k, &v) in callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: f64 = now.elapsed().as_secs_f64();
                assert!(
                    end < v,
                    "A callback take {} f64s and the expected time is {} f64s",
                    end,
                    v
                );
            }
            self.output()
        }

        ///
        /// # Check if all callback's time are less than expected time
        ///
        /// - `callbacks`   The expected callback with the expected time in nanos unit
        ///
        ///
        pub fn nanos(&mut self, callbacks: HashMap<fn(), u128>) -> &mut Performances {
            for (&k, &v) in callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: u128 = now.elapsed().as_nanos();
                assert!(
                    end < v,
                    "A callback take {} ns and the expected time is {} ns",
                    end,
                    v
                );
            }
            self.output()
        }

        ///
        /// # Check if all callback's time are less than expected time
        ///
        /// - `callbacks`   The expected callback with the expected time in micros seconds unit
        ///
        pub fn micros(&mut self, callbacks: HashMap<fn(), u128>) -> &mut Performances {
            for (&k, &v) in callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: u128 = now.elapsed().as_micros();
                assert!(
                    end < v,
                    "A callback take {} µs and the expected time is {} µs",
                    end,
                    v
                );
            }
            self.output()
        }

        ///
        /// # Check if all callback's time are less than expected time
        ///
        /// - `callbacks`   The expected callback with the expected time in millis seconds unit
        ///
        pub fn millis(&mut self, callbacks: HashMap<fn(), u128>) -> &mut Performances {
            for (&k, &v) in callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: u128 = now.elapsed().as_millis();
                assert!(
                    end < v,
                    "A callback take {} ms and the expected time is {} ms",
                    end,
                    v
                );
            }
            self.output()
        }

        ///
        /// # Check if all callback's time are less than expected time
        ///
        /// - `callbacks`   The expected callback with the expected time in seconds unit
        ///
        pub fn secs(&mut self, callbacks: HashMap<fn(), u64>) -> &mut Performances {
            for (&k, &v) in callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: u64 = now.elapsed().as_secs();
                assert!(
                    end < v,
                    "A callback take {} ms and the expected time is {} ms",
                    end,
                    v
                );
            }
            self.output()
        }

        ///
        /// # Print a point in console after a test runned successfully
        ///
        fn output(&mut self) -> &mut Performances {
            print!("{}", ".".white().bold());
            self
        }

        ///
        /// # End of the test
        ///
        pub fn end(&mut self) -> Result<String, String> {
            println!();
            Ok(String::from("ok"))
        }
    }

    #[macro_export]
    macro_rules! millis {
        ($callbacks:expr) => {
            for (&k, &v) in $callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: u128 = now.elapsed().as_millis();
                assert!(
                    end < v,
                    "A callback take {} ms and the expected time is {} ms",
                    end,
                    v
                );
            }
        };
    }

    #[macro_export]
    macro_rules! micros {
        ($callbacks:expr) => {
            for (&k, &v) in $callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: u128 = now.elapsed().as_micros();
                assert!(
                    end < v,
                    "A callback take {} µs and the expected time is {} µs",
                    end,
                    v
                );
            }
        };
    }

    #[macro_export]
    macro_rules! nanos {
        ($callbacks:expr) => {
            for (&k, &v) in $callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: u128 = now.elapsed().as_nanos();
                assert!(
                    end < v,
                    "A callback take {} ns and the expected time is {} ns",
                    end,
                    v
                );
            }
        };
    }

    #[macro_export]
    macro_rules! f32 {
        ($callbacks:expr) => {
            for (&k, &v) in $callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: f32 = now.elapsed().as_secs_f32();
                assert!(
                    end < v,
                    "A callback take {} f32s and the expected time is {} f32s",
                    end,
                    v
                );
            }
        };
    }

    #[macro_export]
    macro_rules! f64 {
        ($callbacks:expr) => {
            for (&k, &v) in $callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: f64 = now.elapsed().as_secs_f64();
                assert!(
                    end < v,
                    "A callback take {} f64s and the expected time is {} f64s",
                    end,
                    v
                );
            }
        };
    }

    #[macro_export]
    macro_rules! secs {
        ($callbacks:expr) => {
            for (&k, &v) in $callbacks.iter() {
                let now: Instant = Instant::now();
                k();
                let end: u64 = now.elapsed().as_secs();
                assert!(
                    end < v,
                    "A callback take {} s and the expected time is {} s",
                    end,
                    v
                );
            }
        };
    }
}
