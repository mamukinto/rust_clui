use::text_io::scan;
use colored::*;
pub fn start() {
    let brightness = "`^,:;Il!i~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let curr_brightness = 0;
    let mut width = 82;
    let mut height = 30;
    let mut amount = 27;
    let mut update_interval_in_ms = 250;
     //show_canvas(&canvas);
   //draw_rect(&mut canvas,10,10,3,3);
    //show_canvas(&canvas);

    //println!("{:?}",generate_random_vec());
    loop {
        println!("Choose sorting algorithm:");
        println!("1. Bubble sort");
        println!("2. Merge sort");
        println!("3. Random data");
        println!("0. Settings");
        let choice: i32;
        scan!("{}",choice);
        match choice {
            0 => (width,height,amount,update_interval_in_ms) = settings(width, height, amount,update_interval_in_ms),
            1 => {
                let mut canvas = vec![vec![brightness.chars().nth(curr_brightness).unwrap(); width]; height];
                animation_loop(&mut canvas,choice,amount,update_interval_in_ms);
            },
            3 => {
                let mut canvas = vec![vec![brightness.chars().nth(curr_brightness).unwrap(); width]; height];
                animation_loop(&mut canvas,choice,amount,update_interval_in_ms);
            },
            _ => start()
        }
    }
    
}

fn settings(w: usize, h: usize, a: i32,i: u64) -> (usize,usize,i32,u64) {
    println!("Choose option:");
    println!("1. Change width of canvas");
    println!("2. Change height of canvas");
    println!("3. Change number of data points");
    println!("4. Change interval time in ms");
    println!("0. Back");  
    let mut width: usize = w;
    let mut height: usize = h;
    let mut amount: i32 = a;
    let mut interval: u64 = i;
    let choice: i32;
    scan!("{}",choice);
    match choice {
        1 => width = get_new_width(w),
        2 => height = get_new_height(h),
        3 => amount = get_new_amount(a,w),
        4 => interval = get_new_interval(i),
        0 => start(),
        _ => start()
    }
    (width,height,amount,interval)
}
fn get_new_interval(i: u64) -> u64 {
    println!("Current interval in ms:{}",i);
    println!("{}","Enter new interval in ms:".green());
    let ni : u64;
    scan!("{}",ni);
    ni

}
fn get_new_amount(a: i32,w: usize) -> i32 {
    println!("Current amount:{}, Maximum amount:{}",a,(w/3));
    println!("{}","Enter new amount:".green());
    let na : i32;
    scan!("{}",na);
    na
}

fn get_new_height(h: usize) -> usize {
    println!("Current height:{}",h);
    println!("{}","Enter new height:".green());
    let nh : usize;
    scan!("{}",nh);
    nh
}
fn get_new_width(w: usize) -> usize {
    println!("Current width:{}",w);
    println!("{}","Enter new width:".green());
    let nw : usize;
    scan!("{}",nw);
    nw
}

fn clear_canvas(canvas : &mut Vec<Vec<char>>) {
    for i in 0..canvas.len() {
        for j in 0..canvas[i].len() {
            canvas[i][j] = '`';
        }
    }
}
// fn draw_random_data(canvas: &mut Vec<Vec<char>>) -> Vec<usize>{
//     clear_canvas(canvas);
//     let data = generate_random_vec();
//     let mut x = 0;
//     for point in data {
//         let zxc = point.try_into().unwrap();
//         draw_rect(canvas, 30-zxc, x, 2,zxc);
//         x = x + 3;
//     }
//     show_canvas(&canvas);
//     data
// }

use core::time;
use std::{thread};

use rand::Rng;
//use timer::oneshot_ms;

fn draw_data(canvas: &mut Vec<Vec<char>>,data: &Vec<usize>) {
    clear_canvas(canvas);
    let mut x = 0;
    let h = canvas.len();
    for point in data {
        draw_rect(canvas, h-point, x, 2,*point);
        x = x + 3;
    }
    show_canvas(&canvas);
}

fn is_sorted(data: &Vec<usize>) -> bool {
    for i in 1..data.len() {
        if data[i-1] > data[i] {
            return false
        }
    }
    true
}



    // test the example with `cargo run --example most_simple`
    // fn zxc() {
    //     // TADAA!
    //     println!("{} {} !", "it".green(), "works".blue().bold());
    // }



fn animation_loop(canvas : &mut Vec<Vec<char>>,choice: i32,amount: i32,update_interval_in_ms: u64) {
    let mut data = generate_random_vec(amount,canvas.len());
    draw_data(canvas,&data);
    thread::sleep(time::Duration::from_millis(500));
    loop {
        if choice == 1 {
            if is_sorted(&data) {
                break;
            } else {
                bubble_sort_step(&mut data);
            }
        } else if choice == 2 {
            break;
        } else if choice == 3 {
            data = generate_random_vec(amount,canvas.len());
        } else {
            break;
        }
        draw_data(canvas,&data);
        thread::sleep(time::Duration::from_millis(update_interval_in_ms));
        
    }
}

fn bubble_sort_step(data: &mut Vec<usize>) {
    for i in 1..data.len() {
        if data[i-1] > data[i] {
            let temp = data[i-1];
            data[i-1] = data[i];
            data[i] = temp;
        }
    }
}
fn generate_random_vec(amount: i32,h: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    let vals: Vec<usize> = (0..amount).map(|_| rng.gen_range(0,h)).collect();

    vals
}   

// fn bubble_sort_step(data: &mut Vec<i32>) {
    
// }

fn show_canvas(canvas: &Vec<Vec<char>>) {
    print!("\x1B[2J\x1B[1;1H");
    canvas.iter().for_each(|it| {
        //println!("{:?}", it);
        println!("{}", it.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ""))
    });
}

fn draw_rect(canvas: &mut Vec<Vec<char>>, x: usize, y: usize, height: usize, width: usize) {
    for horizontal in x..(x + width) {
        for vertical in y..(y + height) {
            canvas[horizontal][vertical] = '@';
        }
    }
}