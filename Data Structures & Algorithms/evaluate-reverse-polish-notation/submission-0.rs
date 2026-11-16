impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];
    for token in tokens {
        match token.as_str() {
            "+" => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                let res = num1 + num2;
                stack.push(res);
            }
            "-" => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                let res = num2 - num1;
                stack.push(res);
            }
            "/" => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                let res = num2 / num1;
                stack.push(res);
            }
            "*" => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                let res = num1*num2;
                stack.push(res);
            }
            _ => {
                stack.push(token.parse().unwrap());
            }
        }
    }

    stack.pop().unwrap()
    }
}
