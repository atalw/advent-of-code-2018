struct Elf {
    curr_recipe: u8,
    curr_index: u32,
}

fn main() {
    a(293801);
}

fn a(input: usize) -> String{
    let mut scores: Vec<u8> = vec![3, 7];

    let mut index_one = 0;
    let mut index_two = 1;

    for _ in 0..input+10 {
        let recipe_one = scores[index_one];
        let recipe_two = scores[index_two];
        let mut sum = recipe_one + recipe_two;

        let mut digits: Vec<u8> = Vec::new();
        while sum >= 10 {
            digits.push(sum % 10);
            sum /= 10;
        }
        digits.push(sum);
        digits.reverse();

        scores.append(&mut digits);

        index_one = (index_one + (1 + recipe_one) as usize) % scores.len();
        index_two = (index_two + (1 + recipe_two) as usize) % scores.len();
    }

    let final_score: String = scores.get(input..input+10)
        .unwrap()
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!("{}", final_score);
    final_score
}

fn b(input: Vec<u8>) -> usize {

    todo!()

}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn examples() {
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

}
