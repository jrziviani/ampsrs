pub mod parsing {
    use crate::engine::metadata;
    use crate::engine::context;
    use crate::engine::context::StackType;
    use crate::engine::metadata::TokenatorTrait;
    use crate::engine::token_types::TokenTypes;

    pub fn parse(meta: &metadata::Metainfo) {
        let mut ctx: context::Context = context::Context::new();

        for tokens in meta {
            match tokens.get_tokens() {
                Some(tks) => {
                    let mut it = metadata::Tokenator::new(tks);
                    parse_statement(&mut it, &mut ctx);
                },
                None => continue,
            }
        }
        //println!("{:#?}", ctx);
    }

    fn parse_statement<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        match iter.next() {
            Some(tk) => match tk.get_type() {
                TokenTypes::PRINT   => parse_print(iter, ctx),
                TokenTypes::IF      => parse_if(iter, ctx),
                TokenTypes::ELIF    => parse_elif(iter, ctx),
                TokenTypes::ELSE    => parse_else(iter, ctx),
                TokenTypes::ENDIF   => parse_endif(ctx),
                _ => println!("{:?}", tk),
            },
            None => (),
        }
    }

    // parse_print
    // {= expression =} or {% print expression %}
    //
    // Returns an evaluated expression
    fn parse_print<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        if !ctx.branch_is_taken().1 {
            return;
        }

        match iter.look() {
            Some(tk) => {
                if tk.get_type() == TokenTypes::IDENTIFIER {
                    println!("print id:{}", tk.get_data());
                }
                else {
                    parse_expression(iter, ctx);
                    match ctx.stack_pop() {
                        Some(d) => match d {
                            StackType::Text(t) => println!("text: {}", t),
                            StackType::Number(t) => println!("number: {}", t),
                            StackType::Bool(t) => println!("bool: {}", t),
                        },
                        None => {
                            println!("Errors:");
                            for err in ctx.errors_it() {
                                println!("\t{:?}", err);
                            }
                        }
                    }
                }
            },
            None => (),
        }
    }

    // parse_if
    // {% if <identifier|primary> condition <identifier|primary> %}
    //   body
    // {% elif <identifier|primary> condition <identifier|primary> %}
    //   body
    // {% else %}
    //   body
    // {% endif %}
    //
    // Evaluates an expression to boolean. If it evaluates to true, body is executed.
    // elif and else are optional
    fn parse_if<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        // if this block is being parsed but the branch above it was not taken,
        // ignore it and skip the next tokens
        if !ctx.branch_is_taken().1 {
            iter.skip_all();
            return;
        }

        parse_expression(iter, ctx);
        match ctx.stack_pop() {
            Some(d) => match d {
                StackType::Bool(b) => ctx.branch_push(TokenTypes::IF, b),
                _ => ctx.errors_push(String::from("if expression must evaluates to boolean")),
            },
            None => ctx.errors_push(String::from("if expression cannot be evaluated")),
        }
    }

    // parse_elif
    // {% elif <identifier|primary> condition <identifier|primary> %}
    // body
    // {% endif %}
    fn parse_elif<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        let branch = ctx.branch_is_taken();

        // make sure we don't have an 'elif' clause beginning a branch sequence
        if branch.0 != TokenTypes::IF {
            ctx.errors_push(String::from("mimatch elif"));
            return;
        }

        // if the last branch was already taken we can skip this one
        if branch.1 {
            iter.skip_all();
            return;
        }

        if ctx.branch_pop().is_err() {
            ctx.errors_push(String::from("unexpected elif error"));
            return;
        }
        parse_if(iter, ctx);
    }

    // parse_else
    // {% else %}
    // body
    // {% endif %}
    fn parse_else<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        let branch = ctx.branch_is_taken();

        // make sure 'else' matches an 'if' clause
        if branch.0 != TokenTypes::IF {
            ctx.errors_push(String::from("mismatch else"));
            return;
        }

        // if the last branch was already taken we can skip this one
        if branch.1 {
            iter.skip_all();
            return;
        }

        if ctx.branch_pop().is_err() {
            ctx.errors_push(String::from("unexpected else error"));
            return;
        }
        ctx.branch_push(TokenTypes::ELSE, true);
    }

    // parse_endif
    // {% endif %}
    //
    // closes an if block
    fn parse_endif(ctx: &mut context::Context) {
        let branch = ctx.branch_is_taken();

        // make sure that this 'endif' really ends an 'if' clause
        if branch.0 != TokenTypes::IF && branch.0 != TokenTypes::ELSE {
            ctx.errors_push(String::from("mismatch endif"));
            return;
        }

        if ctx.branch_pop().is_err() {
            ctx.errors_push(String::from("unexpected else error"));
            return;
        }
    }

    fn parse_expression<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        parse_equality(iter, ctx)
    }

    fn parse_equality<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        parse_logical(iter, ctx);

        while iter.match_next(TokenTypes::EQ) ||
              iter.match_next(TokenTypes::NE) {

            let oper = iter.look_back().unwrap().get_type();
            parse_logical(iter, ctx);

            let right = ctx.stack_pop().unwrap_or(context::StackType::Text(String::from("<null>")));
            let left = ctx.stack_pop().unwrap_or(context::StackType::Text(String::from("<null>")));
            match compute_binary(left, right, oper) {
                Ok(stk) => ctx.stack_push(stk),
                Err(e) => ctx.errors_push(e),
            }
        }
    }

    fn parse_logical<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        parse_comparison(iter, ctx);

        while iter.match_next(TokenTypes::AND) ||
              iter.match_next(TokenTypes::OR) {

            let oper = iter.look_back().unwrap().get_type();
            parse_comparison(iter, ctx);

            let right = ctx.stack_pop().unwrap_or(context::StackType::Text(String::from("<null>")));
            let left = ctx.stack_pop().unwrap_or(context::StackType::Text(String::from("<null>")));
            match compute_binary(left, right, oper) {
                Ok(stk) => ctx.stack_push(stk),
                Err(e)  => ctx.errors_push(e),
            }
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

            let right = ctx.stack_pop().unwrap_or(context::StackType::Text(String::from("<null>")));
            let left = ctx.stack_pop().unwrap_or(context::StackType::Text(String::from("<null>")));
            match compute_binary(left, right, oper) {
                Ok(stk) => ctx.stack_push(stk),
                Err(e) => ctx.errors_push(e),
            }
        }
    }

    fn parse_addition<I>(iter: &mut I, ctx: &mut context::Context)
    where I: TokenatorTrait {
        parse_multiplication(iter, ctx);

        while iter.match_next(TokenTypes::PLUS) ||
              iter.match_next(TokenTypes::MINUS) {

            let oper = iter.look_back().unwrap().get_type();
            parse_multiplication(iter, ctx);

            let right = ctx.stack_pop().unwrap_or(context::StackType::Text(String::from("<null>")));
            let left = ctx.stack_pop().unwrap_or(context::StackType::Text(String::from("<null>")));
            match compute_binary(left, right, oper) {
                Ok(stk) => ctx.stack_push(stk),
                Err(e) => ctx.errors_push(e),
            }
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

            let right = ctx.stack_pop().unwrap_or(context::StackType::Text(String::from("<null>")));
            let left = ctx.stack_pop().unwrap_or(context::StackType::Text(String::from("<null>")));
            match compute_binary(left, right, oper) {
                Ok(stk) => ctx.stack_push(stk),
                Err(e) => ctx.errors_push(e),
            }
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
                    Some(d) => match d {
                        StackType::Text(t)   => ctx.errors_push(String::from(format!("invalid -{}", t))),
                        StackType::Bool(b)   => ctx.errors_push(String::from(format!("invalid -{}", b))),
                        StackType::Number(n) => ctx.stack_push(StackType::Number(-n)),
                    },
                    None => {
                        ctx.errors_push(String::from(format!("no data retrieved from the stack")));
                    }
                }
            }
            else {
                match last_eval {
                    Some(d) => match d {
                        StackType::Text(t)   => ctx.errors_push(String::from(format!("invalid !{}", t))),
                        StackType::Bool(b)   => ctx.stack_push(StackType::Bool(!b)),
                        StackType::Number(n) => ctx.stack_push(StackType::Number(!n)),
                    },
                    None => {
                        ctx.errors_push(String::from(format!("no data retrieved from the stack")));
                    }
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
                Err(e) => ctx.errors_push(String::from(format!("impossible converting {} to u64 - {}", data, e))),
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
        else if iter.match_next(TokenTypes::LPAREN) {
            parse_expression(iter, ctx);

            if !iter.match_next(TokenTypes::RPAREN) {
                ctx.errors_push(String::from("missing closing"));
            }
        }
        else if iter.match_next(TokenTypes::IDENTIFIER) {
            let key = iter.look_back().unwrap().get_data();
            if !ctx.env_key_exists(&key) {
                ctx.errors_push(String::from(format!("invalid identifier {}", key)));
            }

            let found_left_bracket = match iter.look() {
                Some(tk) => tk.get_type() == TokenTypes::LBRACKET,
                None => false,
            };

            if found_left_bracket {
                iter.next();
                parse_primary(iter, ctx);

                if !iter.match_next(TokenTypes::RBRACKET) {
                    ctx.errors_push(String::from("missing closing ]"));
                }

                match ctx.stack_pop() {
                    Some(d) => match d {
                        StackType::Text(id) => ctx.stack_push_from_env_map(&key, &id),
                        StackType::Number(id) => ctx.stack_push_from_env_vector(&key, id as usize),
                        _ => ctx.errors_push(String::from("invalid id")),
                    },
                    None => {
                        ctx.errors_push(String::from(format!("no data retrieved from the stack")));
                    }
                }
            }
            else {
                ctx.stack_push_from_env(&key);
            }
        }
        else {
            ctx.errors_push(String::from(format!("unexpected token {:#?}", iter.look())));
        }
    }

    fn compute_binary(left: StackType, right: StackType, oper: TokenTypes) -> Result<StackType, String> {
        let ret: Result<StackType, String>;

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
                _ => return Err(String::from("run to the hills")),
            };

            let b = match right {
                StackType::Bool(b) => b,
                _ => return Err(String::from("run to the hills")),
            };

            ret = match oper {
                TokenTypes::AND => Ok(StackType::Bool(a && b)),
                TokenTypes::OR  => Ok(StackType::Bool(a || b)),
                TokenTypes::EQ  => Ok(StackType::Bool(a == b)),
                TokenTypes::NE  => Ok(StackType::Bool(a != b)),
                _ => return Err(String::from(format!("Operator {:#?} invalid for booleans", oper))),
            };
        }
        else {
            let lt = match left {
                StackType::Text(t)   => format!("text({})", t),
                StackType::Bool(b)   => format!("bool({})", b),
                StackType::Number(n) => format!("number({})", n),
            };

            let rt = match right {
                StackType::Text(t)   => format!("text({})", t),
                StackType::Bool(b)   => format!("bool({})", b),
                StackType::Number(n) => format!("number({})", n),
            };

            return Err(String::from(format!("mismatch types {} {:?} {}", lt, oper, rt)));
        }

        ret
    }

    fn compute_strings(left: StackType, right: StackType, oper: TokenTypes) -> Result<StackType, String> {
        let a = match left {
            StackType::Text(b) => b,
            _ => return Err(String::from("run to the hills")),
        };

        let b = match right {
            StackType::Text(b) => b,
            _ => return Err(String::from("run to the hills")),
        };

        let result = match oper {
            TokenTypes::NE   => StackType::Bool(a != b),
            TokenTypes::EQ   => StackType::Bool(a == b),
            TokenTypes::GT   => StackType::Bool(a > b),
            TokenTypes::GE   => StackType::Bool(a >= b),
            TokenTypes::LT   => StackType::Bool(a < b),
            TokenTypes::LE   => StackType::Bool(a <= b),
            TokenTypes::PLUS => StackType::Text(a + &b),
            _ => return Err(String::from(format!("Operator {:#?} invalid for strings", oper))),
        };

        Ok(result)
    }

    fn compute_numbers(left: StackType, right: StackType, oper: TokenTypes) -> Result<StackType, String> {
        let a = match left {
            StackType::Number(b) => b,
            _ => return Err(String::from("run to the hills")),
        };

        let b = match right {
            StackType::Number(b) => b,
            _ => return Err(String::from("run to the hills")),
        };

        let result = match oper {
            TokenTypes::NE      => StackType::Bool(a != b),
            TokenTypes::EQ      => StackType::Bool(a == b),
            TokenTypes::GT      => StackType::Bool(a > b),
            TokenTypes::GE      => StackType::Bool(a >= b),
            TokenTypes::LT      => StackType::Bool(a < b),
            TokenTypes::LE      => StackType::Bool(a <= b),
            TokenTypes::PLUS    => {
                match a.checked_add(b) {
                    Some(i) => StackType::Number(i),
                    None    => return Err(String::from(format!("{} + {} overflows", a, b))),
                }
            },
            TokenTypes::MINUS   => {
                match a.checked_sub(b) {
                    Some(i) => StackType::Number(i),
                    None    => return Err(String::from(format!("{} - {} underflows", a, b))),
                }
            },
            TokenTypes::STAR    => {
                match a.checked_mul(b) {
                    Some(i) => StackType::Number(i),
                    None    => return Err(String::from(format!("{} * {} overflows", a, b))),
                }
            }
            TokenTypes::SLASH   => {
                match a.checked_div(b) {
                    Some(i) => StackType::Number(i),
                    None    => return Err(String::from(format!("{} / {} division by 0", a, b))),
                }
            },
            TokenTypes::PERCENT => {
                match a.checked_rem(b) {
                    Some(i) => StackType::Number(i),
                    None    => return Err(String::from(format!("{} / {} division by 0", a, b))),
                }
            },
            _ => return Err(String::from(format!("Operator {:#?} invalid for numbers", oper))),
        };

        Ok(result)
    }
}
