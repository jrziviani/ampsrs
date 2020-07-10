pub mod parsing {
    use crate::metadata;
    use crate::context;
    use crate::context::StackType;
    use crate::metadata::TokenatorTrait;
    use crate::token_types::TokenTypes;

    pub fn parse(meta: &metadata::Metainfo) {
        let mut ctx: context::Context = context::Context::new();

        //println!("{:#?}", meta);

        for tokens in meta {
            match tokens.get_tokens() {
                Some(tks) => {
                    let mut it = metadata::Tokenator::new(tks);
                    parse_statement(&mut it, &mut ctx);
                },
                None => continue,
            }
        }

        println!("{:#?}", ctx);
    }

    fn parse_statement<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        match iter.next() {
            Some(tk) => match tk.get_type() {
                TokenTypes::PRINT => parse_print(iter, ctx),
                _ => println!("{:?}", tk),
            },
            None => (),
        }
    }

    fn parse_print<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        match iter.look() {
            Some(tk) => {
                if tk.get_type() == TokenTypes::IDENTIFIER {
                    println!("print id:{}", tk.get_data());
                }
                else {
                    parse_expression(iter, ctx);
                    match ctx.stack_pop() {
                        StackType::Text(t) => println!("text: {}", t),
                        StackType::Number(t) => println!("number: {}", t),
                        StackType::Bool(t) => println!("bool: {}", t),
                    }
                }
            },
            None => panic!("invalid print format"),
        }
    }

    fn parse_expression<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        parse_equality(iter, ctx);
    }

    fn parse_equality<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        parse_logical(iter, ctx);

        while iter.match_next(TokenTypes::EQ) ||
              iter.match_next(TokenTypes::NE) {

            let oper = iter.look_back().unwrap().get_type();
            parse_logical(iter, ctx);

            let right = ctx.stack_pop();
            let left = ctx.stack_pop();
            ctx.stack_push(compute_binary(left, right, oper));
        }
    }

    fn parse_logical<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        parse_comparison(iter, ctx);

        while iter.match_next(TokenTypes::AND) ||
              iter.match_next(TokenTypes::OR) {

            let oper = iter.look_back().unwrap().get_type();
            parse_comparison(iter, ctx);

            let right = ctx.stack_pop();
            let left = ctx.stack_pop();
            ctx.stack_push(compute_binary(left, right, oper));
        }
    }

    fn parse_comparison<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        parse_addition(iter, ctx);

        while iter.match_next(TokenTypes::GT) ||
              iter.match_next(TokenTypes::GE) ||
              iter.match_next(TokenTypes::LT) ||
              iter.match_next(TokenTypes::LE) {

            let oper = iter.look_back().unwrap().get_type();
            parse_addition(iter, ctx);

            let right = ctx.stack_pop();
            let left = ctx.stack_pop();
            ctx.stack_push(compute_binary(left, right, oper));
        }
    }

    fn parse_addition<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        parse_multiplication(iter, ctx);

        while iter.match_next(TokenTypes::PLUS) ||
              iter.match_next(TokenTypes::MINUS) {

            let oper = iter.look_back().unwrap().get_type();
            parse_multiplication(iter, ctx);

            let right = ctx.stack_pop();
            let left = ctx.stack_pop();
            ctx.stack_push(compute_binary(left, right, oper));
        }
    }

    fn parse_multiplication<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        parse_unary(iter, ctx);

        while iter.match_next(TokenTypes::STAR) ||
              iter.match_next(TokenTypes::PERCENT) ||
              iter.match_next(TokenTypes::SLASH) {

            let oper = iter.look_back().unwrap().get_type();
            parse_unary(iter, ctx);

            let right = ctx.stack_pop();
            let left = ctx.stack_pop();
            ctx.stack_push(compute_binary(left, right, oper));
        }
    }

    fn parse_unary<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        if iter.match_next(TokenTypes::MINUS) ||
           iter.match_next(TokenTypes::NOT) {

            let operator = iter.look_back().unwrap().get_type();

            // handle sequence of unary operators like recursively
            // !!!!var or ---3
            parse_unary(iter, ctx);

            let last_eval = ctx.stack_pop();
            if operator == TokenTypes::MINUS {
                match last_eval {
                    StackType::Text(t) => panic!("invalid -{}", t),
                    StackType::Bool(b) => panic!("invalid -{}", b),
                    StackType::Number(n) => ctx.stack_push(StackType::Number(-n)),
                }
            }
            else {
                match last_eval {
                    StackType::Text(t) => panic!("invalid !{}", t),
                    StackType::Bool(b) => ctx.stack_push(StackType::Bool(!b)),
                    StackType::Number(n) => ctx.stack_push(StackType::Number(!n)),
                }
            }
        }
        else {
            parse_primary(iter, ctx);
        }
    }

    fn parse_primary<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        if iter.match_next(TokenTypes::NUMBER) {
            let data = iter.look_back().unwrap().get_data();
            match data.parse::<i64>() {
                Ok(n) => ctx.stack_push(StackType::Number(n)),
                Err(e) => panic!("impossible converting {} to u64 - {}", data, e),
            }
        }
        else if iter.match_next(TokenTypes::STRING) {
            let data = iter.look_back().unwrap().get_data();
            ctx.stack_push(StackType::Text(data));
        }
        else if iter.match_next(TokenTypes::TRUE) {
            ctx.stack_push(StackType::Bool(true));
        }
        else if iter.match_next(TokenTypes::FALSE) {
            ctx.stack_push(StackType::Bool(false));
        }
        else {
            panic!("unexpected token {:#?}", iter.look());
        }
    }

    fn compute_binary(left: StackType, right: StackType, oper: TokenTypes) -> StackType {
        let ret: StackType;

        if matches!(left, StackType::Text(_)) && matches!(right, StackType::Text(_)) {
            ret = compute_strings(left, right, oper);
        }
        else if matches!(left, StackType::Number(_)) && matches!(right, StackType::Number(_)) {
            ret = compute_numbers(left, right, oper);
        }
        else if matches!(left, StackType::Bool(_)) &&
                matches!(right, StackType::Bool(_)) {
            let a = match left {
                StackType::Bool(b) => b,
                _ => panic!("run to the hills"),
            };

            let b = match right {
                StackType::Bool(b) => b,
                _ => panic!("run to the hills"),
            };

            ret = match oper {
                TokenTypes::AND => StackType::Bool(a && b),
                TokenTypes::OR  => StackType::Bool(a || b),
                TokenTypes::EQ  => StackType::Bool(a == b),
                TokenTypes::NE  => StackType::Bool(a != b),
                _ => panic!("Operator {:#?} invalid for booleans", oper),
            };
        }
        else {
            panic!("mimatch types {:?} and {:?}", left, right);
        }

        ret
    }

    fn compute_strings(left: StackType, right: StackType, oper: TokenTypes) -> StackType {
        let a = match left {
            StackType::Text(b) => b,
            _ => panic!("run to the hills"),
        };

        let b = match right {
            StackType::Text(b) => b,
            _ => panic!("run to the hills"),
        };

        match oper {
            TokenTypes::NE   => StackType::Bool(a != b),
            TokenTypes::EQ   => StackType::Bool(a == b),
            TokenTypes::GT   => StackType::Bool(a > b),
            TokenTypes::GE   => StackType::Bool(a >= b),
            TokenTypes::LT   => StackType::Bool(a < b),
            TokenTypes::LE   => StackType::Bool(a <= b),
            TokenTypes::PLUS => StackType::Text(a + &b),
            _ => panic!("Operator {:#?} invalid for strings", oper),
        }
    }

    fn compute_numbers(left: StackType, right: StackType, oper: TokenTypes) -> StackType {
        let a = match left {
            StackType::Number(b) => b,
            _ => panic!("run to the hills"),
        };

        let b = match right {
            StackType::Number(b) => b,
            _ => panic!("run to the hills"),
        };

        match oper {
            TokenTypes::NE      => StackType::Bool(a != b),
            TokenTypes::EQ      => StackType::Bool(a == b),
            TokenTypes::GT      => StackType::Bool(a > b),
            TokenTypes::GE      => StackType::Bool(a >= b),
            TokenTypes::LT      => StackType::Bool(a < b),
            TokenTypes::LE      => StackType::Bool(a <= b),
            TokenTypes::PLUS    => {
                match a.checked_add(b) {
                    Some(i) => StackType::Number(i),
                    None => panic!("{} + {} underflows", a, b),
                }
            },
            TokenTypes::MINUS   => {
                match a.checked_sub(b) {
                    Some(i) => StackType::Number(i),
                    None => panic!("{} - {} underflows", a, b),
                }
            },
            TokenTypes::STAR    => {
                match a.checked_mul(b) {
                    Some(i) => StackType::Number(i),
                    None => panic!("{} * {} overflows", a, b),
                }
            }
            TokenTypes::SLASH   => {
                match a.checked_div(b) {
                    Some(i) => StackType::Number(i),
                    None => panic!("{} / {} division by 0", a, b),
                }
            },
            TokenTypes::PERCENT => {
                match a.checked_rem(b) {
                    Some(i) => StackType::Number(i),
                    None => panic!("{} % {} division by 0", a, b),
                }
            },
            _ => panic!("Operator {:#?} invalid for strings", oper),
        }
    }
}
