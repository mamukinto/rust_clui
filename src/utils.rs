pub fn clear_canvas(canvas : &mut Vec<Vec<char>>) {
    for i in 0..canvas.len() {
        for j in 0..canvas[i].len() {
            canvas[i][j] = '`';
        }
    }
}

pub fn show_canvas(canvas: &Vec<Vec<char>>) {
    print!("\x1B[2J\x1B[1;1H");
    canvas.iter().for_each(|it| {
        //println!("{:?}", it);
        
        println!("{}", it.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ""))
    });
}

pub fn draw_rect(canvas: &mut Vec<Vec<char>>, x: usize, y: usize, height: usize, width: usize,brightness: char) {
    for horizontal in x..(x + width) {
        for vertical in y..(y + height) {
            canvas[horizontal][vertical] = brightness;
        }
    }
}

pub fn draw_circle(canvas: &mut Vec<Vec<char>>, center_x: usize, center_y: usize, radius: usize,brightness: char) {
    for horizontal in x..(x + width) {
        for vertical in y..(y + height) {
            if is_in_circle(horizontal,vertical,center_x,center_y,radius) {
                canvas[horizontal][vertical] = brightness;        
            }
        }
    }
}

fn is_in_circle(x: usize,y: usize,center_x: usize,center_y: usize,radius: usize) -> bool {
    // (center_x - x)^2 + (center_y - y)^2 = radius^2
    let r_sq_now = (center_x - x)*(center_x - x) + (center_y - y)*(center_y - y);
    if(r_sq_now <= radius*radius) {
        return true;
    }
    false
}