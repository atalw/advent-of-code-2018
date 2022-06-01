fn main() {
    let score = a(293801);
    println!("part 1: {}", score);
    let idx = b(vec![2, 9, 3, 8, 0 ,1]);
    println!("part 2: {}", idx);
}

macro_rules! step {
    ($scores: expr, $index_one: expr, $index_two: expr) => {
        let recipe_one = $scores[$index_one];
        let recipe_two = $scores[$index_two];
        let mut sum = recipe_one + recipe_two;

        let mut digits: Vec<u8> = Vec::new();
        while sum >= 10 {
            digits.push(sum % 10);
            sum /= 10;
        }
        digits.push(sum);
        digits.reverse();

        $scores.append(&mut digits);

        $index_one = ($index_one + (1 + recipe_one) as usize) % $scores.len();
        $index_two = ($index_two + (1 + recipe_two) as usize) % $scores.len();
    };
}

fn a(input: usize) -> String {
    let mut scores: Vec<u8> = vec![3, 7];
    let mut index_one = 0;
    let mut index_two = 1;

    for _ in 0..input+10 {
        step!(scores, index_one, index_two);
    }

    let final_score: String = scores.get(input..input+10)
        .unwrap()
        .iter()
        .map(|x| x.to_string())
        .collect();
    final_score
}

fn b(input: Vec<u8>) -> usize {
    let mut scores: Vec<u8> = vec![3, 7];
    let mut index_one = 0;
    let mut index_two = 1;

    let mut idx = 0;
    loop {
        step!(scores, index_one, index_two);
        if scores.len() <= input.len() { continue }
        let new_idx = scores.len() - input.len();
        for i in idx..=new_idx {
            match scores.get(i..i+input.len()) {
                Some(final_score) if final_score == input => return i,
                _ => continue
            }
        }
        idx = new_idx;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn examples_part_1() {
        let test_vectors = [
            (9, "5158916779"),
            (5, "0124515891"),
            (18, "9251071085"),
            (2018, "5941429882"),
        ];
        for v in test_vectors {
            assert_eq!(a(v.0), v.1);
        }
    }

    #[test]
    fn examples_part_2() {
        let test_vectors = [
            (vec![5, 1, 5, 8, 9, 1], 9),
            (vec![0, 1, 2, 4, 5, 1], 5),
            (vec![9, 2, 5, 1, 0, 7, 1], 18),
            (vec![5, 9, 4, 1, 4], 2018),
        ];
        for v in test_vectors {
            assert_eq!(b(v.0), v.1);
        }
    }

}
