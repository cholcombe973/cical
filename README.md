# Compound Interest Calculator (CICAL)

A comprehensive compound interest calculator written in Rust, providing both a library API and an interactive CLI interface.

## Features

- **Basic Compound Interest**: Calculate final amount, total interest, and effective annual rate
- **Compound Interest with Contributions**: Include regular monthly contributions in calculations
- **Time to Target**: Calculate how long it takes to reach a target amount
- **Principal for Target**: Calculate required initial principal to reach a target amount
- **Year-by-Year Breakdown**: Generate detailed annual growth projections
- **Multiple Compounding Frequencies**: Support for annual, monthly, daily, and custom compounding periods
- **Comprehensive Testing**: Thorough test suite covering all calculation methods

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd cical
```

2. Build the project:
```bash
cargo build
```

3. Run the interactive calculator:
```bash
cargo run
```

## Usage

### Interactive CLI

Run `cargo run` to start the interactive calculator:

```
=== Compound Interest Calculator ===

Choose an option:
1. Calculate compound interest
2. Calculate compound interest with monthly contributions
3. Calculate time to reach target amount
4. Calculate required principal for target amount
5. Generate year-by-year breakdown
6. Exit

Enter your choice (1-6):
```

### Library API

You can also use the library directly in your Rust projects:

```rust
use cical::{CompoundInterestParams, calculate_compound_interest};

let params = CompoundInterestParams {
    principal: 1000.0,
    annual_rate: 0.05,  // 5%
    compounds_per_year: 12,  // Monthly
    years: 10.0,
};

let result = calculate_compound_interest(&params);
println!("Final amount: ${:.2}", result.final_amount);
println!("Total interest: ${:.2}", result.total_interest);
```

## API Reference

### Data Structures

#### `CompoundInterestParams`
```rust
pub struct CompoundInterestParams {
    pub principal: f64,           // Initial amount
    pub annual_rate: f64,         // Annual interest rate (decimal)
    pub compounds_per_year: u32,  // Compounding frequency
    pub years: f64,              // Time period
}
```

#### `CompoundInterestResult`
```rust
pub struct CompoundInterestResult {
    pub final_amount: f64,           // Final amount after interest
    pub total_interest: f64,         // Total interest earned
    pub principal: f64,              // Initial principal
    pub effective_annual_rate: f64,  // Effective annual rate
}
```

### Functions

#### `calculate_compound_interest(params: &CompoundInterestParams) -> CompoundInterestResult`
Calculates compound interest using the standard formula: A = P(1 + r/n)^(nt)

#### `calculate_compound_interest_with_contributions(params: &CompoundInterestParams, monthly_contribution: f64) -> CompoundInterestResult`
Calculates compound interest including regular monthly contributions.

#### `calculate_time_to_target(principal: f64, target_amount: f64, annual_rate: f64, compounds_per_year: u32) -> f64`
Calculates the time needed to reach a target amount.

#### `calculate_principal_for_target(target_amount: f64, annual_rate: f64, compounds_per_year: u32, years: f64) -> f64`
Calculates the required principal to reach a target amount in given time.

#### `generate_breakdown(params: &CompoundInterestParams) -> HashMap<u32, CompoundInterestResult>`
Generates a year-by-year breakdown of compound interest growth.

#### `format_currency(amount: f64) -> String`
Formats a number as currency (e.g., "$1,234.56").

#### `format_percentage(rate: f64) -> String`
Formats a decimal rate as a percentage (e.g., "5.00%").

## Examples

### Example 1: Basic Compound Interest
```rust
let params = CompoundInterestParams {
    principal: 10000.0,
    annual_rate: 0.06,  // 6%
    compounds_per_year: 12,  // Monthly
    years: 20.0,
};

let result = calculate_compound_interest(&params);
// Final amount: $33,102.04
// Total interest: $23,102.04
```

### Example 2: With Monthly Contributions
```rust
let params = CompoundInterestParams {
    principal: 10000.0,
    annual_rate: 0.06,
    compounds_per_year: 12,
    years: 20.0,
};

let result = calculate_compound_interest_with_contributions(&params, 500.0);
// Final amount: $245,560.00
// Total interest: $125,560.00
```

### Example 3: Time to Double Your Money
```rust
let years = calculate_time_to_target(10000.0, 20000.0, 0.07, 12);
// Approximately 9.9 years at 7% monthly compounding
```

## Testing

Run the test suite:
```bash
cargo test
```

The tests cover:
- Basic compound interest calculations
- Monthly vs annual compounding
- Compound interest with contributions
- Time to target calculations
- Principal for target calculations

## Mathematical Formulas

### Basic Compound Interest
```
A = P(1 + r/n)^(nt)
```
Where:
- A = Final amount
- P = Principal amount
- r = Annual interest rate
- n = Number of times interest is compounded per year
- t = Time in years

### Compound Interest with Contributions
```
FV = P(1 + r/n)^(nt) + PMT × ((1 + r/12)^(12t) - 1) / (r/12)
```
Where:
- FV = Future value
- P = Initial principal
- PMT = Monthly contribution
- r = Annual interest rate
- t = Time in years

### Effective Annual Rate
```
EAR = (1 + r/n)^n - 1
```

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 