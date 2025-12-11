use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    u64,
};

fn process_line(str: String) -> Result<u16, std::io::Error> {
    let str_arr: Vec<String> = str.split('x').map(|part| part.to_string()).collect();
    let mut area: u16 = 0;

    if str_arr.len() != 3 {
        return Err(io::ErrorKind::InvalidData.into());
    }

    let length = str_arr[0].parse::<u16>().unwrap();
    let width = str_arr[1].parse::<u16>().unwrap();
    let height = str_arr[2].parse::<u16>().unwrap();
    let sides: Vec<u16> = vec![length * width, width * height, height * length];

    let mut smallest_area = sides[0];
    //smallest area
    for s in sides {
        if s < smallest_area {
            smallest_area = s;
        }
        area += s * 2;
    }
    area += smallest_area;

    return Ok(area);
}

fn main() -> io::Result<()> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut sqfeet_of_paper: u64 = 0;

    for line in reader.lines() {
        match line {
            Ok(l) => sqfeet_of_paper += process_line(l)? as u64,
            Err(e) => eprintln!("error reading line:{}", e),
        }
    }

    println!("Elfs need {} square feet of paper", sqfeet_of_paper);
    return Ok(());
}
