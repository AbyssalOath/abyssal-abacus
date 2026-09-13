//! Core calculator logic shared between the CLI and GUI front ends.
//!
//! This crate has no I/O and no UI dependencies at all, so it compiles
//! instantly and can be unit tested in isolation. Both `abyssal_abacus_cli` and
//! `abyssal_abacus_gui` depend on it, which is what "ties the two together": they
//! are two different front ends wrapping the exact same math.

/// Evaluate a simple two-operand text expression like "3+4", "10 / 2",
/// or "-5*3". This is the original CLI parser: it scans for the first
/// "real" operator (skipping unary minus signs) and evaluates
/// `left OP right`.
pub fn evaluate_expr(expr: &str) -> Result<f64, String> {
    let operators = ['+', '-', '*', '/', '%'];

    for op in &operators {
        if let Some(pos) = find_operator(expr, *op) {
            let left_str = expr[..pos].trim();
            let right_str = expr[pos + 1..].trim();

            let a: f64 = left_str
                .parse()
                .map_err(|_| format!("Invalid number: '{}'", left_str))?;
            let b: f64 = right_str
                .parse()
                .map_err(|_| format!("Invalid number: '{}'", right_str))?;

            return apply_op(*op, a, b);
        }
    }

    Err(format!(
        "Invalid expression: '{}'. Use format: a+b, a-b, a*b, a/b, a%b",
        expr
    ))
}

fn apply_op(op: char, a: f64, b: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Ok(0.0)
            } else {
                Ok(a / b)
            }
        }
        '%' => {
            if b == 0.0 {
                Ok(0.0)
            } else {
                Ok(a % b)
            }
        }
        _ => Err("Unknown operator".to_string()),
    }
}

fn find_operator(expr: &str, op: char) -> Option<usize> {
    let chars: Vec<char> = expr.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c == op {
            // A leading '-' is a sign, not subtraction.
            if op == '-' && i == 0 {
                continue;
            }
            // A '-' right after another operator makes the next number
            // negative (e.g. "5*-3"), it isn't subtraction either.
            if op == '-' && i > 0 {
                let prev = chars[i - 1];
                if "+-*/%".contains(prev) {
                    continue;
                }
            }
            return Some(i);
        }
    }

    None
}

// ---------------------------------------------------------------------
// Stateful engine for button/keyboard-driven UIs (used by the GUI).
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl Op {
    fn apply(self, a: f64, b: f64) -> Result<f64, String> {
        match self {
            Op::Add => Ok(a + b),
            Op::Sub => Ok(a - b),
            Op::Mul => Ok(a * b),
            Op::Div => {
                if b == 0.0 {
                    Ok(0.0)
                } else {
                    Ok(a / b)
                }
            }
            Op::Rem => {
                if b == 0.0 {
                    Ok(0.0)
                } else {
                    Ok(a % b)
                }
            }
        }
    }

    pub fn symbol(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
            Op::Rem => '%',
        }
    }
}

/// A small "four-function" calculator engine: it tracks the number
/// currently being typed, a pending operator, and an accumulator. This is
/// the same model a physical calculator uses (and matches the reference
/// image): press digits, press an operator, press more digits, press `=`.
#[derive(Debug, Clone)]
pub struct CalcEngine {
    display: String,
    accumulator: Option<f64>,
    pending_op: Option<Op>,
    just_evaluated: bool,
    pub error: Option<String>,
}

impl Default for CalcEngine {
    fn default() -> Self {
        Self {
            display: "0".to_string(),
            accumulator: None,
            pending_op: None,
            just_evaluated: false,
            error: None,
        }
    }
}

