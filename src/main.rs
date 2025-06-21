use std::io::{self, Write};
use cical::*;

fn main() {
    println!("=== Compound Interest Calculator ===\n");
    
    loop {
        println!("Choose an option:");
        println!("1. Calculate compound interest");
        println!("2. Calculate compound interest with monthly contributions");
        println!("3. Calculate time to reach target amount");
        println!("4. Calculate required principal for target amount");
        println!("5. Generate year-by-year breakdown");
        println!("6. Exit");
        println!("7. Calculate weekly compounding with yearly tax (trader scenario)");
        print!("\nEnter your choice (1-7): ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        
        match choice {
            "1" => calculate_basic_interest(),
            "2" => calculate_interest_with_contributions(),
            "3" => calculate_time_to_target_interactive(),
            "4" => calculate_principal_for_target_interactive(),
            "5" => generate_breakdown_interactive(),
            "6" => {
                println!("Goodbye!");
                break;
            }
            "7" => calculate_weekly_with_tax_interactive(),
            _ => println!("Invalid choice. Please try again.\n"),
        }
    }
}

fn get_float_input(prompt: &str) -> f64 {
    loop {
        print!("{}: ", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn get_u32_input(prompt: &str) -> u32 {
    loop {
        print!("{}: ", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<u32>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid whole number."),
        }
    }
}

fn calculate_basic_interest() {
    println!("\n--- Basic Compound Interest Calculation ---\n");
    
    let principal = get_float_input("Enter principal amount ($)");
    let annual_rate = get_float_input("Enter annual interest rate (as decimal, e.g., 0.05 for 5%)");
    let compounds_per_year = get_u32_input("Enter number of times interest is compounded per year (1=annually, 12=monthly, 365=daily)");
    let years = get_float_input("Enter number of years");
    
    let params = CompoundInterestParams {
        principal,
        annual_rate,
        compounds_per_year,
        years,
    };
    
    let result = calculate_compound_interest(&params);
    
    println!("\n=== Results ===");
    println!("Initial Principal: {}", format_currency(result.principal));
    println!("Annual Interest Rate: {}", format_percentage(params.annual_rate));
    println!("Compounding Frequency: {} times per year", params.compounds_per_year);
    println!("Time Period: {:.1} years", params.years);
    println!("Final Amount: {}", format_currency(result.final_amount));
    println!("Total Interest Earned: {}", format_currency(result.total_interest));
    println!("Effective Annual Rate: {}", format_percentage(result.effective_annual_rate));
    println!("Growth Factor: {:.2}x", result.final_amount / result.principal);
    println!();
}

fn calculate_interest_with_contributions() {
    println!("\n--- Compound Interest with Monthly Contributions ---\n");
    
    let principal = get_float_input("Enter initial principal amount ($)");
    let annual_rate = get_float_input("Enter annual interest rate (as decimal, e.g., 0.05 for 5%)");
    let compounds_per_year = get_u32_input("Enter number of times interest is compounded per year (1=annually, 12=monthly, 365=daily)");
    let years = get_float_input("Enter number of years");
    let monthly_contribution = get_float_input("Enter monthly contribution amount ($)");
    
    let params = CompoundInterestParams {
        principal,
        annual_rate,
        compounds_per_year,
        years,
    };
    
    let result = calculate_compound_interest_with_contributions(&params, monthly_contribution);
    let result_no_contributions = calculate_compound_interest(&params);
    
    let total_contributions = monthly_contribution * years * 12.0;
    
    println!("\n=== Results ===");
    println!("Initial Principal: {}", format_currency(result.principal));
    println!("Monthly Contribution: {}", format_currency(monthly_contribution));
    println!("Total Contributions: {}", format_currency(total_contributions));
    println!("Annual Interest Rate: {}", format_percentage(params.annual_rate));
    println!("Compounding Frequency: {} times per year", params.compounds_per_year);
    println!("Time Period: {:.1} years", params.years);
    println!("Final Amount: {}", format_currency(result.final_amount));
    println!("Total Interest Earned: {}", format_currency(result.total_interest));
    println!("Effective Annual Rate: {}", format_percentage(result.effective_annual_rate));
    println!();
    println!("--- Comparison ---");
    println!("Without contributions: {}", format_currency(result_no_contributions.final_amount));
    println!("With contributions: {}", format_currency(result.final_amount));
    println!("Difference: {}", format_currency(result.final_amount - result_no_contributions.final_amount));
    println!();
}

fn calculate_time_to_target_interactive() {
    println!("\n--- Time to Reach Target Amount ---\n");
    
    let principal = get_float_input("Enter current principal amount ($)");
    let target_amount = get_float_input("Enter target amount ($)");
    let annual_rate = get_float_input("Enter annual interest rate (as decimal, e.g., 0.05 for 5%)");
    let compounds_per_year = get_u32_input("Enter number of times interest is compounded per year (1=annually, 12=monthly, 365=daily)");
    
    let years = calculate_time_to_target(principal, target_amount, annual_rate, compounds_per_year);
    
    if years > 0.0 {
        println!("\n=== Results ===");
        println!("Current Principal: {}", format_currency(principal));
        println!("Target Amount: {}", format_currency(target_amount));
        println!("Annual Interest Rate: {}", format_percentage(annual_rate));
        println!("Compounding Frequency: {} times per year", compounds_per_year);
        println!("Time to reach target: {:.1} years", years);
        println!("Time to reach target: {:.0} months", years * 12.0);
        println!();
    } else {
        println!("\nError: Cannot reach target amount with given parameters.");
        println!("Make sure your interest rate is positive and target amount is greater than principal.");
        println!();
    }
}

fn calculate_principal_for_target_interactive() {
    println!("\n--- Required Principal for Target Amount ---\n");
    
    let target_amount = get_float_input("Enter target amount ($)");
    let annual_rate = get_float_input("Enter annual interest rate (as decimal, e.g., 0.05 for 5%)");
    let compounds_per_year = get_u32_input("Enter number of times interest is compounded per year (1=annually, 12=monthly, 365=daily)");
    let years = get_float_input("Enter number of years");
    
    let principal = calculate_principal_for_target(target_amount, annual_rate, compounds_per_year, years);
    
    if principal > 0.0 {
        println!("\n=== Results ===");
        println!("Target Amount: {}", format_currency(target_amount));
        println!("Annual Interest Rate: {}", format_percentage(annual_rate));
        println!("Compounding Frequency: {} times per year", compounds_per_year);
        println!("Time Period: {:.1} years", years);
        println!("Required Principal: {}", format_currency(principal));
        println!();
    } else {
        println!("\nError: Cannot calculate required principal with given parameters.");
        println!("Make sure your interest rate and time period are positive.");
        println!();
    }
}

fn generate_breakdown_interactive() {
    println!("\n--- Year-by-Year Breakdown ---\n");
    
    let principal = get_float_input("Enter principal amount ($)");
    let annual_rate = get_float_input("Enter annual interest rate (as decimal, e.g., 0.05 for 5%)");
    let compounds_per_year = get_u32_input("Enter number of times interest is compounded per year (1=annually, 12=monthly, 365=daily)");
    let years = get_float_input("Enter number of years");
    
    let params = CompoundInterestParams {
        principal,
        annual_rate,
        compounds_per_year,
        years,
    };
    
    let breakdown = generate_breakdown(&params);
    
    println!("\n=== Year-by-Year Breakdown ===");
    println!("Initial Principal: {}", format_currency(principal));
    println!("Annual Interest Rate: {}", format_percentage(annual_rate));
    println!("Compounding Frequency: {} times per year", compounds_per_year);
    println!();
    println!("{:<6} {:<15} {:<15} {:<15}", "Year", "Amount", "Interest", "Growth");
    println!("{:-<60}", "");
    
    for year in 1..=(years as u32) {
        if let Some(result) = breakdown.get(&year) {
            println!(
                "{:<6} {:<15} {:<15} {:<15}",
                year,
                format_currency(result.final_amount),
                format_currency(result.total_interest),
                format!("{:.2}x", result.final_amount / principal)
            );
        }
    }
    println!();
}

fn calculate_weekly_with_tax_interactive() {
    println!("\n--- Weekly Compounding with Contributions and Yearly Capital Gains Tax ---\n");
    let principal = get_float_input("Enter initial/carry-forward principal ($)");
    let weekly_rate = get_float_input("Enter weekly rate of return (as decimal, e.g., 0.02 for 2%)");
    let weeks = get_u32_input("Enter number of weeks to extrapolate");
    let weekly_contribution = get_float_input("Enter weekly contribution amount ($)");
    let capital_gains_tax = get_float_input("Enter capital gains tax rate (as decimal, e.g., 0.37 for 37%)");

    let (final_after_tax, profit, tax_paid) = cical::calculate_weekly_with_yearly_tax(
        principal,
        weekly_rate,
        weeks,
        weekly_contribution,
        capital_gains_tax,
    );
    let total_contributions = weekly_contribution * weeks as f64;
    let final_before_tax = final_after_tax + tax_paid;
    println!("\n=== Results ===");
    println!("Initial Principal: {}", cical::format_currency(principal));
    println!("Weekly Contribution: {}", cical::format_currency(weekly_contribution));
    println!("Total Contributions: {}", cical::format_currency(total_contributions));
    println!("Weekly Rate: {}", cical::format_percentage(weekly_rate));
    println!("Weeks: {}", weeks);
    println!("Years: {:.1}", weeks as f64 / 52.0);
    println!("Final Amount (before tax): {}", cical::format_currency(final_before_tax));
    println!("Profit (before tax): {}", cical::format_currency(profit));
    println!("Capital Gains Tax Rate: {}", cical::format_percentage(capital_gains_tax));
    println!("Total Tax Paid (yearly): {}", cical::format_currency(tax_paid));
    println!("Final Amount (after tax): {}", cical::format_currency(final_after_tax));
    println!("Growth Factor (after tax): {:.2}x", final_after_tax / (principal + total_contributions));
    println!();
} 