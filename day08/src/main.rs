use std::fs;

fn main() {
    let input = "input.txt";
    let contents = fs::read_to_string(input).expect("Error reading input");
    let image = contents.chars().map(|item| item.to_digit(10).unwrap()).collect::<Vec<_>>();

    //println!("{:?}", memory);
    //println!("{:#?}", memory);
    part1(&image, 25, 6);
}

fn part1(image_data: &Vec<u32>, width: usize, height: usize) {
    let layer_size = width * height;
    let layers = image_data.chunks_exact(layer_size).collect::<Vec<_>>();
    let mut least_zeroes = std::i32::MAX;
    let mut checksum = 0;
    let mut image: Vec<u32> = Vec::new();
    image.resize(layer_size, 2);

    for layer in &layers {
        let zeroes = layer.to_vec().into_iter().fold(0, |acc, item| if item == 0 { acc + 1 } else { acc });
        if zeroes < least_zeroes {
            least_zeroes = zeroes;
            let ones = layer.to_vec().into_iter().fold(0, |acc, item| if item == 1 { acc + 1 } else { acc });
            let twos = layer.to_vec().into_iter().fold(0, |acc, item| if item == 2 { acc + 1 } else { acc });
            checksum = ones * twos;
            //println!("layer: {:?},  checksum: {:?}", zeroes, checksum);
            //println!("zeroes: {:?}, ones: {:?}, twos: {:?}", zeroes, ones, twos)
        }
        for (index, digit) in layer.iter().enumerate() {
            let digit_clone = digit.clone();
            if image.get(index).unwrap().clone() == 2 {
                if digit_clone == 0 || digit_clone == 1 {
                    image[index] = digit_clone;
                }
            }
        }
    }
    println!("Part1: {:?}", checksum);
    println!("Part2:");
    for y in 0..height {
        for x in 0..width {
            //print!("{}", x+(y*width));
            print!("{}", if image[x+(y*width)] == 1 { "#"} else { " "})
        }
        println!();
    }

}