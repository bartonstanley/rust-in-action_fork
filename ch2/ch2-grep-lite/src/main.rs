use std::fs;
use std::io;

use regex::Regex;
use clap::{Command, Arg};

const NUMBER_OF_CONTEXT_LINES: usize = 2usize;

struct ContextAndTags {
    pub context: Vec<Vec<(usize, String)>>,
    pub tags: Vec<usize>,
}

impl ContextAndTags {
    fn new() -> ContextAndTags {
        ContextAndTags { context: vec![], tags: vec![] }
    }
}

fn get_args() -> Result<(Regex, String), String> {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::new("pattern")
            .help("The pattern to search for")
            .required(true))
        .arg(Arg::new("file_name")
            .help("File to search")
            .required(true))
        .get_matches();

    let pattern = args.get_one::<String>("pattern");
    if pattern.is_none() {
        return Err(String::from("pattern argument missing"));
    }
    let regex = Regex::new(&pattern.unwrap()).unwrap();

    let file_name = args.get_one::<String>("file_name");
    if file_name.is_none() {
        return Err(String::from("file_name argument missing"));
    }

    Ok((regex, String::from(file_name.unwrap())))
}

fn get_context_and_matching_lines(file_contents: &str, regex: Regex) -> ContextAndTags {
    let mut context_and_tags = ContextAndTags::new();

    for (i, line) in file_contents.lines().enumerate() {
        if regex.find(&line).is_some() {
            context_and_tags.tags.push(i);

            let v = Vec::with_capacity(2 * NUMBER_OF_CONTEXT_LINES + 1);
            context_and_tags.context.push(v);
        }
    }

    context_and_tags
}

fn populate_context(file_contents: &str, context_and_tags: &mut ContextAndTags) {
    for (i, line) in file_contents.lines().enumerate() {
        for (j, tag) in context_and_tags.tags.iter().enumerate() {
            let lower_bound =
                tag.saturating_sub(NUMBER_OF_CONTEXT_LINES);
            let upper_bound =
                tag + NUMBER_OF_CONTEXT_LINES;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                context_and_tags.context[j].push(local_ctx);
            }
        }
    }
}

fn display_results(contexts: &Vec<Vec<(usize, String)>>) {
    for local_ctx in contexts.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

fn get_file_contents(file_name: &str) -> Result<String, String> {
    let result = if file_name == "-" {
        io::read_to_string(io::stdin())
    } else {
        fs::read_to_string(file_name)
    };

    match result {
        Ok(file) => Ok(file),
        Err(error) => Err(format!("Problem opening the file: {error:?}")),
    }
}

fn main() -> Result<(), String> {
    let (regex, file_name) = get_args()?;

    let file_contents = get_file_contents(&file_name)?;

    let mut context_and_tags = get_context_and_matching_lines(&file_contents, regex);

    if !context_and_tags.tags.is_empty() {
        populate_context(&file_contents, &mut context_and_tags);
        display_results(&context_and_tags.context);
    }
    
    Ok(())
}