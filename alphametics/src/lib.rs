use std::collections::HashMap;

type HasCarry = Option<bool>;

#[derive(Debug)]
struct Relation {
    lhs_members: Vec<char>,
    result: char,
    has_carry: bool,
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut result: HashMap<char, u8> = HashMap::new();

    let (lhs_members, rhs_member) = get_equation_members(input);

    let has_last_column_carry = lhs_members
        .clone()
        .into_iter()
        .any(|x| x.len() < rhs_member.len());
    result.insert(rhs_member.chars().nth(0).unwrap(), 1);

    // relation to satiyfy, like E + 1 = N
    // letter, letter, result, has_carry
    // if has_carry, then result >= 10
    // type HasCarry -> true, false, maybe
    // decrement column and add [column - 1] that has_carry
    println!("equation_members: {:?}", get_equation_members(input));
    println!("result: {:?}", result);

    create_relation(&lhs_members, &rhs_member);
    Some(result)
}

fn create_relation(lhs_members: &Vec<String>, rhs_member: &String) -> () {
    let mut result: HashMap<char, u8> = HashMap::new();

    let mut prim_relations: Vec<Vec<char>> = vec![];
    let max_lhs_length = lhs_members.into_iter().map(|m| m.len()).max().unwrap();

    for i in 0..max_lhs_length {
        prim_relations.push(vec![]);
    }

    for member in lhs_members.into_iter() {
        let offset = max_lhs_length - member.len();
        for (i, c) in member.chars().enumerate() {
            prim_relations[i + offset].push(c);
        }
    }

    println!("max length: {}", max_lhs_length);
    println!("prim_relations: {:?}", prim_relations);

    let mut relations: Vec<Relation> = vec![];

    if rhs_member.len() > max_lhs_length {
        result.insert(rhs_member.chars().nth(0).unwrap(), 1);

        relations.push(Relation {
            lhs_members: vec![],
            result: rhs_member.chars().nth(0).unwrap(),
            has_carry: true,
        })
    }

    for (i, relation) in prim_relations.into_iter().enumerate() {
        for member in relation {}

        relations.push(Relation {
            lhs_members: vec![],
            result: rhs_member.chars().nth(0).unwrap(),
            has_carry: true,
        })
    }

    println!("relations: {:?}", relations);
    println!("result: {:?}", result);
}

fn get_equation_members(input: &str) -> (Vec<String>, String) {
    let mut lhs_members: Vec<String> = vec![];
    let mut lhs_temp_member: Vec<char> = vec![];
    let mut rhs_member: String = String::new();

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            lhs_temp_member.push(c);
        } else if !lhs_temp_member.is_empty() {
            lhs_members.push(lhs_temp_member.clone().into_iter().collect::<String>());
            lhs_temp_member.drain(0..);
        }

        if c.is_whitespace() && input.chars().nth(i - 1).unwrap() == '=' {
            rhs_member = input.get(i + 1..).unwrap().to_string();
        }
    }

    (lhs_members, rhs_member)
}
