use names::{Generator, Name};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{alphanumeric1, none_of};
use nom::character::is_alphanumeric;
use nom::combinator::opt;
use nom::IResult;
use passwords::PasswordGenerator;
use shared::prelude::*;
use tracing::{info, trace};

pub fn prune_escape(input: &str) -> IResult<&str, &str> {
    let (rem, _) = opt(tag("\""))(input)?;
    Ok((rem, ""))
}

pub fn prune_str(input: &str) -> IResult<&str, &str> {
    let (rem, _) = prune_escape(input)?;
    let (_, word) = alphanumeric1(rem)?;
    Ok((word, ""))
}

pub fn name_part(input: &str) -> IResult<&str, &str> {
    let (rem, _) = opt(tag("-"))(input)?;
    let (rem, word) = alphanumeric1(rem)?;
    let (rem, _) = prune_escape(rem)?;
    Ok((rem, word))
}

pub fn prune_name(input: &str) -> IResult<&str, String> {
    let (mut rem, _) = prune_escape(input)?;
    let mut parts = Vec::new();
    while rem.len() > 0 {
        let (next, part) = name_part(rem)?;
        parts.push(part);
        rem = next;
    }

    let mut result = "".to_string();
    let mut i = 0;
    let cap = parts.len() - 1;
    for part in parts {
        trace!("Word part is {}", part);
        result.push_str(part);
        if i < cap {
            result.push_str("-");
        }
        i += 1;
    }
    Ok((rem, result))
}

pub fn uuid_part(input: &str) -> IResult<&str, &str> {
    let (rem, _) = opt(tag("-"))(input)?;
    let (rem, part) = take_until("-")(rem)?;
    Ok((rem, part))
}

pub struct RandomUser<'a> {
    names: Generator<'a>,
    passwords: passwords::PasswordGenerator,
}

impl<'a> RandomUser<'a> {
    pub fn new() -> Self {
        let names = Generator::with_naming(Name::Numbered);
        let passwords = PasswordGenerator::new();
        Self { names, passwords }
    }

    pub fn username(&mut self) -> String {
        self.names.next().unwrap()
    }

    pub fn usernames(&mut self, count: usize) -> Vec<String> {
        let mut usernames = Vec::new();
        while usernames.len() < count {
            usernames.push(self.names.next().unwrap())
        }
        usernames
    }

    pub fn password(&self) -> String {
        self.passwords.generate_one().unwrap()
    }

    pub fn passwords(&self, count: usize) -> Vec<String> {
        self.passwords.generate(count).unwrap()
    }

    pub fn user(&mut self) -> User {
        let username = self.username();
        let password_hash = self.password();
        User::new(&username, &password_hash)
    }

    pub fn users(&mut self, count: usize) -> Vec<User> {
        let usernames = self.usernames(count);
        let passwords = self.passwords(count);
        let mut users = Vec::new();
        let mut i = 0;
        while users.len() < count {
            users.push(User::new(&usernames[i], &passwords[i]));
            i += 1;
        }
        users
    }
}
