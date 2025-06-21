use cical::*;

fn main() {
    println!("=== Compound Interest Calculator Examples ===\n");

    // Example 1: Basic compound interest
    println!("Example 1: Basic Compound Interest");
    println!("-----------------------------------");
    let params = CompoundInterestParams {
        principal: 10000.0,
        annual_rate: 0.06,  // 6%
        compounds_per_year: 12,  // Monthly
        years: 20.0,
    };

    let result = calculate_compound_interest(&params);
    println!("Initial Principal: {}", format_currency(params.principal));
    println!("Annual Rate: {}", format_percentage(params.annual_rate));
    println!("Time Period: {:.1} years", params.years);
    println!("Final Amount: {}", format_currency(result.final_amount));
    println!("Total Interest: {}", format_currency(result.total_interest));
    println!("Growth Factor: {:.2}x", result.final_amount / params.principal);
    println!();

    // Example 2: Compound interest with monthly contributions
    println!("Example 2: Compound Interest with Monthly Contributions");
    println!("------------------------------------------------------");
    let monthly_contribution = 500.0;
    let result_with_contributions = calculate_compound_interest_with_contributions(&params, monthly_contribution);
    
    let total_contributions = monthly_contribution * params.years * 12.0;
    println!("Initial Principal: {}", format_currency(params.principal));
    println!("Monthly Contribution: {}", format_currency(monthly_contribution));
    println!("Total Contributions: {}", format_currency(total_contributions));
    println!("Final Amount: {}", format_currency(result_with_contributions.final_amount));
    println!("Total Interest: {}", format_currency(result_with_contributions.total_interest));
    println!();

    // Example 3: Time to double your money
    println!("Example 3: Time to Double Your Money");
    println!("------------------------------------");
    let years_to_double = calculate_time_to_target(10000.0, 20000.0, 0.07, 12);
    println!("Principal: {}", format_currency(10000.0));
    println!("Target: {}", format_currency(20000.0));
    println!("Annual Rate: {}", format_percentage(0.07));
    println!("Time to double: {:.1} years", years_to_double);
    println!("Time to double: {:.0} months", years_to_double * 12.0);
    println!();

    // Example 4: Required principal for target
    println!("Example 4: Required Principal for Target");
    println!("----------------------------------------");
    let required_principal = calculate_principal_for_target(100000.0, 0.05, 12, 15.0);
    println!("Target Amount: {}", format_currency(100000.0));
    println!("Annual Rate: {}", format_percentage(0.05));
    println!("Time Period: {:.1} years", 15.0);
    println!("Required Principal: {}", format_currency(required_principal));
    println!();

    // Example 5: Year-by-year breakdown
    println!("Example 5: Year-by-Year Breakdown (First 5 years)");
    println!("------------------------------------------------");
    let short_params = CompoundInterestParams {
        principal: 5000.0,
        annual_rate: 0.08,  // 8%
        compounds_per_year: 12,  // Monthly
        years: 5.0,
    };
    
    let breakdown = generate_breakdown(&short_params);
    println!("Initial Principal: {}", format_currency(short_params.principal));
    println!("Annual Rate: {}", format_percentage(short_params.annual_rate));
    println!();
    println!("{:<6} {:<15} {:<15} {:<15}", "Year", "Amount", "Interest", "Growth");
    println!("{:-<60}", "");
    
    for year in 1..=5 {
        if let Some(result) = breakdown.get(&year) {
            println!(
                "{:<6} {:<15} {:<15} {:<15}",
                year,
                format_currency(result.final_amount),
                format_currency(result.total_interest),
                format!("{:.2}x", result.final_amount / short_params.principal)
            );
        }
    }
    println!();

    // Example 6: Comparison of different compounding frequencies
    println!("Example 6: Compounding Frequency Comparison");
    println!("-------------------------------------------");
    let base_params = CompoundInterestParams {
        principal: 10000.0,
        annual_rate: 0.05,  // 5%
        compounds_per_year: 1,  // Will be overridden for each frequency
        years: 10.0,
    };

    let frequencies = vec![
        ("Annually", 1),
        ("Semi-annually", 2),
        ("Quarterly", 4),
        ("Monthly", 12),
        ("Daily", 365),
    ];

    println!("Initial Principal: {}", format_currency(base_params.principal));
    println!("Annual Rate: {}", format_percentage(base_params.annual_rate));
    println!("Time Period: {:.1} years", base_params.years);
    println!();
    println!("{:<15} {:<15} {:<15} {:<15}", "Frequency", "Final Amount", "Total Interest", "Effective Rate");
    println!("{:-<70}", "");

    for (name, freq) in frequencies {
        let params = CompoundInterestParams {
            compounds_per_year: freq,
            ..base_params
        };
        let result = calculate_compound_interest(&params);
        println!(
            "{:<15} {:<15} {:<15} {:<15}",
            name,
            format_currency(result.final_amount),
            format_currency(result.total_interest),
            format_percentage(result.effective_annual_rate)
        );
    }
} 