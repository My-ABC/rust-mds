#[derive(Clone)]
#[derive(PartialEq)]
enum Token {
    EOF,
    Int(i64),
    Float(f64),

    Add,
    Sub,
    Mul,
    Div,
    Pow,

    LParen,
    RParen,
}

fn to_string(t: &Token) -> String {
    match t {
        Token::EOF => "EOF".to_string(),
        Token::Int(i) => i.to_string(),
        Token::Float(f) => f.to_string(),

        Token::Add => "Add".to_string(),
        Token::Sub => "Sub".to_string(),
        Token::Mul => "Mul".to_string(),
        Token::Div => "Div".to_string(),
        Token::Pow => "Pow".to_string(),

        Token::LParen => "(".to_string(),
        Token::RParen => ")".to_string(),
    }
}

struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
        }
    }

    fn next_token(&mut self) -> Result<Vec<Token>, String> {
        let mut v: Vec<Token> = Vec::new();

        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();

            match c {
                '0'..='9' => {
                    let num = self.number();
                    match num {
                        Ok(n) => {
                            v.push(n);
                            continue;
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                '+' => {
                    v.push(Token::Add);
                    self.position += 1;
                },
                '-' => {
                    v.push(Token::Sub);
                    self.position += 1;
                },
                '*' => {
                    v.push(Token::Mul);
                    self.position += 1;
                },
                '/' => {
                    v.push(Token::Div);
                    self.position += 1;
                },
                '^' => {
                    v.push(Token::Pow);
                    self.position += 1;
                }
                '(' => {
                    v.push(Token::LParen);
                    self.position += 1;
                },
                ')' => {
                    v.push(Token::RParen);
                    self.position += 1;
                },
                ' ' | '\t' | '\n' => {
                    self.position += 1;
                    continue;
                },
                _ => {
                    return Err(format!("Unexpected character: {:?}", c));
                }
            }
        }

        v.push(Token::EOF);
        return Ok(v);
    }

    fn number(&mut self) -> Result<Token, String> {
        let mut num = "".to_string();
        let mut d = 0;

        loop {
            if self.position >= self.input.len() {
                break;
            };
            let c = self.input.chars().nth(self.position).unwrap();

            match c {
                '0'..='9' => {
                    num.push(c);
                    self.position += 1;
                },
                '.' => {
                    if d > 1 {
                        return Err("Too many decimal points".to_string());
                    }
                    num.push_str(".");
                    d += 1;
                    self.position += 1;
                }
                _ => {
                    break;
                }
            }
        }

        if d == 0 {
            match i64::from_str_radix(&num, 10) {
                Ok(n) => {
                    return Ok(Token::Int(n));
                },
                Err(_) => {
                    return Err("Invalid integer".to_string());
                }
            }
        } else {
            if &num.chars().nth(&num.len() - 1).unwrap() == &".".chars().nth(0).unwrap() {
                return Err("Invalid floating point".to_string());
            } else {
                match &num.parse::<f64>() {
                    Ok(n) => {
                        return Ok(Token::Float(*n));
                    },
                    Err(_) => {
                        return Err("Invalid floating point".to_string());
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
enum NodeType {
    Text(String),
    Node(Node),
}

impl NodeType {

    fn get_s(&self) -> String {
        match self {
            NodeType::Text(s) => {
                return s.clone();
            },
            NodeType::Node(_) => {
                return "".to_string();
            }
        }
    }

    fn get_n(&self) -> Node {
        match self {
            NodeType::Text(_) => {
                return Node::new("".to_string());
            },
            NodeType::Node(n) => {
                return n.clone();
            }
        }
    }
}

#[derive(Clone)]
struct Node {
    name : String,
    children : Vec<NodeType>,
}

impl Node {
    fn new(name : String) -> Self {
        Node {
            name,
            children : Vec::new(),
        }
    }


    fn add_child(&mut self, child : NodeType) {
        self.children.push(child);
    }

    /*fn repr(&self) -> String {


        let children_repr = self.children.iter()
            .map(|child| match child {
                NodeType::Node(node) => node.repr(),
                NodeType::Text(text) => text.clone(),
            })
            .collect::<Vec<_>>()
            .join(",");
        
        format!("{}({})", self.name, children_repr)
    }*/
    
}

struct Perser {
    tokens: Vec<Token>,
    position: usize,
}

impl Perser {
    fn new(tokens: Vec<Token>) -> Self {
        Perser {
            tokens,
            position: 0,
        }
    }

    fn parser(&mut self) -> Result<Node, String> {
        return self.expr();
    }

    fn expr(&mut self) -> Result<Node, String> {
        let mut node = Node::new("BinOp".to_string());
        let mut o_node = node.clone();

        // term (('+'|'-') term)*
        let left = &self.term()?;
        node.add_child(NodeType::Node(left.clone()));

        if self.position >= self.tokens.len() {
            return Ok(left.clone());
        }

        let mut i = 0;
        loop {
            let t = self.tokens.get(self.position).unwrap();
            if !matches!(t, Token::Add | Token::Sub) {
                if i == 0 {
                    return Ok(left.clone());
                }
                break;
            }
            if self.position >= self.tokens.len() {
                return Err("Unexpected end of input".to_string());
            }
            let op = self.tokens.get(self.position).unwrap();
            node.add_child(NodeType::Text(to_string(op)));
            self.position += 1;
            let right = self.term()?;
            node.add_child(NodeType::Node(right.clone()));

            if self.tokens.get(self.position).unwrap() == &Token::EOF {
                return Ok(node.clone());
            }

            o_node = node.clone();
            node = Node::new("BinOp".to_string());
            node.add_child(NodeType::Node(o_node.clone()));

            i += 1;
        }

        if node.children.len() == 1 {
            return Ok(o_node);
        }

        return Ok(node);
    }

    fn term(&mut self) -> Result<Node, String> {
        let mut node = Node::new("BinOp".to_string());
        let mut o_node: Node = node.clone();

        // factor (('*'|'/') factor)*
        let left = &self.factor()?;
        node.add_child(NodeType::Node(left.clone()));

        if self.position >= self.tokens.len() {
            return Ok(left.clone());
        }
        let mut i = 0;
        loop {
            let t = self.tokens.get(self.position).unwrap();
            if !matches!(t, Token::Mul | Token::Div) {
                if i == 0 {
                    return Ok(left.clone());
                }
                break;
            }
            if self.position >= self.tokens.len() {
                return Err("Unexpected end of input".to_string());
            }
            let op = self.tokens.get(self.position).unwrap();
            node.add_child(NodeType::Text(to_string(op)));
            self.position += 1;
            let right = self.factor()?;
            node.add_child(NodeType::Node(right.clone()));

            if self.tokens.get(self.position).unwrap() == &Token::EOF {
                return Ok(node.clone());
            }

            o_node = node.clone();
            node = Node::new("BinOp".to_string());
            node.add_child(NodeType::Node(o_node.clone()));

            i += 1;
        }

        if node.children.len() == 1 {
            return Ok(o_node);
        }

        return Ok(node);
    }

    fn factor(&mut self) -> Result<Node, String> {
        let mut unode = Node::new("UnaryOp".to_string());
        
        // ('+'|'-') factor
        // power

        let t = self.tokens.get(self.position).unwrap();
        if matches!(t, Token::Add | Token::Sub) {
            unode.add_child(NodeType::Text(to_string(t)));
            self.position += 1;
            if self.position >= self.tokens.len() {
                return Err("Unexpected end of input".to_string());
            }

            let right = self.factor();
            match right {
                Ok(v) => {
                    unode.add_child(NodeType::Node(v));
                    return Ok(unode);
                },
                Err(e) => {
                    return Err(e);
                }
            }
        } else {
            let p = self.power();
            return p;
        }
    }

    fn power(&mut self) -> Result<Node, String> {
        let mut node = Node::new("BinOp".to_string());

        // atom ['^' power]

        let left = &self.atom()?;
        node.add_child(NodeType::Node(left.clone()));

        if self.position >= self.tokens.len() {
            return Ok(left.clone());
        }

        let t = self.tokens.get(self.position).unwrap();
        if !matches!(t, Token::Pow) {
            if self.position >= self.tokens.len() {
                return Err("Unexpected end of input".to_string());
            }
            return Ok(left.clone());
        } else {
            if self.position >= self.tokens.len() {
                return Err("Unexpected end of input".to_string());
            }
            node.add_child(NodeType::Text(to_string(t)));
            self.position += 1;

            let right = self.power();
            match right {
                Ok(v) => {
                    node.add_child(NodeType::Node(v));
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }

        return Ok(node);
    }

    fn atom(&mut self) -> Result<Node, String> {
        // (INT | FLOAT)
        // '(' expr ')'

        let t = self.tokens.get(self.position).unwrap();
        match t {
            Token::Int(_) => {
                let mut node = Node::new("Int".to_string());
                node.add_child(NodeType::Text(to_string(self.tokens.get(self.position).unwrap())));
                self.position += 1;
                return Ok(node);
            },
            Token::Float(_) => {
                let mut node = Node::new("Float".to_string());
                node.add_child(NodeType::Text(to_string(self.tokens.get(self.position).unwrap())));
                self.position += 1;
                return Ok(node);
            },
            _ => {
                if matches!(self.tokens.get(self.position).unwrap(), Token::LParen) {
                    self.position += 1;
                    if self.position >= self.tokens.len() {
                        return Err("Unexpected end of input".to_string());
                    }
                    let node = self.expr();
                    if self.position >= self.tokens.len() {
                        return Err("Unexpected end of input".to_string());
                    }
                    match node {
                        Ok(node) => {
                            if matches!(self.tokens.get(self.position).unwrap(), Token::RParen) {
                                self.position += 1;
                                return Ok(node);
                            } else {
                                return Err("Expected ')'".to_string());
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                } else {
                    return Err("Expected atom".to_string());
                }
            }
        }
    }
}

#[derive(Clone)]
struct ByteCodes {
    codes : Vec<ByteCode>
}

impl ByteCodes {
    fn new() -> Self {
        Self {
            codes : Vec::new()
        }
    }

    fn add_code(&mut self, code : ByteCode) {
        self.codes.push(code);
    }
}

#[derive(Clone)]
enum ByteCode {
    PUSHI(i64), // i32 is a int value
    PUSHF(f64), // f32 is a float value
    BINOP(i32), // i32 is opc 0: ADD, 1: SUB, 2: MUL, 3: DIV, 4: POW
    UNARYOP(i32) // i32 is opc 0: ADD 1: SUB
}

/*impl ByteCodes {
    fn dis(&self) {
        for code in &self.codes {
            match code {
                ByteCode::PUSHI(i) => {
                    println!("PUSHI {}", i);
                },
                ByteCode::PUSHF(f) => {
                    println!("PUSHF {}", f);
                },
                ByteCode::BINOP(opc) => {
                    println!("BINOP {}", opc);
                },
                ByteCode::UNARYOP(opc) => {
                    println!("UNARYOP {}", opc);
                }
            }
        }
    }
}*/

struct Dis {
    b : ByteCodes
}

impl Dis {
    fn new () -> Dis {
        Dis {
            b : ByteCodes::new()
        }
    }

    fn dis(&mut self, asts: Node) -> ByteCodes {
        if asts.name == "Int"{
            self.b.add_code(ByteCode::PUSHI(
                i64::from_str_radix(asts.children[0].get_s().as_str(), 10).unwrap()
            ));
        } else if asts.name == "Float"{
            self.b.add_code(ByteCode::PUSHF(
                asts.children[0].get_s().as_str().parse::<f64>().unwrap()
            ));
        } else if asts.name == "BinOp"{
            self.dis(asts.children[0].get_n());
            self.dis(asts.children[2].get_n());
            if asts.children[1].get_s() == "Add" {
                self.b.add_code(ByteCode::BINOP(0));
            } else if asts.children[1].get_s() == "Sub" {
                self.b.add_code(ByteCode::BINOP(1));
            } else if asts.children[1].get_s() == "Mul" {
                self.b.add_code(ByteCode::BINOP(2));
            } else if asts.children[1].get_s() == "Div" {
                self.b.add_code(ByteCode::BINOP(3));
            } else if asts.children[1].get_s() == "Pow" {
                self.b.add_code(ByteCode::BINOP(4));
            }
        } else if asts.name == "UnaryOp"{
            self.dis(asts.children[1].get_n());
            if asts.children[0].get_s() == "Add" {
                self.b.add_code(ByteCode::UNARYOP(0));
            } else if asts.children[0].get_s() == "Sub" {
                self.b.add_code(ByteCode::UNARYOP(1));
            }
        }

        return self.b.clone();
    }
}

struct VM {
    b: ByteCodes,
    s: Vec<i64>,
    s2: Vec<f64>,

    now: Vec<NowType>,

    val: Vec<NowType>
}

#[derive(Clone)]
enum NowType {
    Int(i64),
    Float(f64),
}

impl NowType {
    fn get(&mut self) {
        match self {
            NowType::Int(i) => {
                println!("{}", *i);
            },
            NowType::Float(f) => {
                println!("{}", *f);
            }
        }
    }
}

impl VM {
    fn new(b: ByteCodes) -> Self {
        VM {
            b,
            s: vec![],
            s2: vec![],
            now: vec![],

            val: vec![]
        }
    }

    fn run(&mut self) -> Vec<NowType> {
        for bc in self.b.codes.iter() {
            match bc {
                ByteCode::PUSHI(i) => {
                    self.now.push(NowType::Int(*i));
                    self.s.push(*i);
                    self.val.push(NowType::Int(*i));
                },
                ByteCode::PUSHF(f) => {
                    self.now.push(NowType::Float(*f));
                    self.s2.push(*f);
                    self.val.push(NowType::Float(*f));
                },
                ByteCode::BINOP(op) => {
                    let mut v = self.now.pop();
                    if matches!(v, Some(NowType::Int(_))) {
                        v = self.now.pop();
                        if matches!(v, Some(NowType::Int(_))) {
                            let b = self.s.pop().unwrap();
                            let a = self.s.pop().unwrap();

                            match op {
                                0 => {
                                    self.now.push(NowType::Int(a+b));
                                    self.s.push(a+b);
                                    self.val.push(NowType::Int(a+b));
                                },
                                1 => {
                                    self.now.push(NowType::Int(a-b));
                                    self.s.push(a-b);
                                    self.val.push(NowType::Int(a-b));
                                },
                                2 => {
                                    self.now.push(NowType::Int(a*b));
                                    self.s.push(a*b);
                                    self.val.push(NowType::Int(a*b));
                                },
                                3 => {
                                    self.now.push(NowType::Int(a/b));
                                    self.s.push(a/b);
                                    self.val.push(NowType::Int(a/b));
                                },
                                4 => {
                                    self.now.push(NowType::Int(a.pow(b as u32)));
                                    self.s.push(a.pow(b as u32));
                                    self.val.push(NowType::Int(a.pow(b as u32)));
                                },
                                _ => {
                                    println!("Not Def op");
                                }
                                
                            }
                        } else if matches!(v, Some(NowType::Float(_))) {
                            let b = self.s.pop().unwrap() as f64;
                            let a = self.s2.pop().unwrap();

                            match op {
                                0 => {
                                    self.now.push(NowType::Float(a+b));
                                    self.s2.push(a+b);
                                    self.val.push(NowType::Float(a+b));
                                },
                                1 => {
                                    self.now.push(NowType::Float(a-b));
                                    self.s2.push(a-b);
                                    self.val.push(NowType::Float(a-b));
                                },
                                2 => {
                                    self.now.push(NowType::Float(a*b));
                                    self.s2.push(a*b);
                                    self.val.push(NowType::Float(a*b));
                                },
                                3 => {
                                    self.now.push(NowType::Float(a/b));
                                    self.s2.push(a/b);
                                    self.val.push(NowType::Float(a/b));
                                },
                                4 => {
                                    self.now.push(NowType::Float(a.powf(b)));
                                    self.s2.push(a.powf(b));
                                    self.val.push(NowType::Float(a.powf(b)));
                                },
                                _ => {
                                    println!("Not Def op");
                                }
                                
                            }
                        }
                    } else if matches!(v, Some(NowType::Float(_))) {
                        v = self.now.pop();
                        if matches!(v, Some(NowType::Int(_))) {
                            let b = self.s2.pop().unwrap();
                            let a = self.s.pop().unwrap() as f64;

                            match op {
                                0 => {
                                    self.now.push(NowType::Float(a+b));
                                    self.s2.push(a+b);
                                    self.val.push(NowType::Float(a+b));
                                },
                                1 => {
                                    self.now.push(NowType::Float(a-b));
                                    self.s2.push(a-b);
                                    self.val.push(NowType::Float(a-b));
                                },
                                2 => {
                                    self.now.push(NowType::Float(a*b));
                                    self.s2.push(a*b);
                                    self.val.push(NowType::Float(a*b));
                                },
                                3 => {
                                    self.now.push(NowType::Float(a/b));
                                    self.s2.push(a/b);
                                    self.val.push(NowType::Float(a/b));
                                },
                                4 => {
                                    self.now.push(NowType::Float(a.powf(b)));
                                    self.s2.push(a.powf(b));
                                    self.val.push(NowType::Float(a.powf(b)));
                                },
                                _ => {
                                    println!("Not Def op");
                                }
                                
                            }
                        } else if matches!(v, Some(NowType::Float(_))) {
                            let b = self.s2.pop().unwrap();
                            let a = self.s2.pop().unwrap();

                            match op {
                                0 => {
                                    self.now.push(NowType::Float(a+b));
                                    self.s2.push(a+b);
                                    self.val.push(NowType::Float(a+b));
                                },
                                1 => {
                                    self.now.push(NowType::Float(a-b));
                                    self.s2.push(a-b);
                                    self.val.push(NowType::Float(a-b));
                                },
                                2 => {
                                    self.now.push(NowType::Float(a*b));
                                    self.s2.push(a*b);
                                    self.val.push(NowType::Float(a*b));
                                },
                                3 => {
                                    self.now.push(NowType::Float(a/b));
                                    self.s2.push(a/b);
                                    self.val.push(NowType::Float(a/b));
                                },
                                4 => {
                                    self.now.push(NowType::Float(a.powf(b)));
                                    self.s2.push(a.powf(b));
                                    self.val.push(NowType::Float(a.powf(b)));
                                },
                                _ => {
                                    println!("Not Def op");
                                }
                                
                            }
                        }
                    }
                    
                },
                ByteCode::UNARYOP(op) => {
                    let v = self.now.pop();
                    if matches!(v, Some(NowType::Int(_))) {
                        let a = self.s.pop().unwrap();

                        match op {
                            0 => {
                                self.now.push(NowType::Int(a));
                                self.s.push(a);
                                self.val.push(NowType::Int(a));
                            },
                            1 => {
                                self.now.push(NowType::Int(-a));
                                self.s.push(-a);
                                self.val.push(NowType::Int(-a));
                            },
                            _ => println!("Invalid unary operator"),
                        }
                    } else if matches!(v, Some(NowType::Float(_))) {
                        let a = self.s2.pop().unwrap();

                        match op {
                            0 => {
                                self.now.push(NowType::Float(a));
                                self.s2.push(a);
                                self.val.push(NowType::Float(a));
                            },
                            1 => {
                                self.now.push(NowType::Float(-a));
                                self.s2.push(-a);
                                self.val.push(NowType::Float(-a));
                            },
                            _ => println!("Invalid unary operator"),
                        }
                    }
                }
            }
        }

        return self.val.clone();
    }
}

fn main() {
    loop {
        let mut input = String::new();
        let inp = std::io::stdin().read_line(&mut input);
        match inp {
            Ok(0) => {break;},
            Ok(_) => {
                let inp = input.trim().to_string();
                let mut lexer = Lexer::new(inp.to_string());
                let token = lexer.next_token();
                // let mut text: String = "[".to_string();

                match token {
                    Ok(v) => {

                        let ast = &Perser::new(v).parser();
                        match ast {
                            Ok(v) => {
                                let asts = v.clone();
                                let mut dis = Dis::new();
                                //dis.dis(ast.clone()).dis();

                                let mut vm = VM::new(dis.dis(asts));
                                let mut v = vm.run();
                                v.pop().unwrap().get();
                            },
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    }
                    Err(e) => {println!("{}", e);}
                }
            },
            Err(e) => {println!("{}", e);}
        }
    }
}

