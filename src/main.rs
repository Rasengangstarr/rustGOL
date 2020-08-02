use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() {
    println!("Hello, world!");
    
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "wrong number of arguments provided! provide a filename only");
    let filename = &args[1];
    println!("reading file: {}", filename);

    let mut state = read_file(filename.to_string()).unwrap();

    
    let mut buff: [[bool; 10]; 10] = [[false; 10]; 10];

    loop {

        for (x, row) in state.iter().enumerate() {
            for (y, _col) in row.iter().enumerate() {
                if x < 1 || y < 1 || x > 9 || y > 9 {
                    continue;
                }
                let xi = x as i8;
                let yi = y as i8;

                let mut tot = 0;

                let i : i8 = xi-1;
                let j : i8 = yi-1;

                while i <= xi+1 {
                    while j <= yi+1 {
                        if i != 0 || j != 0 {
                            println!("{},{}",i,j);
                            let ii : usize = i as usize;
                            let ji : usize = i as usize;
                            
                            if state[ii][ji] {
                                tot += 1;
                            }
                        }
                    }
                }
                buff[x][y] = (state[x][y] && tot == 2) || tot == 3;
            }
        }

        println!("loop");

        for (x, row) in buff.iter().enumerate() {
            for (y, _col) in row.iter().enumerate() {
                let chr = if buff[x][y] {"X"} else {"0"};
                print!("{}", chr)
            }
            print!("\n");
        }
        println!("loop");
        
        
        state = buff;
        println!("loop");
    }
    println!("loop");


}

fn read_file(filename : String) -> io::Result<[[bool; 10]; 10]> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut init_state: [[bool; 10]; 10] = [[false; 10]; 10];

    for (y,line) in reader.lines().enumerate() {
        let l = line.unwrap();
        for (x,c) in l.chars().enumerate() {
            init_state[x][y] = c == '1';
        }
    }

    Ok(init_state)
}
