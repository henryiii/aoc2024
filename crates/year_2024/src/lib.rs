use seq_macro::seq;

seq!(N in 01..=19 {
    pub mod day_~N;
});
