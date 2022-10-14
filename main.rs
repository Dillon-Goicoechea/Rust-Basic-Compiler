
#![allow(unused_imports)]
#![allow(unused_results)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate queues;
use queues::*;
use std::env;
use std::fs;
use std::iter::FromIterator;
use std::iter::Iterator;
use std::mem::discriminant;
use std::collections::HashSet;
use std::process::Output;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
#[derive(PartialEq,Eq)]
pub enum Tokens{ //Token list
    Id(Vec<char>),
    Num(Vec<char>),
    Semicolon,
    Colon,
    Comma,
    Period,
    LParen,
    RParen,
    Assign,
    Definitions,
    Operations,
    Point,
    Circle,
    Square,
    Print,
    Contained,
    Intersects,
    End,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Definition{ //used for symbol table
    id: String,
    shapetype: String,
    value1: String,
    value2: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation{ // used for symbol table
    function: String,
    id1: String,
    id2: String
}

impl Definition{
    fn new(id: String, shape: String, val1:String, val2:String)-> Definition{ //creates definition for symbol table
        let definition = Definition{
            id: id,
            shapetype: shape,
            value1: val1,
            value2: val2
        };
        return definition;
    }

    fn id(&self) -> String { //returns objects id
        return self.id.clone();
    }

    fn shape(&self) -> String { //returns objects shape
        return self.shapetype.clone();
    }

    fn value1(&self) -> String { //returns objects value1
        return self.value1.clone();
    }

    fn value2(&self) -> String { //returns objects value2
        return self.value2.clone();
    }
}

impl Operation{
    fn new(func: String, id1:String, id2:String)-> Operation{ //creates new object for symbol table
        let operation = Operation{
            function: func,
            id1: id1,
            id2: id2
        };
        return operation;
    }

    fn function(&self) -> String { //returns objects function/command
        return self.function.clone();
    }

    fn id1(&self) -> String { //returns objects first id
        return self.id1.clone();
    }

    fn id2(&self) -> String { //returns objects second id
        return self.id2.clone();
    }

}

fn tokens_eq(a: &Tokens, b: &Tokens) -> bool { //Checks that two tokens share same enum variant
   if std::mem::discriminant(a) == std::mem::discriminant(b) {
    return true;
   }
   else{
    return false;
   }
}

fn lookup_multichar_token(lexemme: String) -> Tokens{ //finds multi character tokens for lexical analysis
    match lexemme.as_str() {
        "definitions" => return Tokens::Definitions ,
        "operations" => return Tokens::Operations ,
        "point" => return Tokens::Point ,
        "circle" => return Tokens::Circle ,
        "square" => return Tokens::Square ,
        "print" => return Tokens::Print ,
        "contained" => return Tokens::Contained ,
        "intersects" => return Tokens::Intersects ,
        "end" => return Tokens::End ,
        _ => {},
    }
    let mut digits_found =0;
    let mut letters_found =0;
    for chr in lexemme.chars() {
        if chr.is_ascii_digit() { 
            digits_found+=1;
        }
        else if chr.is_ascii_lowercase() {
             letters_found+=1;
            }
    }
    if  (digits_found > 0) && (letters_found > 0) {
        let error_string: String = "Lexical Error: ".to_string();
        panic!("{}", (error_string + &lexemme));
    }
    else if (digits_found > 0) && (letters_found ==0) { 
        let number: Vec<char> = lexemme.chars().collect();
        return Tokens::Num(number);
     }
    else if (letters_found > 0) && (digits_found ==0) { 
        let id: Vec<char> = lexemme.chars().collect();
        return Tokens::Id(id);
     }
    else { panic!("Unknown Lexical Error")}
}

fn lexical_analysis(file_name: String) -> Vec<Tokens>{ //Lexer
    let mut program_tokens: Vec<Tokens> = Vec::new(); //List of token found in file
    let mut lexemme_collector: Vec<char> = Vec::new();
    println!("; Processing Input File {}", file_name);
    let contents = fs::read_to_string(file_name).expect("Error: File could Not be Read");
    let input: Vec<char> = contents.chars().filter(|c| !c.is_whitespace()).collect();
   for lexemme in input {
        match lexemme{
            '=' => { let lexemme_collector_string: String = lexemme_collector.iter().collect();
                    if !lexemme_collector.is_empty() {program_tokens.push(lookup_multichar_token(lexemme_collector_string));}
                    program_tokens.push(Tokens::Assign);
                    lexemme_collector.clear();
                    continue;
                },
            ';' => {let lexemme_collector_string: String = lexemme_collector.iter().collect();
                    if !lexemme_collector.is_empty() {program_tokens.push(lookup_multichar_token(lexemme_collector_string));}
                    program_tokens.push(Tokens::Semicolon);
                    lexemme_collector.clear();
                    continue;
                    },
            ':' => {let lexemme_collector_string: String = lexemme_collector.iter().collect();
                    if !lexemme_collector.is_empty() {program_tokens.push(lookup_multichar_token(lexemme_collector_string));}
                    program_tokens.push(Tokens::Colon);
                    lexemme_collector.clear();
                    continue;
                    },
            ',' => {let lexemme_collector_string: String = lexemme_collector.iter().collect();
                    if !lexemme_collector.is_empty() {program_tokens.push(lookup_multichar_token(lexemme_collector_string));}
                    program_tokens.push(Tokens::Comma);
                    lexemme_collector.clear();
                    continue;
                    },
            '.' => {let lexemme_collector_string: String = lexemme_collector.iter().collect();
                    if !lexemme_collector.is_empty() {program_tokens.push(lookup_multichar_token(lexemme_collector_string));}
                    program_tokens.push(Tokens::Period);
                    lexemme_collector.clear();
                    continue;
                    },
            '(' => {let lexemme_collector_string: String = lexemme_collector.iter().collect();
                    if !lexemme_collector.is_empty() {program_tokens.push(lookup_multichar_token(lexemme_collector_string));}
                    program_tokens.push(Tokens::LParen);
                    lexemme_collector.clear();
                    continue;
                    },
            ')' => {let lexemme_collector_string: String = lexemme_collector.iter().collect();
                    if !lexemme_collector.is_empty() {program_tokens.push(lookup_multichar_token(lexemme_collector_string));}
                    program_tokens.push(Tokens::RParen);
                    lexemme_collector.clear();
                    continue;
                    },
             _  => {},
        }
        if lexemme.is_ascii_digit() {
           lexemme_collector.push(lexemme);
        }
        else if lexemme.is_ascii_lowercase() {
            lexemme_collector.push(lexemme);
        }
        else{
            let mut error_string: String = "Lexical Error: ".to_string();
            error_string.push(lexemme);
            panic!("{}", error_string);
        }
    }
    return program_tokens;
}
fn syntax_analysis(program_tokens: &Vec<Tokens>){ //Parser
    let mut definitions_or_operations = 0;
    for (i , token) in program_tokens.iter().enumerate(){
        match token{
            Tokens::Definitions => {
                if tokens_eq(&program_tokens[i+1],&Tokens::Colon) {
                    continue;
                }
                else {
                    panic!("Syntax Error: Expected :");
                }
            }
            Tokens::Colon => {
                if definitions_or_operations == 0{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Id(Vec::<char>::new())) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected Id Declaration");
                    }
                 }
                else{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Print) {
                        continue;
                    }
                    else if tokens_eq(&program_tokens[i+1],&Tokens::Contained) {
                        continue;
                    }
                    else if tokens_eq(&program_tokens[i+1],&Tokens::Intersects) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected contained, print, or intersects");
                    }
                }
            }
            Tokens::Semicolon => {
                if definitions_or_operations == 0{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Id(Vec::<char>::new())) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected Id Declaration");
                    }
                 }
                else{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Print) {
                        continue;
                    }
                    else if tokens_eq(&program_tokens[i+1],&Tokens::Contained) {
                        continue;
                    }
                    else if tokens_eq(&program_tokens[i+1],&Tokens::Intersects) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected contained, print, or intersects");
                    }
                }
            }
            Tokens::Assign => {
                if definitions_or_operations == 0{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Point) {
                        continue;
                    }
                    else if tokens_eq(&program_tokens[i+1],&Tokens::Circle) {
                        continue;
                    }
                    else if tokens_eq(&program_tokens[i+1],&Tokens::Square) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected circle,square, or point");
                    }
                 }
                else{
                    panic!("Syntax Error: Assignment Outside of Declarations");
                }
            }
            Tokens::Point =>{
                if definitions_or_operations == 0{
                    if tokens_eq(&program_tokens[i+1],&Tokens::LParen) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected ( ");
                    }
                 }
                else{
                    panic!("Syntax Error: Point Outside of Declarations");
                }
            }
            Tokens::Circle =>{
                if definitions_or_operations == 0{
                    if tokens_eq(&program_tokens[i+1],&Tokens::LParen) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected ( ");
                    }
                 }
                else{
                    panic!("Syntax Error: Point Outside of Declarations");
                }
            }
            Tokens::Square =>{
                if definitions_or_operations == 0{
                    if tokens_eq(&program_tokens[i+1],&Tokens::LParen) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected ( ");
                    }
                 }
                else{
                    panic!("Syntax Error: Point Outside of Declarations");
                }
            }
            Tokens::LParen =>{
                if definitions_or_operations == 0{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Num(Vec::<char>::new())) {
                        continue;
                    }
                    else if tokens_eq(&program_tokens[i+1],&Tokens::Id(Vec::<char>::new())) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected Num or Id ");
                    }
                 }
                else{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Id(Vec::<char>::new())) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected Id");
                    }
                }
            }
            Tokens::Comma =>{
                if definitions_or_operations == 0{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Num(Vec::<char>::new())) {
                        continue;
                    }
                    else if tokens_eq(&program_tokens[i+1],&Tokens::Id(Vec::<char>::new())) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected Num or Id ");
                    }
                 }
                else{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Id(Vec::<char>::new())) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected Id");
                    }
                }
            }
            Tokens::RParen => {
                if definitions_or_operations == 0 {
                    if tokens_eq(&program_tokens[i+1],&Tokens::Semicolon) {
                        continue;
                    }
                    else if tokens_eq(&program_tokens[i+1],&Tokens::Operations) {
                        definitions_or_operations =1;
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected ; or operations ");
                    } 
                }
                else{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Semicolon) {
                        continue;
                    }
                    else if tokens_eq(&program_tokens[i+1],&Tokens::End) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected ; or end ");
                    }
                }
            }
            Tokens::End =>{
                if definitions_or_operations == 0 {
                    panic!("Syntax Error: expectated operations before end ");
                }
                else{
                    if tokens_eq(&program_tokens[i+1],&Tokens::Period) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected . ");
                    }
                }
            }
            Tokens::Period =>{
                if definitions_or_operations == 0 {
                    panic!("Syntax Error: expectated operations before end ");
                }
                else{
                    if tokens_eq(&program_tokens[i-1],&Tokens::End) {
                        continue;
                    }
                    else {
                        panic!("Syntax Error: Expected end before . ");
                    }
                }
            }
            _ => {}
        }
        if tokens_eq(&token,&Tokens::Id(Vec::<char>::new())) {
            if definitions_or_operations == 0 {
                if tokens_eq(&program_tokens[i+1],&Tokens::Assign) {
                    continue;
                }
                else  if tokens_eq(&program_tokens[i+1],&Tokens::Comma) {
                    continue;
                }
                else  if tokens_eq(&program_tokens[i+1],&Tokens::RParen) {
                    continue;
                }
                else{
                    panic!("Syntax Error: Expected = , ',' or ) " );
                }
            }
            else{
                if tokens_eq(&program_tokens[i+1],&Tokens::Comma) {
                    continue;
                }
                else  if tokens_eq(&program_tokens[i+1],&Tokens::RParen) {
                    continue;
                }
                else{
                    panic!("Syntax Error: Expected ',' or ) " );
                }
            }
        }
        if tokens_eq(&token,&Tokens::Num(Vec::<char>::new())) {
            if definitions_or_operations == 0 {
               if tokens_eq(&program_tokens[i+1],&Tokens::Comma) {
                    continue;
                }
                else  if tokens_eq(&program_tokens[i+1],&Tokens::RParen) {
                    continue;
                }
                else{
                    panic!("Syntax Error: Expected ',' or ) " );
                }
            }
            else{
                panic!(" Syntax Error: Num after definitions")
            }
        }
    }  
}

