pub fn count(lines: &[&str]) -> u32 {
    let mut ans = 0u32;
    let r = lines.len();
    if r == 0 {
        return ans;
    }
    let c = lines[0].len();
    if c < 2 {
        return 0;
    }

    for k in 0..r {
        for l in 0..c {
            if lines[k].chars().nth(l).unwrap() != '+' {
                continue;
            }

            for i in 0..k {
                for j in 0..l {
                    if lines[i].chars().nth(j).unwrap() != '+' {
                        continue;
                    }

                    if lines[i].chars().nth(l).unwrap() != '+' {
                        continue;
                    }

                    if lines[k].chars().nth(j).unwrap() != '+' {
                        continue;
                    }

                    // All horizontals are dash
                    let mut valid = true;
                    for h in j+1..l {
                        if lines[i].chars().nth(h).unwrap() != '-' && lines[i].chars().nth(h).unwrap() != '+' {
                            valid = false;
                            break;
                        }
                        if lines[k].chars().nth(h).unwrap() != '-' && lines[k].chars().nth(h).unwrap() != '+'{
                            valid = false;
                            break;
                        }
                    }
                    if !valid {
                        continue;
                    }

                    // All vertical are |
                    for v in i+1..k {
                        if lines[v].chars().nth(j).unwrap() != '|' && lines[v].chars().nth(j).unwrap() != '+' {
                            valid = false;
                            break;
                        }

                        if lines[v].chars().nth(l).unwrap() != '|' && lines[v].chars().nth(l).unwrap() != '+' {
                            valid = false;
                            break;
                        }
                    }

                    if !valid {
                        continue;
                    }
                    ans += 1;
                }
            }
        }
    }
    ans
}
