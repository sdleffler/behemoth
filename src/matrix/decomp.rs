use matrix::{Matrix, Square};

pub fn lup<M>(a: M) -> (M, M, M) where M: Matrix + Square {
    let mut l = M::identity();
    let mut u = a;
    let mut p = M::identity();

    let m = u.order();

    for k in 0..m {
        let mut i = k;
        for j in k..m {
            if u[(j, k)] > u[(i, k)] {
                i = j;
            }
        }
        for j in 0..k {
            let tmp = l[(k, j)];
            l[(k, j)] = l[(i, j)];
            l[(i, j)] = tmp;
        }
        u.row_switch_mut(k, i);
        p.col_switch_mut(k, i);
        for j in k+1..m {
            let l_jk = u[(j, k)] / u[(k, k)];
            l[(j, k)] = l_jk;
            u.row_add_mut(j, k, -l_jk);
        }
    }

    (p, l, u)
}
