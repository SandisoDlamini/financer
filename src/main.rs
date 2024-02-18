use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::fs;

const SAVINGS_PERCENTAGE: f64 = 0.5;
const SPENDING_PERCENTAGE: f64 = 0.3;
const SURPLUS_PERCENTAGE: f64 = 0.2;

fn create_file() -> String {
    loop {
        
        let mut file_name = String::new();

        println!("Please input name of month to save to:");

        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read input");

        let file_name = file_name.trim().to_string();

        println!("Saving to: {}", &file_name);


        let mut choice_confirmation = String::new();

        println!("Are you sure you want to save to {}? [y/N]", file_name);

        io::stdin()
            .read_line(&mut choice_confirmation)
            .expect("Cannot save to file.");
        
        let choice_confirmation = choice_confirmation.trim();

        let choices_for = ["Y", "y", "YES", "yes"];
        let choices_against = ["N", "n", "NO", "no"];

        if choices_for.iter().any(|&i| i == choice_confirmation) {
            break file_name;
        }

        else if choices_against.iter().any(|&i| i == choice_confirmation) {
            continue;
        }

        else {
            continue;
        }
    }

}

fn open_file(filename: String) {

    let file_name = &filename;

    let file_path_buff = fs::canonicalize("./src/month_files/").expect("Cannot get path");
    let inter_file_path = file_path_buff.into_os_string()
        .into_string()
        .expect("cannot convert file path");
    let file_path = inter_file_path + "/" + file_name + ".txt";
    
    let mut file = File::create(file_path).expect("Failed to create file");
    let amount = money_divider(user_input());

    for i in amount.into_iter() {
        println!("{}", format!("{:.2}", &i));
        let j = format!("{:.2}", i);    
        file.write(j.as_bytes()).expect("Failed to write to file");
    }
}

fn user_input() -> f64 {
    
    loop {
        
        let mut income = String::new();

        println!("Please input amount of money earned:");

        io::stdin()
            .read_line(&mut income)
            .expect("Failed to read input");

        let income: f64 = match income.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You earned: {}", income);

        return income;

    }
}

fn money_divider(a: f64) -> Vec<f64> {

    let gross_income = &a;
    let savings = &*gross_income * SAVINGS_PERCENTAGE;
    let spending_budget = &*gross_income * SPENDING_PERCENTAGE;
    let surplus_budget = &*gross_income * SURPLUS_PERCENTAGE;

    println!("{}",format!("Your necessary savings are: E {:.2}", &savings));
    println!("{}",format!("Your spending budget is:    E {:.2}", &spending_budget));
    println!("{}",format!("Your surplus/fun budget is: E {:.2}", &surplus_budget));

    let expenditure = vec![savings, spending_budget, surplus_budget];

    return expenditure

}


fn main() {
    println!("Hello, User!");
    open_file(create_file());
}