fn scheme_output(program_tokens: Vec<Tokens>,filename: &String){ //Outputs scheme using token list
    let mut definitions_or_operations = 0;
    let mut object_fields : Queue<String> = Queue::new();
    let mut definitions_list: Vec<Definition> = Vec::new();
    let mut operations_list: Vec<Operation> = Vec::new();
    for token in program_tokens.iter(){
        if definitions_or_operations ==0{
            match token{
                Tokens::Assign|Tokens::LParen|Tokens::Comma|Tokens::RParen =>{
                    continue;
                }
                Tokens::Point =>{
                    object_fields.add(" (makepoint ".to_string()).ok();
                    continue;
                }
                Tokens::Circle =>{
                    object_fields.add("-circle".to_string()).ok();
                    continue;
                }
                Tokens::Square =>{
                    object_fields.add("-square".to_string()).ok();
                    continue;
                }
                Tokens::Semicolon =>{
                    let idval: String = object_fields.remove().unwrap();
                    let shapetype: String = object_fields.remove().unwrap();
                    let val1: String = object_fields.remove().unwrap();
                    let val2: String = object_fields.remove().unwrap();

                    definitions_list.push(Definition::new(idval,shapetype,val1,val2));
                    continue;
                }
                Tokens::Operations =>{
                    let idval: String = object_fields.remove().unwrap();
                    let shapetype: String = object_fields.remove().unwrap();
                    let val1: String = object_fields.remove().unwrap();
                    let val2: String = object_fields.remove().unwrap();

                    definitions_list.push(Definition::new(idval,shapetype,val1,val2));
                    definitions_or_operations +=1;
                    continue;
                }
                _ =>{}
            }
            if tokens_eq(&token,&Tokens::Id(Vec::<char>::new())) {
                if let Tokens::Id(value) = token {
                    let stringform: String = value.iter().collect();
                    object_fields.add(stringform).ok();
                    continue;
                }
            
            } 
            if tokens_eq(&token,&Tokens::Num(Vec::<char>::new())) {
                if let Tokens::Num(value) = token {
                    let stringform: String = value.iter().collect();
                    object_fields.add(stringform).ok();
                    continue;
                }
            }
        }
        else{
            match token{
                Tokens::LParen|Tokens::Comma|Tokens::RParen =>{
                    continue;
                }
                Tokens::Print =>{
                    object_fields.add("(print".to_string()).ok();
                    continue;
                }
                Tokens::Contained =>{
                    object_fields.add("(contained".to_string()).ok();
                    continue;
                }
                Tokens::Intersects =>{
                    object_fields.add("(intersects".to_string()).ok();
                    continue;
                }
                Tokens::Semicolon =>{
                    let func: String = object_fields.remove().unwrap();
                    let id1: String = object_fields.remove().unwrap();
                    if object_fields.size() !=0{
                        let id2: String = object_fields.remove().unwrap();
                        }
                        let id2: String = "".to_string();

                    operations_list.push(Operation::new(func,id1,id2));
                    continue;
                }
                Tokens::End =>{
                    let func: String = object_fields.remove().unwrap();
                    let id1: String = object_fields.remove().unwrap();
                    if object_fields.size() !=0{
                    let id2: String = object_fields.remove().unwrap();
                    operations_list.push(Operation::new(func,id1,id2));
                    }
                    else{
                    let id2: String = "".to_string();
                    operations_list.push(Operation::new(func,id1,id2));
                    }
                    continue;
                }
                _ =>{}
            }
            if tokens_eq(&token,&Tokens::Id(Vec::<char>::new())) {
                if let Tokens::Id(value) = token {
                    let stringform: String = value.iter().collect();
                    object_fields.add(stringform).ok();
                    continue;
                }
            } 
        }
    }

    for operation in operations_list.iter() {
        let mut output_string:String = String::new();
        let mut val1_type: String = String::new();
        let mut val2_type: String = String::new();
        let mut val3_value: String = String::new();
        let mut val4_value: String = String::new();
        let mut val1_value: String = String::new();
        let mut val2_value: String = String::new();
        output_string = output_string + &operation.function();

        for definition in definitions_list.iter(){
            if definition.id() == operation.id1() {
                val1_type = definition.shape();
                for definition2 in definitions_list.iter(){
                    if definition.value1() == definition2.id() {
                        val1_value = definition2.shape() + &definition2.value1() +&" ".to_string() + &definition2.value2() + &")".to_string();
                    }
                }
                val3_value = definition.value2();
            }
        }
        if operation.id2() != "" {
            for definition in definitions_list.iter(){
                if definition.id() == operation.id2() {
                    val2_type = definition.shape() + &" ".to_string();
                    for definition2 in definitions_list.iter(){
                        if definition.value1() == definition2.id() {
                            val2_value = definition2.shape() + &definition2.value1() +&" ".to_string() + &definition2.value2() +&")".to_string() ;
                        }
                    }
                    val4_value = definition.value2();
                }
            }
            output_string = output_string + &val1_type + &val2_type + &val1_value + &" ".to_string() + &val3_value + &val2_value + &" ".to_string() + &val4_value + &")".to_string();
            println!("{}",output_string);
        }
        else{
            output_string = output_string + &val1_type + &" ".to_string() + &val1_value + &" ".to_string() + &val3_value + &"))".to_string();
            println!("{}",output_string);
        }
    }

    extra_credit_parse_tree(program_tokens,filename);
}

