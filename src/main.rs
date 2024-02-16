use std::io;

const SAVINGS_PERCENTAGE: f64 = 0.5;
const SPENDING_PERCENTAGE: f64 = 0.3;
const SURPLUS_PERCENTAGE: f64 = 0.2;


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

fn money_divider(a: f64) {

    let gross_income = &a;
    let savings = &*gross_income * SAVINGS_PERCENTAGE;
    let spending_budget = &*gross_income * SPENDING_PERCENTAGE;
    let surplus_budget = &*gross_income * SURPLUS_PERCENTAGE;

    println!("{}",format!("Your necessary savings are: E {:.2}", savings));
    println!("{}",format!("Your spending budget is:    E {:.2}", spending_budget));
    println!("{}",format!("Your surplus/fun budget is: E {:.2}", surplus_budget));

}

fn main() {
    println!("Hello, Siphesihle!");
    money_divider(user_input());
}
