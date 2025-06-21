use std::collections::HashMap;

/// Represents the parameters for compound interest calculations
#[derive(Debug, Clone)]
pub struct CompoundInterestParams {
    /// Initial principal amount
    pub principal: f64,
    /// Annual interest rate (as a decimal, e.g., 0.05 for 5%)
    pub annual_rate: f64,
    /// Number of times interest is compounded per year
    pub compounds_per_year: u32,
    /// Number of years
    pub years: f64,
}

/// Represents the result of a compound interest calculation
#[derive(Debug, Clone)]
pub struct CompoundInterestResult {
    /// Final amount after compound interest
    pub final_amount: f64,
    /// Total interest earned
    pub total_interest: f64,
    /// Initial principal
    pub principal: f64,
    /// Effective annual rate
    pub effective_annual_rate: f64,
}

/// Calculate compound interest using the standard formula
/// A = P(1 + r/n)^(nt)
/// Where:
/// A = Final amount
/// P = Principal amount
/// r = Annual interest rate
/// n = Number of times interest is compounded per year
/// t = Time in years
pub fn calculate_compound_interest(params: &CompoundInterestParams) -> CompoundInterestResult {
    let principal = params.principal;
    let rate = params.annual_rate;
    let compounds = params.compounds_per_year as f64;
    let years = params.years;
    
    let final_amount = principal * (1.0 + rate / compounds).powf(compounds * years);
    let total_interest = final_amount - principal;
    let effective_annual_rate = (1.0 + rate / compounds).powf(compounds) - 1.0;
    
    CompoundInterestResult {
        final_amount,
        total_interest,
        principal,
        effective_annual_rate,
    }
}

/// Calculate compound interest with regular contributions
/// This uses the future value of annuity formula combined with compound interest
pub fn calculate_compound_interest_with_contributions(
    params: &CompoundInterestParams,
    monthly_contribution: f64,
) -> CompoundInterestResult {
    let principal = params.principal;
    let rate = params.annual_rate;
    let compounds = params.compounds_per_year as f64;
    let years = params.years;
    let monthly_rate = rate / 12.0;
    let total_months = years * 12.0;
    
    // Future value of initial principal
    let principal_future_value = principal * (1.0 + rate / compounds).powf(compounds * years);
    
    // Future value of monthly contributions (annuity)
    let contribution_future_value = if monthly_rate > 0.0 {
        monthly_contribution * ((1.0 + monthly_rate).powf(total_months) - 1.0) / monthly_rate
    } else {
        monthly_contribution * total_months
    };
    
    let final_amount = principal_future_value + contribution_future_value;
    let total_interest = final_amount - principal - (monthly_contribution * total_months);
    let effective_annual_rate = (1.0 + rate / compounds).powf(compounds) - 1.0;
    
    CompoundInterestResult {
        final_amount,
        total_interest,
        principal,
        effective_annual_rate,
    }
}

/// Calculate the time needed to reach a target amount
pub fn calculate_time_to_target(
    principal: f64,
    target_amount: f64,
    annual_rate: f64,
    compounds_per_year: u32,
) -> f64 {
    let rate = annual_rate;
    let compounds = compounds_per_year as f64;
    
    if rate <= 0.0 || principal <= 0.0 || target_amount <= principal {
        return 0.0;
    }
    
    let years = (target_amount / principal).ln() / (compounds * (1.0 + rate / compounds).ln());
    years
}

/// Calculate the required principal to reach a target amount in given time
pub fn calculate_principal_for_target(
    target_amount: f64,
    annual_rate: f64,
    compounds_per_year: u32,
    years: f64,
) -> f64 {
    let rate = annual_rate;
    let compounds = compounds_per_year as f64;
    
    if rate <= 0.0 || years <= 0.0 {
        return 0.0;
    }
    
    let principal = target_amount / (1.0 + rate / compounds).powf(compounds * years);
    principal
}

/// Generate a year-by-year breakdown of compound interest
pub fn generate_breakdown(params: &CompoundInterestParams) -> HashMap<u32, CompoundInterestResult> {
    let mut breakdown = HashMap::new();
    
    for year in 1..=(params.years as u32) {
        let year_params = CompoundInterestParams {
            years: year as f64,
            ..params.clone()
        };
        breakdown.insert(year, calculate_compound_interest(&year_params));
    }
    
    breakdown
}

/// Format currency values for display
pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

/// Format percentage values for display
pub fn format_percentage(rate: f64) -> String {
    format!("{:.2}%", rate * 100.0)
}

