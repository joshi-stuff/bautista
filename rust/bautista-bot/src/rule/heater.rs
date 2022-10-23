use super::RuleEval;

#[derive(Clone, Copy, Debug)]
pub struct RuleHeater {
    pivot_hour: i64,
}

impl RuleHeater {
    pub fn new(pivot_hour: i64) -> RuleHeater {
        RuleHeater { pivot_hour }
    }
}

impl RuleEval for RuleHeater {
    fn eval(&self) -> Option<bool> {
        Some(true)
    }

    fn is_consumed(&self) -> bool {
        false
    }
}
