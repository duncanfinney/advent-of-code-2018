use itertools::Itertools;

pub fn solve() {
    let answer = part_one(909441);
    println!("answer={}", answer);

    let answer = part_two(909441);
    println!("answer={}", answer);
}

fn part_one(input: usize) -> String {
    let mut seq = vec!['3', '7'];
    let mut left: usize = 0;
    let mut right: usize = 1;

    while seq.len() < (input + 10 + 3) {
        let left_val = (*seq.get(left).unwrap() as u32) - ('0' as u32);
        let right_val = (*seq.get(right).unwrap() as u32) - ('0' as u32);
        //        dbg!(left_val); dbg!(right_val);

        for c in (left_val + right_val).to_string().chars() {
            seq.push(c);
        }

        left = (left + left_val as usize + 1) % seq.len();
        right = (right + right_val as usize + 1) % seq.len();

        //        debug_print(&seq, left, right);
    }

    seq[input..input + 10].iter().collect()
}

// TODO: remove copy/paste from part one
fn part_two(input: usize) -> usize {
    let mut seq = vec!['3', '7'];
    let mut left: usize = 0;
    let mut right: usize = 1;

    let mut search_idx: usize = 0;

    while seq.len() < 20500000 {
        search_idx += 1;


        let left_val = (*seq.get(left).unwrap() as u32) - ('0' as u32);
        let right_val = (*seq.get(right).unwrap() as u32) - ('0' as u32);
        //        dbg!(left_val); dbg!(right_val);

        for c in (left_val + right_val).to_string().chars() {
            seq.push(c);
        }

        left = (left + left_val as usize + 1) % seq.len();
        right = (right + right_val as usize + 1) % seq.len();

        //        debug_print(&seq, left, right);
    }

    let as_vec :Vec<_> = input.to_string().chars().collect();
    for (i, chars) in seq.windows(as_vec.len()).enumerate() {
       if chars == &as_vec[..] {
           return i;
       }
    }
    0
}

fn debug_print(seq: &Vec<char>, left: usize, right: usize) {
    for (idx, c) in seq.iter().enumerate() {
        if idx == left {
            print!("({}) ", c);
        } else if idx == right {
            print!("[{}] ", c);
        } else {
            print!(" {} ", c);
        }
    }
    print!("\n");
}
