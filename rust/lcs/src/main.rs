// https://rosettacode.org/wiki/Longest_common_subsequence
// Recursive: Translated from C#
use std::collections::VecDeque;

fn lcs_r<T: Eq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    if a.len() == 0 || b.len() == 0 {
        return Vec::new();
    }

    let a_sub = &a[0..a.len() - 1];
    let b_sub = &b[0..b.len() - 1];

    if a[a.len() - 1] == b[b.len() - 1] {
        let mut s = lcs_r(a_sub, b_sub);
        s.push(a[a.len() - 1].clone());
        s
    } else {
        let (x, y) = (lcs_r(a, b_sub), lcs_r(a_sub, b));
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn lcs_r_string(a: &str, b: &str) -> String {
    let a_vec = a.chars().collect::<Vec<_>>();
    let b_vec = b.chars().collect::<Vec<_>>();

    lcs_r(&a_vec, &b_vec).into_iter().collect::<String>()
}

// Dynamic Programming: Translated from C
fn lcs<T: Eq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut c = (0..a.len() + 1).map(|_| Vec::new()).collect::<Vec<_>>();

    for _ in 0..b.len() + 1 {
        c[0].push(0);
    }

    for i in 1..a.len() + 1 {
        c[i].push(0);
        for j in 1..b.len() + 1 {
            let val = if a[i - 1] == b[j - 1] {
                c[i - 1][j - 1] + 1
            } else {
                std::cmp::max(c[i - 1][j], c[i][j - 1])
            };
            c[i].push(val);
        }
    }

    let t = c[a.len()][b.len()];
    let mut s = VecDeque::with_capacity(t);
    {
        let (mut i, mut j) = (a.len(), b.len());
        for _ in 0..t {
            while a[i - 1] != b[j - 1] {
                if c[i][j - 1] > c[i - 1][j] {
                    j -= 1;
                } else {
                    i -= 1;
                }
            }

            s.push_front(a[i - 1].clone());
            i -= 1;
            j -= 1;
        }
    }
    Vec::from(s)
}

fn lcs_string(a: &str, b: &str) -> String {
    let a_vec = a.chars().collect::<Vec<_>>();
    let b_vec = b.chars().collect::<Vec<_>>();

    lcs(&a_vec, &b_vec).into_iter().collect::<String>()
}

fn main() {
    let a = "thisisatest";
    let b = "testing123testing";

    println!("a: {}", a);
    println!("b: {}", b);

    {
        let result = lcs_r_string(a, b);
        println!("Recursive: {}", result);
    }
    {
        let result = lcs_string(a, b);
        println!("Dynamic Programming: {}", result);
    }
}
