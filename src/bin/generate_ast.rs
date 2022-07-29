use std::{
    env,
    fs,
    str
};

fn main()
{
    env::args().skip(1).for_each(|arg| {
        match fs::read(&arg)
        {
            Ok(data) => match str::from_utf8(&data)
            {
                Ok(text) => match fs::write(&arg, generate_ast(text))
                {
                    Ok(()) => {},
                    Err(err) => println!("Error writing {arg}: {err}")
                },
                Err(err) => println!("Error reading {arg}: {err}")
            },
            Err(err) => println!("Error reading {arg}: {err}")
        }
    });
}

fn generate_ast(mut slice: &str) -> String
{
    const MAIN_TYPE: &str = "Expr";
    const GEN_MARKER: &str = "\n// autogenerated code\n";
    const TYPE_CODE: &str = "\npub struct TYPE\n{\nARGSN\n}\n";
    const MAIN_TRAIT: &str = "\npub trait MAIN\n{\n\tMETHODS\n}\n";
    const MAIN_ATTR: &str = "\tpub NAME: TYPE;\n";
    const ATTR_DECL: &str = "\tfn NAME(&self) -> TYPE;\n";
    const MAIN_VISIT: &str = "\tfn VERB(&self) -> RETURNS;\n";
    const MAIN_VISIT_BODY: &str = "\n\t{ VISIT::visit_TYPEL(self) }";
    const VISIT_TRAIT: &str = "\ntrait Visitor<I>\n{\nMETHODS}\n";
    const VISIT_METHOD: &str = "\tfn visit_TYPEL(expr: &TYPE) -> I;\n";
    const VISIT_CODE: &str = "\nimpl MAIN for TYPE\n{\n\tMETHODS\n}\n";

    if let Some(index) = slice.find(GEN_MARKER)
    { slice = &slice[..index]; }
    let mut text: String = slice.to_string();
    let mut pos = 0;
    let mut visitors: Vec<(&str, &str, &str)> = Vec::new();
    let mut attrs: Vec<(&str, &str)> = Vec::new();

    // find and remember implementors
    while pos < slice.len()
    {
        // TODO: merge if let chaining becomes stable
        if let Some(index) = slice[pos..]
            .find("// impl Visitor<").map(|i| i + pos)
        {
            if let Some(post_type) = slice[index..]
                .find("> for").map(|i| i + index)
            {
                if let Some(post_struct) = slice[post_type..]
                    .find(':').map(|i| i + post_type)
                {
                    if let Some(end) = slice[post_struct..]
                        .find(';').map(|i| i + post_struct)
                    {
                        visitors.push((&slice[index + 16 .. post_type].trim(),
                            &slice[post_type + 5 .. post_struct].trim(),
                            &slice[post_struct + 1 .. end].trim()));
                        pos = index + 1;
                    }
                    else { break }
                }
                else { break }
            }
            else { break }
        }
        else { break }
    }

    // find and remember attributes
    pos = 0;
    while pos < slice.len()
    {
        // TODO: merge if let chaining becomes stable
        if let Some(index) = slice[pos..]
            .find("// attr").map(|i| i + pos)
        {
            if let Some(post_name) = slice[index..]
                .find(":").map(|i| i + index)
            {
                if let Some(end) = slice[post_name..]
                    .find(";").map(|i| i + post_name)
                {
                    attrs.push((&slice[index + 7 .. post_name].trim(),
                        &slice[post_name + 1 .. end].trim()));
                    pos = index + 1
                }
                else { break }
            }
            else { break }
        }
        else { break }
    }

    text.push_str(GEN_MARKER);

    let mut main_methods = String::new();
    let main_attrs = &attrs.iter().map(|tuple| {
        let (name, kind) = tuple;
        MAIN_ATTR.to_string()
            .replace("NAME", name)
            .replace("TYPE", kind)
    }).collect::<Vec<String>>().join("");
    let attr_decl = &attrs.iter().map(|tuple| {
        let (name, kind) = tuple;
        ATTR_DECL.to_string()
            .replace("NAME", name)
            .replace("TYPE", kind)
    }).collect::<Vec<String>>().join("");
    let attr_impl = &attrs.iter().map(|tuple| {
        let (name, kind) = tuple;
        ATTR_DECL.to_string()
            .replace(";", "\n\t{ self.NAME }")
            .replace("NAME", name)
            .replace("TYPE", kind)
    }).collect::<Vec<String>>().join("");
    if attrs.len() != 0
    {
        main_methods.push_str(&attrs.iter().map(|tuple| {
            let (name, kind) = tuple;
            ATTR_DECL.to_string()
                .replace("NAME", name)
                .replace("TYPE", kind)
        }).collect::<Vec<String>>().join(""));
        main_methods.push_str("\n");
    }
    if visitors.len() != 0
    {
        main_methods.push_str(&visitors.iter().map(|tuple| {
            let (returns, kind, verb) = tuple;
            MAIN_VISIT.to_string()
                .replace("VERB", verb)
                .replace("TYPE", kind)
                .replace("RETURNS", returns)
        }).collect::<Vec<String>>().join(""));
        main_methods.push_str("\n");
    }
    text.push_str(&MAIN_TRAIT.to_string()
        .replace("MAIN", MAIN_TYPE)
        .replace("METHODS", &main_methods.trim()));

    let expr_types = Vec::from([
        ("Binary", "left: Box<dyn Expr>, oper: TokenType, right: Box<dyn Expr>"),
        ("Grouping", "expr: Box<dyn Expr>"),
        ("Literal", "value: LoxValue"),
        ("Unary", "oper: TokenType, expr: Box<dyn Expr>")
    ]);
    let mut visit_methods = String::new();
    if attrs.len() != 0
    {
        //visit_methods.push_str();
    }
    if visitors.len() != 0
    {
        text.push_str(&VISIT_TRAIT.to_string()
            .replace("METHODS", &expr_types.iter().map(|tuple| {
                let (kind, _) = tuple;
                let kindl = &kind.to_lowercase();
                VISIT_METHOD.to_string()
                    .replace("TYPEL", kindl)
                    .replace("TYPE", kind)
            }).collect::<Vec<String>>().join("")));
        visit_methods.push_str(&visitors.iter().map(|tuple| {
            let (returns, kind, verb) = tuple;
            MAIN_VISIT.to_string()
                .replace("VERB", verb)
                .replace("TYPE", kind)
                .replace("RETURNS", returns)
                .replace(";", &MAIN_VISIT_BODY.to_string()
                    .replace("VISIT", kind))
        }).collect::<Vec<String>>().join(""));
    }
    let mut visit_body = attr_impl.to_string();
    if attrs.len() != 0
    {
        visit_body.push('\n');
    }
    if visitors.len() != 0
    {
        visit_body.push_str(&visit_methods);
        visit_body.push('\n');
    }
    expr_types.iter().for_each(|tuple| {
        let (kind, args) = tuple;
        let kindl = &kind.to_lowercase();
        let mut argsn = main_attrs.to_string()
            .replace(";", ",");
        argsn.push_str("\tpub ");
        argsn.push_str(&args.to_string()
            .replace(", ", ",\n\tpub "));
        text.push_str(&TYPE_CODE.to_string()
            .replace("TYPE", kind)
            .replace("ARGSN", &argsn)
            .replace("ARGS", &args));
        text.push_str(&VISIT_CODE.to_string()
            .replace("MAIN", MAIN_TYPE)
            .replace("METHODS", &visit_body.trim())
            .replace("TYPEL", kindl)
            .replace("TYPE", kind));
    });

    text
}
