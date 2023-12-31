pub mod performances {
    use colored_truecolor::Colorize;
    use std::collections::HashMap;
    use std::time::Instant;

    pub struct Performances {}

    impl Default for Performances {
        fn default() -> Self {
            Self::new()
        }
    }

    ///
    /// To run performance test
    ///
    impl Performances {
        ///
        /// Constructor
        ///
        pub fn new() -> Performances {
            println!();
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
                Self::output("The callback take less than the expected max time");
            }
            Self::output("End of f32 callback measure test");
            self
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
                Self::output("The callback take less than the expected max time");
            }
            Self::output("End of f64 callback measure test");
            self
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
                Self::output("The callback take less than the expected max time");
            }
            Self::output("End of nanos callback measure test");
            self
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
                Self::output("The callback take less than the expected max time");
            }
            Self::output("End of micros callback measure test");
            self
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
                Self::output("The callback take less than the expected max time");
            }
            Self::output("End of millis callback measure test");
            self
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
                    "A callback take {} secs and the expected time is {} secs",
                    end,
                    v
                );
                Self::output("The callback take less than the expected max time");
            }
            Self::output("End of secs callback measure test");
            self
        }

        ///
        /// # Print a point in console after a test run successfully
        ///
        fn output(x: &str) {
            println!(
                "     {}",
                format!("{} {}", "[ ✓ ]".green().bold(), x.blue().bold()).as_str()
            );
        }

        ///
        /// # End of the test
        ///
        pub fn end(&mut self) -> Result<(), String> {
            println!();
            Ok(())
        }
    }

    #[macro_export]
    macro_rules! millis {
        ($callbacks:expr) => {
            let mut p = Performances::default();
            assert!(p.millis($callbacks).end().is_ok());
        };
    }

    #[macro_export]
    macro_rules! micros {
        ($callbacks:expr) => {
            let mut p = Performances::default();
            assert!(p.micros($callbacks).end().is_ok());
        };
    }

    #[macro_export]
    macro_rules! nanos {
        ($callbacks:expr) => {
            let mut p = Performances::default();
            assert!(p.nanos($callbacks).end().is_ok());
        };
    }

    #[macro_export]
    macro_rules! f32 {
        ($callbacks:expr) => {
            let mut p = Performances::default();
            assert!(p.f32($callbacks).end().is_ok());
        };
    }

    #[macro_export]
    macro_rules! f64 {
        ($callbacks:expr) => {
            let mut p = Performances::default();
            assert!(p.f64($callbacks).end().is_ok());
        };
    }

    #[macro_export]
    macro_rules! secs {
        ($callbacks:expr) => {
            let mut p = Performances::default();
            assert!(p.secs($callbacks).end().is_ok());
        };
    }
}
