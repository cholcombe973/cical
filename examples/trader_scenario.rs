use cical::*;

fn main() {
    println!("=== Trader Scenario: Weekly Compounding with Yearly Tax ===\n");
    
    // User's specific parameters
    let principal = 13500.0;
    let weekly_rate = 0.02; // 2% per week
    let weeks = 156; // 3 years * 52 weeks
    let weekly_contribution = 100.0;
    let capital_gains_tax = 0.37; // 37%
    
    println!("Parameters:");
    println!("Initial Principal: {}", format_currency(principal));
    println!("Weekly Rate: {}", format_percentage(weekly_rate));
    println!("Weeks: {} ({} years)", weeks, weeks as f64 / 52.0);
    println!("Weekly Contribution: {}", format_currency(weekly_contribution));
    println!("Capital Gains Tax Rate: {}", format_percentage(capital_gains_tax));
    println!();
    
    let (final_after_tax, profit_before_tax, total_tax_paid) = calculate_weekly_with_yearly_tax(
        principal,
        weekly_rate,
        weeks,
        weekly_contribution,
        capital_gains_tax,
    );
    
    let total_contributions = weekly_contribution * weeks as f64;
    let final_before_tax = final_after_tax + total_tax_paid;
    
    println!("=== Results ===");
    println!("Initial Principal: {}", format_currency(principal));
    println!("Total Contributions: {}", format_currency(total_contributions));
    println!("Total Invested: {}", format_currency(principal + total_contributions));
    println!();
    println!("Final Amount (before tax): {}", format_currency(final_before_tax));
    println!("Profit (before tax): {}", format_currency(profit_before_tax));
    println!("Total Tax Paid (yearly): {}", format_currency(total_tax_paid));
    println!("Final Amount (after tax): {}", format_currency(final_after_tax));
    println!();
    println!("Net Profit (after tax): {}", format_currency(final_after_tax - principal - total_contributions));
    println!("Growth Factor (after tax): {:.2}x", final_after_tax / (principal + total_contributions));
    println!("Effective Annual Return (after tax): {:.2}%", 
        ((final_after_tax / (principal + total_contributions)).powf(1.0 / (weeks as f64 / 52.0)) - 1.0) * 100.0);
    println!();
    
    // Comparison: what if there was no tax?
    let (final_no_tax, profit_no_tax, _) = calculate_weekly_with_yearly_tax(
        principal,
        weekly_rate,
        weeks,
        weekly_contribution,
        0.0, // No tax
    );
    
    println!("=== Tax Impact Comparison ===");
    println!("Without tax: {}", format_currency(final_no_tax));
    println!("With tax: {}", format_currency(final_after_tax));
    println!("Tax impact: {}", format_currency(final_no_tax - final_after_tax));
    println!("Tax reduces final amount by: {:.1}%", 
        (final_no_tax - final_after_tax) / final_no_tax * 100.0);
} 