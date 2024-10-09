use std::ops;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Value {
    data: usize,
    grad: usize,
    _childrens: Vec<Value>,
    _backward: Option<dyn fn()>,
}

impl Value {
    fn new(data: usize) -> Self {
        Self {
            data,
            grad: 0,
            _childrens: Vec::new(),
        }
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, other: Value) -> Self::Output {
        let data: usize = self.data + other.data;
        let mut _childrens: Vec<Value> = Vec::new();
        _childrens.push(self);
        _childrens.push(other);
        Value {
            data,
            grad: 0,
            _childrens,
        }
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Self::Output {
        let data: usize = self.data * other.data;
        let mut _childrens: Vec<Value> = Vec::new();
        _childrens.push(self);
        _childrens.push(other);
        Value {
            data,
            grad: 0,
            _childrens: _childrens,
        }
    }
}

#[cfg(test)]
mod micro_grad_simple_test {
    use super::*;

    #[test]
    fn add() {
        let result = Value::new(3) + Value::new(4);
        assert_eq!(result.data, 7);
    }

    #[test]
    fn mul() {
        let result = Value::new(3) * Value::new(4);
        assert_eq!(result.data, 12);
    }

    #[test]
    fn temp_test() {
        let result = Value::new(3) + Value::new(4);
        let new_result = result * Value::new(2);
        println!("{:?}", new_result._childrens);
        println!("{}", new_result.data);
    }
}
