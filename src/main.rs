use std::str::FromStr;
extern crate termion;

use termion::{color, style};

const WIDTH:i32 = 60;
const HEIGHT:i32 = 50;
fn main() {

    let mut points:[u8;(WIDTH*HEIGHT * 2) as usize] = [0;(WIDTH*HEIGHT * 2) as usize];
    let mut points_copy:[u8;(WIDTH*HEIGHT * 2) as usize] = [0;(WIDTH*HEIGHT * 2) as usize];
    // points.iter_mut().for_each(|m| *m = 0);
    
    println!("Hello, world of life!");
    for _i in 0..(WIDTH*2 + 5) as usize{
        print!("=");
    }
    println!();
    for i in 0..HEIGHT{
        print!("|| ");
        for j in 0..WIDTH{
            if points[(i*WIDTH+j)as usize] == 1{
                print!("{}{} ",color::Fg(color::Red),points[(i*WIDTH+j)as usize]);
            }
            print!("{} ",points[(i*WIDTH+j) as usize]);
        }
        println!("||");
    }
    for _i in 0..(WIDTH*2 + 5) as usize{
        print!("=");
    }
    println!();

    let mut is_adding = true;
    while is_adding == true{
        let mut line = String::new();
        println!("Enter point [x:y][{}:{}]:",WIDTH,HEIGHT);
        std::io::stdin().read_line(&mut line).unwrap();
        let args:Vec<&str> = line.split(":").collect();
        let x:u8 = u8::from_str(&args[0]).expect("sussy baka 1") - 1;
        let mut y:u8 = 0;
        match args[1].len(){
            2usize => {y = u8::from_str(&args[1][0..1]).expect("sussy baka 2") - 1;},
            3 => {y = u8::from_str(&args[1][0..2]).expect("sussy baka 3") - 1;}
            _ => ()
        }
        // let y:u8 = u8::from_str(&args[1][0..1]).expect("sussy baka 2");
   
        points[((y as u32 * WIDTH as u32)+x as u32) as usize] = 1;

        for _i in 0..(WIDTH*2 + 5) as usize{
            print!("=");
        }
        println!();
        for i in 0..HEIGHT{
            print!("|| ");
            for j in 0..WIDTH{
                if points[(i*WIDTH+j) as usize] == 1{
                    print!("{}{}{} ",color::Fg(color::LightMagenta),style::Bold,points[(i*WIDTH+j)as usize]);
                }
                else{
                    print!("{}{}{} ",color::Fg(color::White),style::Bold,points[(i*WIDTH+j)as usize]);
                }
            }
            println!("||");
        }
        for _i in 0..(WIDTH*2 + 5) as usize{
            print!("=");
        }
        println!();
        println!("Do you want to add a point? [type anytching and enter to cancel]");
        let mut line = String::new();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        // std::io::stdin().read_line(&mut line).unwrap();
        // if line.as_str() == "y" || line.as_str() == "N" || line.as_str() == "no" || line.as_str() == "No" || line.as_str() == "NO"{
        //     is_adding = false;
        // }
        // println!("{} {}",b1,b1.to_string().as_str());
        match b1.to_string().as_str(){
            "1" => {},
            "2" => {is_adding=false},
            _=> {is_adding=false}
        }
    }
    loop {
            
        // Births: Each dead cell adjacent to exactly three live neighbors will become live in the next generation.
        // Death by isolation: Each live cell with one or fewer live neighbors will die in the next generation.
        // Death by overcrowding: Each live cell with four or more live neighbors will die in the next generation.
        // Survival: Each live cell with either two or three live neighbors will remain alive for the next generation.

        points_copy = points.clone();
        
        for i in 0..HEIGHT {
            for j in 0..WIDTH{
                // if points[(i*WIDTH+j) as usize] == 1{
                //     for k in get_cells(j, i).iter(){
                //             // println!("{}",k);
                //             if k > &-5{
                //                 points_copy[*k as usize] = 1;
                //             } 
                //         }
                // }


                let mut live_counter:u8 = 0;
                let xx = j;
                let yy = i;
                for k in get_cells(xx, yy).iter(){
                    // println!("{}",k);
                    if k > &-5{
                        if points[*k as usize] == 1{
                            live_counter += 1;
                        }
                    }
                }
                if points[(yy*WIDTH+xx) as usize] == 0{
                    if live_counter == 3{
                        points_copy[(yy*WIDTH+xx) as usize] = 1;
                    }
                }
                else{
                    if live_counter <= 1 || live_counter >= 4{
                        points_copy[(yy*WIDTH+xx) as usize] = 0;
                    }
                }


                // println!("{}",live_counter);
            }
        }
        
        for _i in 0..(WIDTH*2 + 5) as usize{
            print!("=");
        }
        println!();
        for i in 0..HEIGHT{
            print!("|| ");
            for j in 0..WIDTH{
                if points[(i*WIDTH+j) as usize] == 1{
                    print!("{}{}{} ",color::Fg(color::LightMagenta),style::Bold,points[(i*WIDTH+j)as usize]);
                }
                else{
                    print!("{}{}{} ",color::Fg(color::White),style::Bold,points[(i*WIDTH+j)as usize]);
                }
            }
            println!("||");
        }
        for _i in 0..(WIDTH*2 + 5) as usize{
            print!("=");
        }
        println!();
        std::io::stdin().read_line(&mut "line".to_string()).unwrap();
        
        points = points_copy.clone();
    }
}
fn get_cells(x:i32,y:i32) -> [i32;8]{
    let mut cells:[i32;8] = [-10;8];
    // let x = x-1;
    // let y = y-1;
    if x != 0 {
        cells[0] = y*WIDTH+(x-1);
    }
    if x != WIDTH - 1 {
        cells[1] = y*WIDTH+(x+1);
    }
    if y != 0 {
        cells[2] = (y-1)*WIDTH+x;
    }
    if x != HEIGHT - 1 {
        cells[3] = (y+1)*WIDTH+x;
    }
    if x != 0 && y != 0{
        cells[4] = (y-1)*WIDTH+(x-1);
    }
    if x != 0 && y != HEIGHT - 1{
        cells[5] = (y+1)*WIDTH+(x-1);
    }
    if x != WIDTH - 1 && y != 0{
        cells[6] = (y-1)*WIDTH+(x+1);
    }
    if x != WIDTH - 1 && y != HEIGHT - 1{
        cells[7] = (y+1)*WIDTH+(x+1);
    }
    return cells;
}