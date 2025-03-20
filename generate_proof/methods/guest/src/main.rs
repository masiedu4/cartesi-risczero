use risc0_zkvm::guest::env;

// Minimum age requirement (cannot be modified during runtime)
const MINIMUM_AGE: u64 = 21;
const SECONDS_IN_YEAR: u64 = 31_536_000; // 365 days
const SECONDS_IN_DAY: u64 = 86_400;

fn main() {
    // Read the birthdate timestamp from the host
    let birthdate_timestamp: u64 = env::read();

    // Read the current time from host (to prevent time manipulation)
    let current_timestamp: u64 = env::read();

    // Calculate approximate age in years
    let age_in_seconds = current_timestamp.saturating_sub(birthdate_timestamp);
    let mut age = age_in_seconds / SECONDS_IN_YEAR;

    // Account for leap years more precisely by checking day of year
    let birthdate_day_of_year = day_of_year(birthdate_timestamp);
    let current_day_of_year = day_of_year(current_timestamp);

    // If we haven't reached the birthday day this year, subtract 1 from age
    if current_day_of_year < birthdate_day_of_year {
        age = age.saturating_sub(1);
    }

    // Verify age requirement
    assert!(age >= MINIMUM_AGE, "Age verification failed: Too young!");

    // Commit only the verification result, not the actual age
    let verification_result = true;
    env::commit(&verification_result);
}

// Helper function to calculate day of year (1-366)
fn day_of_year(timestamp: u64) -> u64 {
    // Get days since epoch
    let days_since_epoch = timestamp / SECONDS_IN_DAY;

    // Calculate year (approximate)
    let year = 1970 + (days_since_epoch / 365);

    // Calculate start of year timestamp
    let mut year_start = 0;
    for y in 1970..year {
        year_start += if is_leap_year(y) { 366 } else { 365 };
    }
    year_start *= SECONDS_IN_DAY;

    // Calculate day of year
    ((timestamp - year_start) / SECONDS_IN_DAY) + 1
}

// Helper function to check if a year is a leap year
fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
