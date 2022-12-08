use std::{fs,};

fn read_file(filepath: &str) -> String{
    return fs::read_to_string(filepath).unwrap();
}

fn part_1(elves: &Vec<Vec<i32>>){
    let mut total_calories: Vec<i32> = Vec::new();
    for (_, elf) in elves.iter().enumerate() {
        let mut sum_calories = 0;
        for (_, item) in elf.iter().enumerate() { 
            sum_calories += item;
        }
        total_calories.push(sum_calories);

        // println!("{:?}", elf);
    }
    // println!("{:?}", total_calories)
    println!("Part 1 Solution: {:?}", total_calories.iter().max());

    part_2(&mut total_calories);

}

fn part_2(calories: &mut Vec<i32>){
    calories.sort();
    calories.reverse();

    println!("Part 2 Solution: {}", (calories[0]+calories[1]+calories[2]));
}

fn main() {
    let filepath = "data/dec1.txt";
    let data = read_file(filepath);
    let df: Vec<&str> = data.split("\n\n").collect();

    let elves: Vec<Vec<i32>> = df.iter().map(|&backpack| {
        let food: Vec<&str> = backpack.split("\n").collect();
        let calories: Vec<i32> = food.iter().map(|&items| {
            return items.parse::<i32>().unwrap();
        }).collect();
        return calories;
    }).collect();

    part_1(&elves);    
}
