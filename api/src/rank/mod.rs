use itertools::Itertools;
use ndarray::{prelude::*, ViewRepr};

pub type FnMetric = fn(&Array1<f32>, &Array1<f32>) -> f32;

pub fn cosine_similarity(a: &Array1<f32>, b: &Array1<f32>) -> f32 {
    let a = a.view();
    let b = b.view();
    let a_norm = a.dot(&a).sqrt();
    let b_norm = b.dot(&b).sqrt();
    a.dot(&b) / (a_norm * b_norm)
}

pub fn rank_by<T: std::fmt::Debug>(query: &Array1<f32>, chunk: &mut Vec<(T, Array1<f32>)>, metric: FnMetric) {
    // chunk
    // //.axis_iter(Axis(0))
    // .into_iter()
    // //.into_par_iter()
    // .map(|(key, embed)| {
    //     let m = metric(&query, &embed);
    //     println!("{}", m);
    //     m
    // })
    // .collect::<Vec<f32>>().sort_by(|a, b| b.partial_cmp(a).unwrap());
    chunk.sort_by_cached_key(|(_, embed)| {
        -(metric(&query, &embed) * 100000.0) as i32;
    });
    //sims.par_sort_unstable_by(|a, b| b.partial_cmp(a).unwrap())
    //sims.voracious_mt_sort(16); <- this is slower than the above
}