impl CalcEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display(&self) -> &str {
        &self.display
    }

    /// Which operator (if any) is currently pending, for UI highlighting.
    pub fn pending_op(&self) -> Option<Op> {
        self.pending_op
    }

    pub fn input_digit(&mut self, d: char) {
        if !d.is_ascii_digit() {
            return;
        }
        self.error = None;
        if self.just_evaluated {
            self.display.clear();
            self.just_evaluated = false;
        }
        if self.display == "0" {
            self.display = d.to_string();
        } else {
            self.display.push(d);
        }
    }

    pub fn input_dot(&mut self) {
        self.error = None;
        if self.just_evaluated {
            self.display = "0".to_string();
            self.just_evaluated = false;
        }
        if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    pub fn input_operator(&mut self, op: Op) {
        self.error = None;
        let current: f64 = self.display.parse().unwrap_or(0.0);

        if let (Some(acc), Some(pending)) = (self.accumulator, self.pending_op) {
            match pending.apply(acc, current) {
                Ok(result) => {
                    self.accumulator = Some(result);
                    self.display = format_number(result);
                }
                Err(e) => {
                    self.error = Some(e);
                    self.clear_all();
                    return;
                }
            }
        } else {
            self.accumulator = Some(current);
        }

        self.pending_op = Some(op);
        self.just_evaluated = true; // next digit starts a fresh number
    }

    pub fn equals(&mut self) {
        self.error = None;
        let current: f64 = self.display.parse().unwrap_or(0.0);

        if let (Some(acc), Some(op)) = (self.accumulator, self.pending_op) {
            match op.apply(acc, current) {
                Ok(result) => self.display = format_number(result),
                Err(e) => {
                    self.error = Some(e);
                    self.clear_all();
                    return;
                }
            }
        }

        self.accumulator = None;
        self.pending_op = None;
        self.just_evaluated = true;
    }

    pub fn clear(&mut self) {
        self.clear_all();
    }

    fn clear_all(&mut self) {
        self.display = "0".to_string();
        self.accumulator = None;
        self.pending_op = None;
        self.just_evaluated = false;
    }

    pub fn backspace(&mut self) {
        self.error = None;
        if self.just_evaluated {
            return;
        }
        self.display.pop();
        if self.display.is_empty() || self.display == "-" {
            self.display = "0".to_string();
        }
    }

    pub fn toggle_sign(&mut self) {
        self.error = None;
        if self.display == "0" {
            return;
        }
        if let Some(rest) = self.display.strip_prefix('-') {
            self.display = rest.to_string();
        } else {
            self.display = format!("-{}", self.display);
        }
    }

    pub fn percent(&mut self) {
        self.error = None;
        if let Ok(v) = self.display.parse::<f64>() {
            self.display = format_number(v / 100.0);
        }
    }
}

fn format_number(n: f64) -> String {
    if !n.is_finite() {
        return "Error".to_string();
    }
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        let s = format!("{:.10}", n);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_expr() {
        assert_eq!(evaluate_expr("3+4"), Ok(7.0));
        assert_eq!(evaluate_expr("10/2"), Ok(5.0));
        assert_eq!(evaluate_expr("-5+3"), Ok(-2.0));
        assert_eq!(evaluate_expr("5*-3"), Ok(-15.0));
    }

    #[test]
    fn divide_by_zero_expr() {
        assert_eq!(evaluate_expr("5/0"), Ok(0.0));
    }

    #[test]
    fn divide_by_small_decimal_is_unaffected() {
        let result = evaluate_expr("5/0.01153").unwrap();
        assert!(result > 400.0 && result < 450.0); // ~433.65, well away from 0
    }

    #[test]
    fn engine_chain() {
        let mut c = CalcEngine::new();
        c.input_digit('3');
        c.input_operator(Op::Add);
        c.input_digit('4');
        c.equals();
        assert_eq!(c.display(), "7");
    }

    #[test]
    fn engine_chained_ops() {
        // 5 + 2 * 3 evaluated left-to-right (no precedence), like a
        // simple physical calculator: (5+2)=7, then 7*3=21.
        let mut c = CalcEngine::new();
        c.input_digit('5');
        c.input_operator(Op::Add);
        c.input_digit('2');
        c.input_operator(Op::Mul);
        c.input_digit('3');
        c.equals();
        assert_eq!(c.display(), "21");
    }

    #[test]
    fn engine_divide_by_zero() {
        let mut c = CalcEngine::new();
        c.input_digit('5');
        c.input_operator(Op::Div);
        c.input_digit('0');
        c.equals();
        assert!(c.error.is_none());
        assert_eq!(c.display(), "0");
    }

    #[test]
    fn engine_backspace_and_sign() {
        let mut c = CalcEngine::new();
        c.input_digit('1');
        c.input_digit('2');
        c.input_digit('3');
        c.backspace();
        assert_eq!(c.display(), "12");
        c.toggle_sign();
        assert_eq!(c.display(), "-12");
    }
}
