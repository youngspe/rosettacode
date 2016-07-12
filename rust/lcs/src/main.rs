// https://rosettacode.org/wiki/Longest_common_subsequence
// Translated from C#
fn lcs<F: Eq + Clone>(a: &[F], b: &[F]) -> Vec<F> {
    if a.len() == 0 || b.len() == 0 {
        return Vec::new();
    }

    let a_sub = &a[0..a.len() - 1];
    let b_sub = &b[0..b.len() - 1];

    if a[a.len() - 1] == b[b.len() - 1] {
        let mut s = lcs(a_sub, b_sub);
        s.push(a[a.len() - 1].clone());
        s
    } else {
        let (x, y) = (lcs(a, b_sub), lcs(a_sub, b));
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn lcs_string(a: &str, b: &str) -> String {
    let a_vec = a.chars().collect::<Vec<_>>();
    let b_vec = b.chars().collect::<Vec<_>>();

    lcs(&a_vec, &b_vec).into_iter().collect::<String>()
}

fn main() {
    let a = "thisisatest";
    let b = "testing123testing";

    let result = lcs_string(a, b);
    println!("{}", result);
}
