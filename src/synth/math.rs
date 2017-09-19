use super::*;

pub struct Add {
    terms: Vec<GenBox>,
    buf: SampleBuffer,
}

impl Generator for Add {
    fn eval<'a>(&'a mut self, params: &Parameters) -> &'a SampleBuffer {
        if self.terms.is_empty() {
            self.buf.zero();
        } else {
            let (first, next) = self.terms.split_at_mut(1);
            self.buf.update_from(first[0].eval(params));
            for term in next {
                self.buf.sum_into(term.eval(params));
            }
        }
        &self.buf
    }
}

pub struct Mul {
    factors: Vec<GenBox>,
    buf: SampleBuffer,
}

impl Generator for Mul {
    fn eval<'a>(&'a mut self, params: &Parameters) -> &'a SampleBuffer {
        if self.factors.is_empty() {
            self.buf.zero();
        } else {
            let (first, next) = self.factors.split_at_mut(1);
            self.buf.update_from(first[0].eval(params));
            for factor in next {
                self.buf.mul_into(factor.eval(params));
            }
        }
        &self.buf
    }
}