fn check(
    inp: &[&str],
    i: usize,
    j: usize,
    u_l: bool,
    u: bool,
    u_r: bool,
    l: bool,
    r: bool,
    d_l: bool,
    d: bool,
    d_r: bool,
) -> u32 {
    let mut res: u32 = 0;

    if u_l {
        if inp[i - 1].chars().nth(j - 1).unwrap_or('.') == '*' {
            res += 1;
        }
    }

    if u {
        if inp[i - 1].chars().nth(j).unwrap_or('.') == '*' {
            res += 1;
        }
    }

    if u_r {
        if inp[i - 1].chars().nth(j + 1).unwrap_or('.') == '*' {
            res += 1;
        }
    }

    if l {
        if inp[i].chars().nth(j - 1).unwrap_or('.') == '*' {
            res += 1;
        }
    }

    if r {
        if inp[i].chars().nth(j + 1).unwrap_or('.') == '*' {
            res += 1;
        }
    }

    if d_l {
        if inp[i + 1].chars().nth(j - 1).unwrap_or('.') == '*' {
            res += 1;
        }
    }

    if d {
        if inp[i + 1].chars().nth(j).unwrap_or('.') == '*' {
            res += 1;
        }
    }

    if d_r {
        if inp[i + 1].chars().nth(j + 1).unwrap_or('.') == '*' {
            res += 1;
        }
    }

    res
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res_vec: Vec<String> = Vec::new();
    for (i, _) in minefield.iter().enumerate() {
        println!("GOT HERE1 {i}");
        let (mut u_l, mut u, mut u_r, mut l, mut r, mut d_l, mut d, mut d_r) =
            (false, false, false, false, false, false, false, false);
        let s: Vec<char> = minefield[i].chars().collect();
        let mut line_res = String::new();
        for j in 0..s.len() {
            println!("GOT HERE2 {j}");
            if minefield.len() == 1 {
                if j == 0 {
                    r = true;
                } else if j == s.len() - 1 {
                    l = true;
                } else {
                    r = true;
                    l = true;
                }
            } else if i == minefield.len() - 1 {
                if j == 0 {
                    u = true;
                    u_r = true;
                    r = true;
                } else if j == s.len() - 1 {
                    u = true;
                    u_l = true;
                    l = true;
                } else {
                    u_l = true;
                    u = true;
                    u_r = true;
                    l = true;
                    r = true;
                }
            } else if i == 0 {
                if j == 0 {
                    r = true;
                    d = true;
                    d_r = true;
                } else if j == s.len() - 1 {
                    l = true;
                    d = true;
                    d_l = true;
                } else {
                    l = true;
                    r = true;
                    d_l = true;
                    d = true;
                    d_r = true;
                }
            } else {
                if j == 0 {
                    u = true;
                    u_r = true;
                    r = true;
                    d = true;
                    d_r = true;
                } else if j == s.len() - 1 {
                    u = true;
                    u_l = true;
                    l = true;
                    d = true;
                    d_l = true;
                } else {
                    u_l = true;
                    u = true;
                    u_r = true;
                    l = true;
                    r = true;
                    d = true;
                    d_l = true;
                    d_r = true;
                }
            }
            if s[j] == '*' {
                line_res.push('*');
            } else {
                let point = check(minefield, i, j, u_l, u, u_r, l, r, d_l, d, d_r);
                if point == 0 {
                    line_res.push(s[j]);
                } else {
                    line_res.push_str(point.to_string().as_str());
                }
            }
        }
        res_vec.push(line_res);
    }

    println!("HERE");
    println!("{res_vec:?}");
    res_vec
}
