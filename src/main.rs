use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::{thread, time};

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
                if x < 1 || y < 1 || x > 8 || y > 8 {
                    continue;
                }
                //println!("x{},y{}",x,y);
                let xi = x as i8;
                let yi = y as i8;

                let mut tot = 0;

                let mut i : i8 = xi-1;
                let mut j : i8 = yi-1;

                while i <= xi+1 {
                    while j <= yi+1 {
                        if i != xi || j != yi {
                            
                            let ii : usize = i as usize;
                            let ji : usize = j as usize;

                            if state[ii][ji] {
                                tot += 1;
                            }
                        }
                        j += 1;
                    }
                    j = yi-1;
                    i += 1;
                }
                buff[x][y] = (state[x][y] && tot == 2) || tot == 3;
                //println!("{}{}{}", x, y, tot);
            }
        }

        for (x, row) in buff.iter().enumerate() {
            for (y, _col) in row.iter().enumerate() {
                let chr = if buff[x][y] {"X"} else {"0"};
                print!("{}", chr);
                state[x][y] = buff[x][y];
            }
            print!("\n");
        }
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
        print!("{}[2J", 27 as char);

        //Clear buffer
        for (x, row) in state.iter().enumerate() {
            for (y, _col) in row.iter().enumerate() {
                buff[x][y] = false;
            }
        }
        
    }
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
