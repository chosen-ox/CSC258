use std::env;
extern crate mpi;
// use mpi::point_to_point as p2p;
// use mpi::request::WaitGuard;
use mpi::ffi::MPI_Wtime;
use mpi::traits::*;
use rayon::prelude::*;

fn compute_row_of_sums(a_row: &[f64], b: &[f64], n: usize) -> Vec<f64> {
    let mut unordered_columns = (0..n)
        .into_par_iter()
        .map(|j| (j, (0..n).map(|k| a_row[k] * b[k * n + j]).sum()))
        .collect::<Vec<(usize, f64)>>();

    unordered_columns.par_sort_by(|left, right| left.0.cmp(&right.0));

    unordered_columns
        .into_iter()
        .map(|(_, col_el)| col_el)
        .collect()
}

fn compute_matrix_multiplication(
    a: &Vec<f64>,
    b: &Vec<f64>,
    n: usize,
    row_idx: usize,
    rows: usize,
) -> Vec<f64> {
    let mut unordered_rows = (row_idx..row_idx + rows)
        .into_par_iter()
        .map(move |i| {
            let a_row = &a[i * n..(i + 1) * n];

            (i, compute_row_of_sums(a_row, &b, n as usize))
        })
        .collect::<Vec<(usize, Vec<f64>)>>();

    unordered_rows.par_sort_by(|left, right| left.0.cmp(&right.0));

    unordered_rows
        .into_iter()
        .map(|(_, row)| row)
        .flatten()
        .collect()
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    //   let size = world.size();
    let rank = world.rank();
    let args: Vec<String> = env::args().collect();
    let t1: f64;

    let n = args[2].parse::<usize>().unwrap();

    let mut a = vec![0f64; n * n];
    let mut b = vec![0f64; n * n];
    let processor_num = args[1].parse::<usize>().unwrap();
    let rows = n / processor_num;

    for i in 0..n {
        for j in 0..n {
            let idx = i * n + j;
            a[idx] = (idx + 1) as f64;
            b[idx] = 1f64 / (idx as f64 + 1f64);
        }
    }

    if rank == 0 {
        unsafe {
            t1 = MPI_Wtime();
        }
        let mut c = compute_matrix_multiplication(&a, &b, n as usize, 0, rows as usize);
        c.resize_with((n * n) as usize, Default::default);

        for i in 1..processor_num {
            world
                .process_at_rank(i as i32)
                .receive_into(&mut c[(i * rows * n) as usize..((i + 1) * rows * n) as usize]);
        }
        unsafe {
            let t2 = MPI_Wtime();
            println!(
                "{}",
                (2 * n * n * n) as f64  / (t2-t1) / 1E6 
            );
        }
        let mut d = vec![0f64; n * n];

        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let idx = i as usize * n + j as usize;
                    d[idx] += a[i * n + k] * b[k * n + j];
                }
            }
        }

        for i in 0..n * n {
            if c[i] != d[i] {
                println!("{} {} {}", i, c[i], d[i]);
            }
        }
    } else {
        let c = compute_matrix_multiplication(&a, &b, n, rank as usize * rows, rows);
        world.process_at_rank(0).send(&c);
    }

    // println!("{}", mpi::environment::processor_name().unwrap());
}
