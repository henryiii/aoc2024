#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]

use seq_macro::seq;

seq!(N in 01..=19 {
    pub mod day_~N;
});