fn prolog_ouput(program_tokens: Vec<Tokens>,filename: &String){ //outputs prolog using tokens list
    let mut definitions_or_operations = 0;
    let mut object_fields : Queue<String> = Queue::new();
    let mut definitions_list: Vec<Definition> = Vec::new();
    let mut operations_list: Vec<Operation> = Vec::new();
    for token in program_tokens.iter(){
        if definitions_or_operations ==0{
            match token{
                Tokens::Assign|Tokens::LParen|Tokens::Comma|Tokens::RParen =>{
                    continue;
                }
                Tokens::Point =>{
                    object_fields.add("(point2d(".to_string()).ok();
                    continue;
                }
                Tokens::Circle =>{
                    object_fields.add("circle".to_string()).ok();
                    continue;
                }
                Tokens::Square =>{
                    object_fields.add("square".to_string()).ok();
                    continue;
                }
                Tokens::Semicolon =>{
                    let idval: String = object_fields.remove().unwrap();
                    let shapetype: String = object_fields.remove().unwrap();
                    let val1: String = object_fields.remove().unwrap();
                    let val2: String = object_fields.remove().unwrap();

                    definitions_list.push(Definition::new(idval,shapetype,val1,val2));
                    continue;
                }
                Tokens::Operations =>{
                    let idval: String = object_fields.remove().unwrap();
                    let shapetype: String = object_fields.remove().unwrap();
                    let val1: String = object_fields.remove().unwrap();
                    let val2: String = object_fields.remove().unwrap();

                    definitions_list.push(Definition::new(idval,shapetype,val1,val2));
                    definitions_or_operations +=1;
                    continue;
                }
                _ =>{}
            }
            if tokens_eq(&token,&Tokens::Id(Vec::<char>::new())) {
                if let Tokens::Id(value) = token {
                    let stringform: String = value.iter().collect();
                    object_fields.add(stringform).ok();
                    continue;
                }
            
            } 
            if tokens_eq(&token,&Tokens::Num(Vec::<char>::new())) {
                if let Tokens::Num(value) = token {
                    let stringform: String = value.iter().collect();
                    object_fields.add(stringform).ok();
                    continue;
                }
            }
        }
        else{
            match token{
                Tokens::LParen|Tokens::Comma|Tokens::RParen =>{
                    continue;
                }
                Tokens::Print =>{
                    object_fields.add("".to_string()).ok();
                    continue;
                }
                Tokens::Contained =>{
                    object_fields.add("contained".to_string()).ok();
                    continue;
                }
                Tokens::Intersects =>{
                    object_fields.add("intersects".to_string()).ok();
                    continue;
                }
                Tokens::Semicolon =>{
                    let func: String = object_fields.remove().unwrap();
                    let id1: String = object_fields.remove().unwrap();
                    if object_fields.size() !=0{
                        let id2: String = object_fields.remove().unwrap();
                        }
                        let id2: String = "".to_string();

                    operations_list.push(Operation::new(func,id1,id2));
                    continue;
                }
                Tokens::End =>{
                    let func: String = object_fields.remove().unwrap();
                    let id1: String = object_fields.remove().unwrap();
                    if object_fields.size() !=0{
                    let id2: String = object_fields.remove().unwrap();
                    operations_list.push(Operation::new(func,id1,id2));
                    }
                    else{
                    let id2: String = "".to_string();
                    operations_list.push(Operation::new(func,id1,id2));
                    }
                    continue;
                }
                _ =>{}
            }
            if tokens_eq(&token,&Tokens::Id(Vec::<char>::new())) {
                if let Tokens::Id(value) = token {
                    let stringform: String = value.iter().collect();
                    object_fields.add(stringform).ok();
                    continue;
                }
            } 
        }
    }

    for operation in operations_list.iter() {
        let mut output_string:String = "query(".to_string();
        let mut val1_type: String = String::new();
        let mut val2_type: String = String::new();
        let mut val3_value: String = String::new();
        let mut val4_value: String = String::new();
        let mut val1_value: String = String::new();
        let mut val2_value: String = String::new();
        output_string = output_string + &operation.function();

        for definition in definitions_list.iter(){
            if definition.id() == operation.id1() {
                val1_type = definition.shape();
                for definition2 in definitions_list.iter(){
                    if definition.value1() == definition2.id() {
                        val1_value = definition2.shape() + &definition2.value1() + &",".to_string() + &definition2.value2() + &")".to_string();
                    }
                }
                val3_value = definition.value2();
            }
        }
        if operation.id2() != "" {
            for definition in definitions_list.iter(){
                if definition.id() == operation.id2() {
                    val2_type = definition.shape();
                    for definition2 in definitions_list.iter(){
                        if definition.value1() == definition2.id() {
                            val2_value = definition2.shape() + &definition2.value1() + &",".to_string() + &definition2.value2() +&")".to_string() ;
                        }
                    }
                    val4_value = definition.value2();
                }
            }
            output_string = output_string + &val1_type + &"(".to_string()+  &val1_value + &", ".to_string() + &val3_value + &"), ".to_string() + &val2_type + &val2_value +&", ".to_string() + &val4_value + &"))).".to_string();
            println!("{}",output_string);
        }
        else{
            output_string = output_string + &val1_type +  &val1_value + &", ".to_string() + &val3_value + &")).".to_string();
            println!("{}",output_string);
        }
    }
    println!("writeln(T) :- write(T), nl.\nmain:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),
          halt.");

    extra_credit_parse_tree(program_tokens, filename);
}

fn extra_credit_parse_tree(program_tokens: Vec<Tokens>, filename: &String){ //creates parse tree with tokens list
    let mut definitions_or_operations =0;
    let mut file : String = filename.to_string();
    for n in 0..3{
        file.pop();
    }
    file = file + &".pt".to_string();
    let mut pt_file = File::create(file.as_str()).expect("Error encountered while creating file!");
    for (i, token) in program_tokens.iter().enumerate(){
        match token {
            Tokens::Definitions =>{
                pt_file.write_all(b"DEFINITIONS\n").expect("Error encountered while creating file!");
            }
            Tokens::Colon =>{
                pt_file.write_all(b"-:\n").expect("Error encountered while creating file!");
            }
            Tokens::Semicolon =>{
                pt_file.write_all(b"--;\n").expect("Error encountered while creating file!");
            }
            Tokens::Assign =>{
                pt_file.write_all(b"----=\n").expect("Error encountered while creating file!");
            }
            Tokens::Point =>{
                pt_file.write_all(b"-----Point\n").expect("Error encountered while creating file!");
            }
            Tokens::Circle =>{
                pt_file.write_all(b"-----Circle\n").expect("Error encountered while creating file!");
            }
            Tokens::Square =>{
                pt_file.write_all(b"-----Square\n").expect("Error encountered while creating file!");
            }
            Tokens::LParen =>{
                pt_file.write_all(b"------(\n").expect("Error encountered while creating file!");
            }
            Tokens::RParen =>{
                pt_file.write_all(b"------)\n").expect("Error encountered while creating file!");
            }
            Tokens::Operations =>{
                pt_file.write_all(b"OPERATIONS\n").expect("Error encountered while creating file!");
                definitions_or_operations=1;
            }
            Tokens::Print =>{
                pt_file.write_all(b"-----Print\n").expect("Error encountered while creating file!");
            }
            Tokens::Contained =>{
                pt_file.write_all(b"-----Contained\n").expect("Error encountered while creating file!");
            }
            Tokens::Intersects =>{
                pt_file.write_all(b"-----Intersects\n").expect("Error encountered while creating file!");
            }
            Tokens::Comma =>{
                pt_file.write_all(b"-------,\n").expect("Error encountered while creating file!");
            }
            Tokens::End =>{
                pt_file.write_all(b"END\n").expect("Error encountered while creating file!");
            }
            Tokens::Period =>{
                pt_file.write_all(b".\n").expect("Error encountered while creating file!");
            }
            _ =>{}
        }
        if tokens_eq(token,&Tokens::Id(Vec::<char>::new())) {
            if definitions_or_operations != 0 || &program_tokens[i-2] == &Tokens::Circle || &program_tokens[i-2]==&Tokens::Square {
                let id_num = format!("--------{:?}\n", token);
                pt_file.write_all(id_num.as_bytes()).expect("Error encountered while creating file!");
            }
            else {
                let id_num = format!("---{:?}\n", token);
                pt_file.write_all(id_num.as_bytes()).expect("Error encountered while creating file!");
            }
            
        }
        if tokens_eq(token, &Tokens::Num(Vec::<char>::new())){
            let id_num = format!("--------{:?}\n", token);
                pt_file.write_all(id_num.as_bytes()).expect("Error encountered while creating file!");
        }
    }
}

fn main() {
    let filename_and_purpose: Vec<String> = env::args().collect(); //Reads filename and -s or -p from command line argument
    let filename: &String = &filename_and_purpose[1]; //name of codefile to compile
    let scheme_or_p: &String = &filename_and_purpose[2]; // whether to output scheme or queries
    if scheme_or_p.as_str() == "-p" {println!("/*");}
    let program_tokens: Vec<Tokens> =lexical_analysis(filename.to_string()); // calls lexical analysis function to lex input file 
    syntax_analysis(&program_tokens); // Calls parser after input passes lexer with reference to program_tokens
    println!("; Lexical and Syntax analysis passed"); // required assignment print out
    if scheme_or_p.as_str() == "-s"{
        scheme_output(program_tokens,filename);
    }
    else if scheme_or_p.as_str() == "-p" {
        println!("*/");
        prolog_ouput(program_tokens,filename);
    }
    else{
        println!("Error: Output Print Command Unknown");
    }
    
}