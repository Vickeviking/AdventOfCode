use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    u64,
};

fn process_line(str: String) -> Result<u16, std::io::Error> {
    let str_arr: Vec<String> = str.split('x').map(|part| part.to_string()).collect();

    if str_arr.len() != 3 {
        return Err(io::ErrorKind::InvalidData.into());
    }

    let length = str_arr[0].parse::<u16>().unwrap();
    let width = str_arr[1].parse::<u16>().unwrap();
    let height = str_arr[2].parse::<u16>().unwrap();
    let perimeters: Vec<u16> = vec![
        2 * length + 2 * width,
        2 * width + 2 * height,
        2 * height + 2 * length,
    ];

    let mut ribbon = perimeters[0];
    for s in perimeters {
        if s < ribbon {
            ribbon = s;
        }
    }

    let area = length * width * height;
    ribbon += area;

    return Ok(ribbon);
}

fn main() -> io::Result<()> {
    let path = Path::new("src/test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut ft_of_ribbon: u64 = 0;

    for line in reader.lines() {
        match line {
            Ok(l) => ft_of_ribbon += process_line(l)? as u64,
            Err(e) => eprintln!("error reading line:{}", e),
        }
    }

    println!("Elfs need {} square feet of ribbon", ft_of_ribbon);
    return Ok(());
}