/// Calculate compound interest with weekly contributions, weekly compounding, and yearly capital gains tax
/// P = initial principal
/// R = weekly rate (as decimal)
/// weeks = number of weeks
/// weekly_contribution = amount contributed each week
/// capital_gains_tax = tax rate on profits (as decimal, e.g., 0.37 for 37%)
/// Returns (final_amount_after_tax, total_interest_before_tax, total_tax_paid)
pub fn calculate_weekly_with_yearly_tax(
    principal: f64,
    weekly_rate: f64,
    weeks: u32,
    weekly_contribution: f64,
    capital_gains_tax: f64,
) -> (f64, f64, f64) {
    let weeks_per_year = 52;
    let years = weeks / weeks_per_year;
    let remaining_weeks = weeks % weeks_per_year;
    
    let mut current_principal = principal;
    let mut total_tax_paid = 0.0;
    let mut total_contributions = 0.0;
    
    // Process complete years
    for year in 0..years {
        let year_start_principal = current_principal;
        let year_contributions = weekly_contribution * weeks_per_year as f64;
        total_contributions += year_contributions;
        
        // Calculate growth for the year
        let year_end_principal = year_start_principal * (1.0 + weekly_rate).powf(weeks_per_year as f64);
        let year_end_contributions = if weekly_rate > 0.0 {
            year_contributions * ((1.0 + weekly_rate).powf(weeks_per_year as f64) - 1.0) / weekly_rate
        } else {
            year_contributions
        };
        let year_end_total = year_end_principal + year_end_contributions;
        
        // Calculate profit for the year and apply tax
        let year_profit = year_end_total - year_start_principal - year_contributions;
        let year_tax = if year_profit > 0.0 { year_profit * capital_gains_tax } else { 0.0 };
        total_tax_paid += year_tax;
        
        // Carry forward after-tax amount
        current_principal = year_end_total - year_tax;
    }
    
    // Process remaining weeks
    if remaining_weeks > 0 {
        let remaining_contributions = weekly_contribution * remaining_weeks as f64;
        total_contributions += remaining_contributions;
        
        let final_principal = current_principal * (1.0 + weekly_rate).powf(remaining_weeks as f64);
        let final_contributions = if weekly_rate > 0.0 {
            remaining_contributions * ((1.0 + weekly_rate).powf(remaining_weeks as f64) - 1.0) / weekly_rate
        } else {
            remaining_contributions
        };
        let final_total = final_principal + final_contributions;
        
        // Apply tax to remaining weeks (pro-rated for partial year)
        let remaining_profit = final_total - current_principal - remaining_contributions;
        let remaining_tax = if remaining_profit > 0.0 { 
            remaining_profit * capital_gains_tax * (remaining_weeks as f64 / weeks_per_year as f64)
        } else { 
            0.0 
        };
        total_tax_paid += remaining_tax;
        
        current_principal = final_total - remaining_tax;
    }
    
    let total_profit_before_tax = current_principal + total_tax_paid - principal - total_contributions;
    (current_principal, total_profit_before_tax, total_tax_paid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_compound_interest() {
        let params = CompoundInterestParams {
            principal: 1000.0,
            annual_rate: 0.05,
            compounds_per_year: 1,
            years: 10.0,
        };
        
        let result = calculate_compound_interest(&params);
        
        // A = 1000 * (1 + 0.05)^10 = 1000 * 1.6289 = 1628.89
        assert!((result.final_amount - 1628.89).abs() < 0.01);
        assert!((result.total_interest - 628.89).abs() < 0.01);
    }

    #[test]
    fn test_monthly_compounding() {
        let params = CompoundInterestParams {
            principal: 1000.0,
            annual_rate: 0.05,
            compounds_per_year: 12,
            years: 1.0,
        };
        
        let result = calculate_compound_interest(&params);
        
        // Monthly compounding should give slightly higher result than annual
        assert!(result.final_amount > 1050.0);
    }

    #[test]
    fn test_compound_interest_with_contributions() {
        let params = CompoundInterestParams {
            principal: 1000.0,
            annual_rate: 0.05,
            compounds_per_year: 12,
            years: 10.0,
        };
        
        let result = calculate_compound_interest_with_contributions(&params, 100.0);
        
        // Should be higher than without contributions
        let result_no_contributions = calculate_compound_interest(&params);
        assert!(result.final_amount > result_no_contributions.final_amount);
    }

    #[test]
    fn test_time_to_target() {
        let years = calculate_time_to_target(1000.0, 2000.0, 0.05, 1);
        
        // Should take approximately 14.2 years to double at 5% annual interest
        assert!((years - 14.2).abs() < 0.5);
    }

    #[test]
    fn test_principal_for_target() {
        let principal = calculate_principal_for_target(2000.0, 0.05, 1, 10.0);
        
        // Should be approximately 1227.83
        assert!((principal - 1227.83).abs() < 1.0);
    }

    #[test]
    fn test_weekly_with_tax() {
        let principal = 10000.0;
        let weekly_rate = 0.01; // 1% per week
        let weeks = 52 * 2; // 2 years
        let weekly_contribution = 100.0;
        let capital_gains_tax = 0.3; // 30%

        let (final_after_tax, profit, tax_paid) = calculate_weekly_with_yearly_tax(
            principal,
            weekly_rate,
            weeks,
            weekly_contribution,
            capital_gains_tax,
        );

        // Check that final amount after tax is less than without tax
        let (final_no_tax, profit_no_tax, _) = calculate_weekly_with_yearly_tax(
            principal,
            weekly_rate,
            weeks,
            weekly_contribution,
            0.0,
        );
        assert!(final_after_tax < final_no_tax);
        assert!(tax_paid > 0.0);
        // With yearly tax, the relationship is different - tax is paid each year
        // so the total profit after tax should be less than without tax
        assert!(final_after_tax < final_no_tax);
        // Sanity check: final after tax should be greater than principal + contributions
        let total_contributions = weekly_contribution * weeks as f64;
        assert!(final_after_tax > principal + total_contributions);
    }
}
