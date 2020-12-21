#![feature(is_sorted)]

use common::aoc::{load_input, print_result, print_time, run_many, run_once};
use smallvec::SmallVec;

fn main() {
    let (input, dur_load) = run_once(|| load_input("day21"));

    print_time("Load", dur_load);

    let (input, dur_parse) = run_many(10000, || Input::parse(&input));
    let ((res_part1, inerts), dur_part1) = run_many(10000, || part1(&input));
    let (res_part2, dur_part2) = run_many(10000, || part2(&input, &inerts));

    print_result("P1", res_part1);
    print_result("P2", &res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);

    assert_eq!(res_part1, 2436);
    assert_eq!(res_part2.as_str(), "dhfng,pgblcd,xhkdc,ghlzj,dstct,nqbnmzx,ntggc,znrzgs")
}

fn part1(input: &Input) -> (u64, Vec<usize>) {
    let mut result = 0;
    let total_mask = (1 << input.allergens.len()) - 1;
    let mut inerts = Vec::with_capacity(16);

    for i in 0..input.ingredients.len() {
        let mut count = 0;
        let mut mask = total_mask;
        for food in input.foods.iter() {
            if food.contains_ingredient(i) {
                count += 1;
            } else {
                mask &= !food.allergen_mask;
            }
        }

        if mask == 0 {
            result += count;
            inerts.push(i);
        }
    }

    (result, inerts)
}

fn part2(input: &Input, inerts: &[usize]) -> String {
    // Identify not inerts
    let mut not_inerts: SmallVec<[usize; 16]> = SmallVec::with_capacity(input.allergens.len());
    let mut prev_inert = 0;
    for (i, inert) in inerts.iter().cloned().enumerate() {
        if i == 0 && inert != 0 {
            not_inerts.push(0);
        }

        while inert > prev_inert + 1 {
            not_inerts.push(prev_inert + 1);
            prev_inert += 1;
        }

        prev_inert = inert;
    }


    // Identify allergens
    let mut bad_ingredients = vec![0; not_inerts.len()];
    let mut has_found_baddie = vec![false; not_inerts.len()];
    let mut baddie_count = 0;
    let mut has_found_ingredient = vec![false; not_inerts.len()];

    while baddie_count < not_inerts.len() {
        for (i, allergen_index) in input.allergen_order.iter().cloned().enumerate() {
            if has_found_baddie[i] {
                continue;
            }

            let allergen_mask = 1 << allergen_index;
            let mut candidates: SmallVec<[usize; 4]> = SmallVec::default();

            for (j, ingredient_index) in not_inerts.iter().cloned().enumerate() {
                if has_found_ingredient[j] {
                    continue;
                }

                let mut failed = false;
                for food in input.foods.iter() {
                    if !food.contains_ingredient(ingredient_index) {
                        if food.allergen_mask & allergen_mask == allergen_mask {
                            failed = true;
                            break;
                        }
                    }
                }

                if !failed {
                    candidates.push(j);
                }
            }

            if candidates.len() == 1 {
                let c = candidates[0];

                bad_ingredients[i] = not_inerts[c];
                has_found_ingredient[c] = true;
                has_found_baddie[i] = true;
                baddie_count += 1;
            }
        }
    }

    // Assemble string
    let mut res = String::with_capacity(16 + not_inerts.len() * 8);
    for (i, bad_ingredient) in bad_ingredients.iter().enumerate() {
        if i > 0 {
            res.push(',');
        }

        res.push_str(input.ingredients[*bad_ingredient]);
    }

    res
}

struct Input<'a> {
    allergens: Vec<&'a str>,
    ingredients: Vec<&'a str>,
    allergen_order: Vec<usize>,
    foods: Vec<Food>
}

impl<'a> Input<'a> {
    fn parse(input_str: &'a str) -> Input {
        let mut input = Input{
            allergens: Vec::with_capacity(64),
            ingredients: Vec::with_capacity(256),
            allergen_order: Vec::new(),
            foods: Vec::with_capacity(64),
        };

        for line in input_str.lines() {
            if line.is_empty() {
                continue;
            }

            if line.ends_with(')') {
                let para_pos = line.find('(').unwrap();

                let mut ingredients = SmallVec::new();
                for ingredient in line[..para_pos - 1].split(' ') {
                    let ingredient_index = if let Some(index) = input.ingredients.iter().position(|p| *p == ingredient) {
                        index
                    } else {
                        input.ingredients.push(ingredient);
                        input.ingredients.len() - 1
                    };

                    ingredients.push(ingredient_index);
                }

                let mut allergen_mask = 0u64;
                for allergen in line[para_pos + 10..line.len() - 1].split(' ') {
                    let allergen = allergen.trim_end_matches(',');
                    let allergen_index = if let Some(index) = input.allergens.iter().position(|p| *p == allergen) {
                        index
                    } else {
                        input.allergens.push(allergen);
                        input.allergens.len() - 1
                    };

                    allergen_mask |= 1 << allergen_index;
                }

                input.foods.push(Food{ingredients, allergen_mask});
            } else {
                let mut ingredients = SmallVec::new();
                for ingredient in line.split(' ') {
                    let ingredient_index = if let Some(index) = input.ingredients.iter().position(|p| *p == ingredient) {
                        index
                    } else {
                        input.ingredients.push(ingredient);
                        input.ingredients.len() - 1
                    };

                    ingredients.push(ingredient_index);
                }

                input.foods.push(Food{ingredients, allergen_mask: 0});
            }
        }

        let mut allergen_order: Vec<usize> = (0..input.allergens.len()).collect();
        allergen_order.sort_by(|a, b| {
            input.allergens[*a].cmp(input.allergens[*b])
        });

        input.allergen_order = allergen_order;

        input
    }
}

struct Food {
    ingredients: SmallVec<[usize; 16]>,
    allergen_mask: u64,
}

impl Food {
    fn contains_ingredient(&self, ingredient_index: usize) -> bool {
        self.ingredients.iter().find(|i| **i == ingredient_index).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_1: &str = "sqjhc fvjkl (contains soy)
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc mxmxvkd sbzzf (contains fish)
";

    #[test]
    fn test_part1() {
        let input = Input::parse(EXAMPLE_1);
        println!("{:?}", input.ingredients);
        println!("{:?}", input.allergens);
        let (res_part1, _) = part1(&input);
        assert_eq!(res_part1, 5);
    }

    #[test]
    fn test_part2() {
        let input = Input::parse(EXAMPLE_1);
        let (res_part1, inerts) = part1(&input);
        assert_eq!(res_part1, 5);
        assert_eq!(part2(&input, &inerts), "mxmxvkd,sqjhc,fvjkl".to_owned());
    }
}