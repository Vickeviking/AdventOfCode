use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let file = File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = io::BufReader::new(file);

    let lines = reader
        .lines()
        .map(|x| x.expect("Valid strings due to proj desc"))
        .collect::<Vec<String>>();

    let mut iter = lines.split(|l| l.is_empty());
    let fresh_ingredients_ranges = iter.next().unwrap().to_vec();
    let available_ingredients = iter
        .next()
        .unwrap()
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut fresh_ingredients = BTreeMap::<u64, u64>::new();

    //insert start as key, and end as value
    for range in fresh_ingredients_ranges {
        let (a, b) = range.split_once("-").unwrap();
        let (a, b) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
        let entry = fresh_ingredients.entry(a).or_insert(b);
        *entry = (*entry).max(b);
    }

    let mut num_fresh = 0;

    for i in available_ingredients {
        num_fresh += fresh_ingredients.range(..=i).any(|(_, &end)| i <= end) as u64;
    }

    println!("{:?} ingredients where fresh!", num_fresh);

    Ok(())
}
