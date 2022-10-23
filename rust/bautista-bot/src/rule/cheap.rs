use super::RuleEval;

#[derive(Clone, Copy, Debug)]
pub struct RuleCheap {
    hours: i64,
}

impl RuleCheap {
    pub fn new(hours: i64) -> RuleCheap {
        RuleCheap { hours }
    }
}

impl RuleEval for RuleCheap {
    fn eval(&self) -> Option<bool> {
        Some(true)
    }

    fn is_consumed(&self) -> bool {
        false
    }
}
