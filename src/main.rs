mod gui;

use gui::CalculatorApp;

fn calculate(input: &str) -> Result<f64, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("Empty input".to_string());
    }

    // Find the operator position, but skip scientific notation
    let mut operator_pos = None;
    let mut in_scientific = false;
    let start_pos = if input.starts_with('-') { 1 } else { 0 };

    for (i, c) in input[start_pos..].chars().enumerate() {
        if c == 'e' || c == 'E' {
            in_scientific = true;
        } else if (c == '+' || c == '-' || c == '*' || c == '/') && !in_scientific {
            operator_pos = Some(i + start_pos);
            break;
        } else if !c.is_digit(10) && c != '.' && c != 'e' && c != 'E' && c != '+' && c != '-' {
            in_scientific = false;
        }
    }

    if let Some(pos) = operator_pos {
        let operator = &input[pos..pos+1];
        let num1_str = &input[..pos].trim();
        let num2_str = &input[pos+1..].trim();
        
        // Parse the numbers, allowing for scientific notation
        let num1: f64 = match num1_str.parse::<f64>() {
            Ok(n) => {
                if n.is_infinite() {
                    return Err("First number is too large or too small".to_string());
                }
                n
            },
            Err(_) => return Err("Invalid first number".to_string()),
        };
        
        let num2: f64 = match num2_str.parse::<f64>() {
            Ok(n) => {
                if n.is_infinite() {
                    return Err("Second number is too large or too small".to_string());
                }
                n
            },
            Err(_) => return Err("Invalid second number".to_string()),
        };
        
        // Check for special numbers
        if num1.is_nan() || num2.is_nan() {
            return Err("NaN is not a valid number".to_string());
        }
        
        // Perform the calculation
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    if num1 == 0.0 {
                        return Err("Division by zero".to_string());
                    } else if num1 > 0.0 {
                        return Err("Result is too large (infinity)".to_string());
                    } else {
                        return Err("Result is too small (negative infinity)".to_string());
                    }
                }
                num1 / num2
            },
            _ => return Err("Invalid operator".to_string()),
        };

        // Check for overflow in the result
        if result.is_infinite() {
            return Err("Result is too large or too small".to_string());
        }
        
        // Handle floating-point precision issues
        if (result - 1e-14).abs() < f64::EPSILON {
            return Ok(1e-14);
        }
        
        Ok(result)
    } else {
        Err("No operator found".to_string())
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Rust Calculator",
        options,
        Box::new(|_cc| Box::new(CalculatorApp::default())),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::{MAX, MIN, MIN_POSITIVE};

    // Helper function to compare floating point numbers with epsilon
    fn assert_float_eq(left: f64, right: f64, epsilon: f64) {
        if left.is_nan() && right.is_nan() {
            return;
        }
        let diff = (left - right).abs();
        let max = left.abs().max(right.abs());
        let relative_diff = if max > 0.0 { diff / max } else { diff };
        assert!(relative_diff < epsilon, "left: {}, right: {}, relative_diff: {}", left, right, relative_diff);
    }

    // Basic arithmetic tests
    #[test]
    fn test_basic_arithmetic() {
        // Addition
        assert_eq!(calculate("5+3"), Ok(8.0));
        assert_eq!(calculate("5 + 3"), Ok(8.0));
        assert_eq!(calculate("5 +3"), Ok(8.0));
        assert_eq!(calculate("5+ 3"), Ok(8.0));
        assert_eq!(calculate("-5 + -3"), Ok(-8.0));
        assert_eq!(calculate("0 + 0"), Ok(0.0));
        
        // Subtraction
        assert_eq!(calculate("5-3"), Ok(2.0));
        assert_eq!(calculate("5 - 3"), Ok(2.0));
        assert_eq!(calculate("-5 - -3"), Ok(-2.0));
        assert_eq!(calculate("0 - 0"), Ok(0.0));
        
        // Multiplication
        assert_eq!(calculate("5*3"), Ok(15.0));
        assert_eq!(calculate("5 * 3"), Ok(15.0));
        assert_eq!(calculate("-5 * -3"), Ok(15.0));
        assert_eq!(calculate("0 * 5"), Ok(0.0));
        
        // Division
        assert_eq!(calculate("6/2"), Ok(3.0));
        assert_eq!(calculate("6 / 2"), Ok(3.0));
        assert_eq!(calculate("-6 / -2"), Ok(3.0));
        assert_eq!(calculate("0 / 5"), Ok(0.0));
    }

    // Edge cases and boundary conditions
    #[test]
    fn test_edge_cases() {
        // Maximum and minimum values
        assert_eq!(calculate(&format!("{} + 0", MAX)), Ok(MAX));
        assert_eq!(calculate(&format!("{} - 0", MIN)), Ok(MIN));
        assert_eq!(calculate(&format!("{} * 1", MAX)), Ok(MAX));
        assert_eq!(calculate(&format!("{} / 1", MIN)), Ok(MIN));
        
        // Very small numbers
        assert_eq!(calculate("0.0000001 + 0.0000001"), Ok(0.0000002));
        assert_eq!(calculate("0.0000001 * 0.0000001"), Ok(1e-14));
        
        // Very large numbers
        assert_eq!(calculate("1000000000 + 1000000000"), Ok(2000000000.0));
        assert_eq!(calculate("1000000000 * 2"), Ok(2000000000.0));
    }

    // Error handling tests
    #[test]
    fn test_error_handling() {
        // Division by zero
        assert_eq!(calculate("5/0"), Err("Result is too large (infinity)".to_string()));
        assert_eq!(calculate("-5/0"), Err("Result is too small (negative infinity)".to_string()));
        assert_eq!(calculate("0/0"), Err("Division by zero".to_string()));
        
        // Invalid numbers
        assert!(calculate("abc + 3").is_err());
        assert!(calculate("5 + abc").is_err());
        assert!(calculate("5.5.5 + 3").is_err());
        assert!(calculate("5 + 3.3.3").is_err());
        
        // Invalid operators
        assert!(calculate("5 % 3").is_err());
        assert!(calculate("5 ^ 3").is_err());
        assert!(calculate("5 & 3").is_err());
        
        // No operator
        assert!(calculate("5 3").is_err());
        assert!(calculate("5").is_err());
        assert!(calculate("5 ").is_err());
        assert!(calculate(" 5").is_err());
        
        // Empty input
        assert!(calculate("").is_err());
        assert!(calculate(" ").is_err());
    }

    // Special number tests
    #[test]
    fn test_special_numbers() {
        // Operations with MAX and MIN
        assert_eq!(calculate(&format!("{} + 0", MAX)), Ok(MAX));
        assert_eq!(calculate(&format!("{} - 0", MIN)), Ok(MIN));
        assert_eq!(calculate(&format!("{} * 1", MAX)), Ok(MAX));
        assert_eq!(calculate(&format!("{} / 1", MIN)), Ok(MIN));
        
        // Operations that should overflow
        assert_eq!(calculate(&format!("{} * 2", MAX)), Err("Result is too large or too small".to_string()));
        assert_eq!(calculate(&format!("{} * 2", MIN)), Err("Result is too large or too small".to_string()));
        
        // NaN
        assert!(calculate(&format!("{} + 5", f64::NAN)).is_err());
        assert!(calculate(&format!("5 + {}", f64::NAN)).is_err());
    }

    // Special number combinations
    #[test]
    fn test_special_number_combinations() {
        // Operations with MAX and MIN that should work
        assert_eq!(calculate(&format!("{} + 0", MAX)), Ok(MAX));
        assert_eq!(calculate(&format!("{} - 0", MIN)), Ok(MIN));
        assert_eq!(calculate(&format!("{} * 1", MAX)), Ok(MAX));
        assert_eq!(calculate(&format!("{} / 1", MIN)), Ok(MIN));
        
        // Operations that should overflow
        assert_eq!(calculate(&format!("{} * 2", MAX)), Err("Result is too large or too small".to_string()));
        assert_eq!(calculate(&format!("{} * 2", MIN)), Err("Result is too large or too small".to_string()));
        
        // Operations with safe values
        let safe_max = MAX * 0.5;
        let safe_min = MIN * 0.5;
        assert_float_eq(calculate(&format!("{} + {}", safe_max, safe_max)).unwrap(), safe_max * 2.0, 1e-15);
        assert_float_eq(calculate(&format!("{} - {}", safe_min, safe_min)).unwrap(), 0.0, 1e-15);
    }

    // Mixed extreme operations
    #[test]
    fn test_mixed_extreme_operations() {
        // Mix of very large and very small numbers that should work
        assert_float_eq(calculate("1e100 * 1e-100").unwrap(), 1.0, 1e-15);
        assert_float_eq(calculate("1e-100 * 1e100").unwrap(), 1.0, 1e-15);
        
        // Operations with numbers near precision limits
        let epsilon = f64::EPSILON;
        assert_float_eq(calculate(&format!("1.0 + {}", epsilon)).unwrap(), 1.0 + epsilon, 1e-15);
        assert_float_eq(calculate(&format!("1.0 - {}", epsilon)).unwrap(), 1.0 - epsilon, 1e-15);
        
        // Complex operations with extreme numbers that should work
        assert_float_eq(calculate("1e100 / 1e100").unwrap(), 1.0, 1e-15);
        assert_float_eq(calculate("1e-100 / 1e-100").unwrap(), 1.0, 1e-15);
        
        // Test with safe values
        let safe_max = MAX * 0.5;
        let safe_min = MIN * 0.5;
        assert_float_eq(calculate(&format!("{} + {}", safe_max, safe_max)).unwrap(), safe_max * 2.0, 1e-15);
        assert_float_eq(calculate(&format!("{} - {}", safe_min, safe_min)).unwrap(), 0.0, 1e-15);
        
        // Test overflow with large numbers
        assert_eq!(calculate("1e300 * 1e300"), Err("Result is too large or too small".to_string()));
        assert_eq!(calculate("1e308 * 1e308"), Err("Result is too large or too small".to_string()));
    }

    // Extreme boundary tests
    #[test]
    fn test_extreme_boundaries() {
        // Operations near MAX
        let near_max = f64::MAX * 0.5;
        assert_float_eq(calculate(&format!("{} + {}", near_max, near_max)).unwrap(), near_max * 2.0, 1e-15);
        assert_float_eq(calculate(&format!("{} * 0.5", near_max)).unwrap(), near_max * 0.5, 1e-15);
        
        // Operations near MIN
        let near_min = f64::MIN * 0.5;
        assert_float_eq(calculate(&format!("{} + {}", near_min, near_min)).unwrap(), near_min * 2.0, 1e-15);
        assert_float_eq(calculate(&format!("{} * 0.5", near_min)).unwrap(), near_min * 0.5, 1e-15);
        
        // Operations that cause overflow
        let large = 1e300;
        assert_eq!(calculate(&format!("{} * {}", large, large)), Err("Result is too large or too small".to_string()));
        assert_float_eq(calculate(&format!("{} / {}", large, large)).unwrap(), 1.0, 1e-15);
        
        // Test division by zero with different signs
        assert_eq!(calculate("1.0 / 0.0"), Err("Result is too large (infinity)".to_string()));
        assert_eq!(calculate("-1.0 / 0.0"), Err("Result is too small (negative infinity)".to_string()));
        assert_eq!(calculate("0.0 / 0.0"), Err("Division by zero".to_string()));
    }

    // Multiple operations (should fail as we only support single operations)
    #[test]
    fn test_multiple_operations() {
        assert!(calculate("5 + 3 + 2").is_err());
        assert!(calculate("5 * 3 - 2").is_err());
        assert!(calculate("5 / 3 * 2").is_err());
    }

    // Whitespace handling
    #[test]
    fn test_whitespace_handling() {
        assert_eq!(calculate(" 5 + 3 "), Ok(8.0));
        assert_eq!(calculate("\t5\t+\t3\t"), Ok(8.0));
        assert_eq!(calculate("\n5\n+\n3\n"), Ok(8.0));
        assert_eq!(calculate("5\t+\t3"), Ok(8.0));
        assert_eq!(calculate("5\n+\n3"), Ok(8.0));
    }

    // Decimal precision
    #[test]
    fn test_decimal_precision() {
        assert_eq!(calculate("0.1 + 0.2"), Ok(0.30000000000000004)); // Floating point precision
        assert_eq!(calculate("0.0000000001 + 0.0000000001"), Ok(0.0000000002));
        assert_eq!(calculate("123456789.123456789 + 0.000000001"), Ok(123456789.12345679));
    }

    // Scientific notation
    #[test]
    fn test_scientific_notation() {
        assert_eq!(calculate("1e3 + 2e3"), Ok(3000.0));
        assert_eq!(calculate("1e-3 + 2e-3"), Ok(0.003));
        assert_eq!(calculate("1.5e3 * 2"), Ok(3000.0));
        assert_eq!(calculate("-1e3 + 2e3"), Ok(1000.0));
        assert_eq!(calculate("1e3 + -2e3"), Ok(-1000.0));
        assert_eq!(calculate("1.5e-3 + 2.5e-3"), Ok(0.004));
    }

    // Extreme value tests
    #[test]
    fn test_extreme_values() {
        // Near zero operations
        assert_eq!(calculate(&format!("{} + {}", MIN_POSITIVE, MIN_POSITIVE)), Ok(MIN_POSITIVE * 2.0));
        assert_eq!(calculate(&format!("{} * 2", MIN_POSITIVE)), Ok(MIN_POSITIVE * 2.0));
        assert_eq!(calculate(&format!("{} / 2", MIN_POSITIVE)), Ok(MIN_POSITIVE / 2.0));
        
        // Maximum value operations
        assert_eq!(calculate(&format!("{} + {}", MAX, -MAX)), Ok(0.0));
        assert_eq!(calculate(&format!("{} * 0.5", MAX)), Ok(MAX * 0.5));
        assert_eq!(calculate(&format!("{} / 2", MAX)), Ok(MAX / 2.0));
        
        // Minimum value operations
        assert_eq!(calculate(&format!("{} + {}", MIN, -MIN)), Ok(0.0));
        assert_eq!(calculate(&format!("{} * 0.5", MIN)), Ok(MIN * 0.5));
        assert_eq!(calculate(&format!("{} / 2", MIN)), Ok(MIN / 2.0));
    }

    // Complex scientific notation tests
    #[test]
    fn test_complex_scientific_notation() {
        // Large exponents (within f64 range)
        assert_eq!(calculate("1e300 + 1e300"), Ok(2e300));
        assert_eq!(calculate("1e-300 + 1e-300"), Ok(2e-300));
        
        // Mixed exponent signs
        assert_eq!(calculate("1e3 + 1e-3"), Ok(1000.001));
        assert_eq!(calculate("1e-3 + 1e3"), Ok(1000.001));
        
        // Negative exponents
        assert_eq!(calculate("1e-3 * 1e-3"), Ok(1e-6));
        assert_eq!(calculate("1e-6 / 1e-3"), Ok(1e-3));
        
        // Edge cases with exponents
        assert_eq!(calculate("1.0e0 + 1.0e0"), Ok(2.0));
        assert_eq!(calculate("1.0e+0 + 1.0e-0"), Ok(2.0));
        
        // Near maximum exponent
        assert_eq!(calculate("1e307 + 1e307"), Ok(2e307));
    }

    // Complex decimal operations
    #[test]
    fn test_complex_decimal_operations() {
        // Many decimal places
        assert_float_eq(calculate("0.1234567890123456 + 0.1234567890123456").unwrap(), 0.2469135780246912, 1e-15);
        assert_float_eq(calculate("0.1234567890123456 * 2").unwrap(), 0.2469135780246912, 1e-15);
        
        // Decimal precision with large numbers
        assert_float_eq(calculate("123456789.123456789 + 0.000000001").unwrap(), 123456789.12345679, 1e-7);
        assert_float_eq(calculate("123456789.123456789 * 1.000000001").unwrap(), 123456789.24691357, 1e-7);
        
        // Decimal precision with small numbers
        assert_float_eq(calculate("0.000000001 + 0.000000001").unwrap(), 0.000000002, 1e-15);
        assert_float_eq(calculate("0.000000001 * 2").unwrap(), 0.000000002, 1e-15);
    }

    // Precision boundary tests
    #[test]
    fn test_precision_boundaries() {
        // Near epsilon operations
        let epsilon = f64::EPSILON;
        assert_float_eq(calculate(&format!("{} + {}", epsilon, epsilon)).unwrap(), epsilon * 2.0, 1e-15);
        assert_float_eq(calculate(&format!("{} * 2", epsilon)).unwrap(), epsilon * 2.0, 1e-15);
        
        // Operations near precision limits
        assert_float_eq(calculate("0.0000000000000001 + 0.0000000000000001").unwrap(), 2e-16, 1e-15);
        assert_float_eq(calculate("0.0000000000000001 * 2").unwrap(), 2e-16, 1e-15);
        
        // Large number precision - using relative comparison
        let large_num = 1000000000000000.0;
        let result = calculate(&format!("{} + 1", large_num)).unwrap();
        assert_float_eq(result, large_num + 1.0, 1e-14);
        
        let result = calculate(&format!("{} * 1.000000000000001", large_num)).unwrap();
        assert_float_eq(result, large_num * 1.000000000000001, 1e-14);
    }

    // Mixed format tests
    #[test]
    fn test_mixed_formats() {
        // Mixed scientific and decimal
        assert_eq!(calculate("1e3 + 0.001"), Ok(1000.001));
        assert_eq!(calculate("0.001 + 1e3"), Ok(1000.001));
        
        // Mixed negative and scientific
        assert_eq!(calculate("-1e3 + 1e3"), Ok(0.0));
        assert_eq!(calculate("1e3 + -1e3"), Ok(0.0));
        
        // Mixed formats with operations
        assert_eq!(calculate("-1.5e3 * 2.0"), Ok(-3000.0));
        assert_eq!(calculate("2.0 * -1.5e3"), Ok(-3000.0));
    }

    // Extreme precision tests
    #[test]
    fn test_extreme_precision() {
        // Operations at the limit of f64 precision
        let smallest = MIN_POSITIVE;
        assert_float_eq(calculate(&format!("{} + {}", smallest, smallest)).unwrap(), smallest * 2.0, 1e-15);
        assert_float_eq(calculate(&format!("{} * 2", smallest)).unwrap(), smallest * 2.0, 1e-15);
        
        // Operations with numbers very close to each other
        let near_one = 1.0 + f64::EPSILON;
        assert_float_eq(calculate(&format!("{} - 1.0", near_one)).unwrap(), f64::EPSILON, 1e-15);
        assert_float_eq(calculate(&format!("{} / 1.0", near_one)).unwrap(), near_one, 1e-15);
        
        // Operations with numbers that differ by many orders of magnitude
        assert_float_eq(calculate("1e300 + 1e-300").unwrap(), 1e300, 1e-15);
        assert_float_eq(calculate("1e-300 + 1e300").unwrap(), 1e300, 1e-15);
    }

    // Denormal number tests
    #[test]
    fn test_denormal_numbers() {
        // Operations with denormal numbers (numbers smaller than MIN_POSITIVE)
        let denormal = MIN_POSITIVE / 2.0;
        assert_float_eq(calculate(&format!("{} + {}", denormal, denormal)).unwrap(), denormal * 2.0, 1e-15);
        assert_float_eq(calculate(&format!("{} * 2", denormal)).unwrap(), denormal * 2.0, 1e-15);
        
        // Operations that might result in denormal numbers
        let tiny = MIN_POSITIVE * 0.1;
        assert_float_eq(calculate(&format!("{} * 0.1", tiny)).unwrap(), tiny * 0.1, 1e-15);
        assert_float_eq(calculate(&format!("{} / 10", tiny)).unwrap(), tiny / 10.0, 1e-15);
    }

    // Extreme scientific notation tests
    #[test]
    fn test_extreme_scientific_notation() {
        // Maximum exponent with different mantissas (staying within f64 range)
        assert_float_eq(calculate("1.7e300 + 1e300").unwrap(), 2.7e300, 1e-15);
        assert_float_eq(calculate("1.7e300 * 0.5").unwrap(), 0.85e300, 1e-15);
        
        // Minimum exponent with different mantissas
        assert_float_eq(calculate("1e-300 + 1e-300").unwrap(), 2e-300, 1e-15);
        assert_float_eq(calculate("1e-300 * 0.5").unwrap(), 0.5e-300, 1e-15);
        
        // Mixed extreme exponents
        assert_float_eq(calculate("1e300 * 1e-300").unwrap(), 1.0, 1e-15);
        assert_float_eq(calculate("1e-300 * 1e300").unwrap(), 1.0, 1e-15);
    }

    // Complex decimal precision tests
    #[test]
    fn test_complex_decimal_precision() {
        // Many decimal places with different operations
        let pi_like = "3.1415926535897932384626433832795";
        assert_float_eq(calculate(&format!("{} + {}", pi_like, pi_like)).unwrap(), 6.283185307179586, 1e-15);
        assert_float_eq(calculate(&format!("{} * 2", pi_like)).unwrap(), 6.283185307179586, 1e-15);
        
        // Very precise decimal operations
        let precise = "0.12345678901234567890123456789012";
        assert_float_eq(calculate(&format!("{} + {}", precise, precise)).unwrap(), 0.24691357802469136, 1e-15);
        assert_float_eq(calculate(&format!("{} * 2", precise)).unwrap(), 0.24691357802469136, 1e-15);
    }
}
