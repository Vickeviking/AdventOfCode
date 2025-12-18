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

    let mut ranges = fresh_ingredients
        .iter()
        .map(|(&start, &end)| (start, end))
        .collect::<Vec<_>>();

    ranges.sort_by_key(|&(start, _)| start);

    let mut sum = 0;
    let mut last_end = 0;

    for (start, end) in ranges {
        let start = start.max(last_end + 1);
        if start <= end {
            sum += end - start + 1;
            last_end = end;
        }
    }

    println!("{:?} ingredients where fresh!", sum);

    Ok(())
}
