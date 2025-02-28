use anyhow::Error;
use std::collections::HashSet;
use std::fs;

#[derive(thiserror::Error, Debug)]
enum DomainListError {
    #[error("List is empty")]
    EmptyList,
    #[error("Couldn't read blacklists from README.md")]
    ReadmeError,
}

fn main() {
    let allow_lists = get_allow_lists();
    let allow_lists_union: HashSet<String> = allow_lists
        .iter()
        .flat_map(|(_, s)| s.iter())
        .cloned()
        .collect();
    let block_lists = get_block_lists();
    let block_lists_union: HashSet<String> = block_lists
        .iter()
        .flat_map(|(_, s)| s.iter())
        .cloned()
        .collect();
    println!("Allow lists: {:?}", allow_lists.len());
    println!(
        "Unique entries in allow lists: {:?}",
        allow_lists_union.len()
    );
    println!("Block lists: {:?}", block_lists.len());
    println!(
        "Unique entries in block lists: {:?}",
        block_lists_union.len()
    );

    // Check if any of the domains in the allow lists are not in any of the block lists
    for (allow_name, allow_set) in allow_lists {
        let remaining: Vec<_> = allow_set.difference(&block_lists_union).collect();

        if !remaining.is_empty() {
            println!("Domains in {allow_name} not in any block list:");
            for domain in remaining {
                println!("- {domain}");
            }
        }
    }

    // Check for block lists that are subsets of other block lists
    for (block_name, block_set) in &block_lists {
        for (other_name, other_set) in &block_lists {
            if block_name == other_name {
                continue;
            }
            if block_set.is_subset(other_set) {
                println!("{block_name} is a subset of {other_name}");
            }
        }
    }
}

fn get_allow_lists() -> Vec<(String, HashSet<String>)> {
    let mut allow_lists = Vec::new();

    match read_file_from_disk("allowlist") {
        Ok(content) => match parse_list(&content) {
            Ok(set) => {
                allow_lists.push(("allowlist".into(), set));
            }
            Err(e) => {
                eprintln!("Failed to parse allowlist: {e}");
            }
        },
        Err(e) => {
            eprintln!("Failed to read allowlist: {e}");
        }
    }

    allow_lists
}

fn get_block_lists() -> Vec<(String, HashSet<String>)> {
    let mut block_lists = Vec::new();

    let urls = match get_block_list_urls() {
        Ok(urls) => urls,
        Err(e) => {
            eprintln!("Failed to get blocklist URLs: {e}");
            return block_lists;
        }
    };

    for url in urls {
        match read_file_from_web(&url) {
            Ok(content) => match parse_list(&content) {
                Ok(set) => {
                    block_lists.push((url, set));
                }
                Err(e) => {
                    eprintln!("Failed to parse blocklist '{url}': {e}");
                }
            },
            Err(e) => {
                eprintln!("Failed to read blocklist '{url}': {e}");
            }
        }
    }

    block_lists
}

fn get_block_list_urls() -> Result<Vec<String>, Error> {
    // Read blocklist from README.md
    let readme = read_file_from_disk("README.md")?;
    let mut urls = Vec::new();

    // Check text between `## Blocklists` and `## Allowlists`
    let blocklists_section = readme
        .split_once("## Blocklists")
        .map(|(_, s)| s)
        .and_then(|s| s.split_once("## Allowlists"))
        .map(|(s, _)| s)
        .ok_or(DomainListError::ReadmeError)?;

    for line in blocklists_section.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || !line.starts_with("- ") {
            continue;
        }
        let blocklist = line
            .split_once("- ")
            .map(|(_, s)| s.to_string())
            .ok_or(DomainListError::ReadmeError)?;
        urls.push(blocklist);
    }

    if urls.is_empty() {
        return Err(DomainListError::ReadmeError.into());
    }

    Ok(urls)
}

fn parse_list(content: &str) -> Result<HashSet<String>, Error> {
    let mut set = HashSet::new();

    for line in content.lines() {
        // If a line contains a space, it's probably in the form 0.0.0.0 example.com # Comment
        // We only want the domain part
        let parts: Vec<_> = line
            .split_once('#')
            .unwrap_or((line, ""))
            .0
            .split_whitespace()
            .filter(|part| !part.is_empty())
            .collect();
        if parts.len() == 1 {
            set.insert(parts[0].into());
        } else if parts.len() == 2 {
            set.insert(parts[1].into());
        }
    }

    if set.is_empty() {
        return Err(DomainListError::EmptyList.into());
    }

    Ok(set)
}

fn read_file_from_disk(path: &str) -> Result<String, Error> {
    let content = fs::read_to_string(path)?;
    if content.trim().is_empty() {
        return Err(DomainListError::EmptyList.into());
    }
    Ok(content)
}

fn read_file_from_web(url: &str) -> Result<String, Error> {
    let content = reqwest::blocking::get(url)?.text()?;
    if content.trim().is_empty() {
        return Err(DomainListError::EmptyList.into());
    }
    Ok(content)
}
