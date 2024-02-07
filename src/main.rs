use std::io::{self, Write};

use scanner_rust::ScannerAscii;

fn main() {
    print!("Please input one integer ");
    io::stdout().flush().unwrap();

    let mut sc = ScannerAscii::new(io::stdin());

    let to_insert = {
        loop {
            match sc.next_isize() {
                Ok(i) => break i.unwrap_or(0),
                
                Err(_) => {
                    print!("Re-input a and b: ");
                    io::stdout().flush().unwrap();
                }
            }
        }
    };

    let mut sorted_arr: &mut Vec<isize> = &mut vec![1,2,3,4,5,6,7,8,9];

    let arr_len = sorted_arr.len();

    let mut upper_bound = arr_len - 1;
    let mut lower_bound = 0;

    println!("sorted_arr before {:?}\n", sorted_arr);

    'main_loop: while lower_bound <= upper_bound  {
        let mut midpoint = (upper_bound + lower_bound) / 2 as usize;
        
        let mut val_at_midpoint: isize = sorted_arr[midpoint];

        if to_insert == val_at_midpoint {
            sorted_arr.push(11);

            // print!("{:?}\n", sorted_arr);

            for i in (midpoint..arr_len).rev() {
                print!("mid = {} . i = {} ", midpoint, i);
                sorted_arr[i+1] = sorted_arr[i];

                if i == midpoint {
                    break 'main_loop;
                }

            }

            sorted_arr[midpoint] = to_insert as isize;

        } else if to_insert < val_at_midpoint {
            upper_bound = midpoint - 1;
            
        } else if to_insert > val_at_midpoint {
            lower_bound = midpoint + 1;
        }
    }


    println!("\nto_insert {}\n sorted_arr{:?}", to_insert, sorted_arr);
}
