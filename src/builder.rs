// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

pub trait MkArg<'a> {
    fn mk(&self, option: &'a str, args: &mut Vec<&'a str>) -> Option<String>;
}

impl<'a> MkArg<'a> for bool {
    fn mk(&self, option: &'a str, args: &mut Vec<&'a str>) -> Option<String> {
        if *self {
            args.push(option);
        }
        None
    }
}

impl<'a, T: std::string::ToString> MkArg<'a> for Option<T> {
    fn mk(&self, _option: &'a str, _args: &mut Vec<&'a str>) -> Option<String> {
        if let Some(x) = self {
            Some(x.to_string())
        } else {
            None
        }
    }
}

impl<'a> MkArg<'a> for u32 {
    fn mk(&self, _option: &'a str, _args: &mut Vec<&'a str>) -> Option<String> {
        Some(self.to_string())
    }
}

impl<'a> MkArg<'a> for &'a [&'a str] {
    fn mk(&self, option: &'a str, args: &mut Vec<&'a str>) -> Option<String> {
        for v in *self {
            if !v.is_empty() {
                args.push(option);
                args.push(&v);
            }
        }
        None
    }
}

impl<'a> MkArg<'a> for &'a str {
    fn mk(&self, option: &'a str, args: &mut Vec<&'a str>) -> Option<String> {
        if !self.is_empty() {
            args.push(option);
            args.push(self);
        }
        None
    }
}

macro_rules! fn_array {
    ($N: expr) => {
        impl<'a> MkArg<'a> for &'a [&'a str; $N] {
            fn mk(&self, option: &'a str, args: &mut Vec<&'a str>) -> Option<String> {
                for v in *self {
                    if !v.is_empty() {
                        args.push(option);
                        args.push(&v);
                    }
                }
                None
            }
        }
    };
}

// In waiting for generics with const values
fn_array!(1);
fn_array!(2);
fn_array!(3);
fn_array!(4);
fn_array!(5);
fn_array!(6);
fn_array!(7);
fn_array!(8);
fn_array!(9);
fn_array!(10);
fn_array!(11);
fn_array!(12);
fn_array!(13);
fn_array!(14);
fn_array!(15);
fn_array!(16);

#[macro_export]
macro_rules! debug_vec {
    ($v: expr) => {{
        let tmp: Vec<char> = $v.iter().map(|x| *x as char).collect();
        println!("DEBUG {}: {:?}", stringify!($v), tmp);
    }};
}

#[macro_export]
macro_rules! hg {
    ($client: expr, $com: ident $(, $a: ident = $v: expr)* ) => {{
        #![allow(clippy::needless_update)]
        $client.$com($com::Arg {
            $(
                $a: $v,
            )*
            ..Default::default()
        })
      }}
}

#[macro_export]
macro_rules! HG {
    ($client: ident $(, $a: ident = $v: expr)* ) => {{
        #![allow(clippy::needless_update)]
        $client::Arg {
            $(
                $a: $v,
            )*
            ..Default::default()
        }.run(&mut Basic { })
      }}
}

#[macro_export]
macro_rules! runcommand {
    ( $client: expr, $name: expr, $args: expr $(, $o: expr, $x: expr )* ) => {{
        let mut tmp = Vec::new();
        tmp.push($name);
        $(
            let v = if let Some(s) = $x.mk($o, &mut tmp) {
                s
            } else {
                String::new()
            };
            if !v.is_empty() {
                tmp.push($o);
                tmp.push(&v);
            }
        )*
        if !$args.is_empty() {
            tmp.push("--");
            for arg in $args {
                if !arg.is_empty() {
                    tmp.push(arg);
                }
            }
        }
        $client.runcommand(&tmp, None)
    }};
}

#[macro_export]
macro_rules! runcommand_with_prompt {
    ( $client: expr, $name: expr, $prompt: expr, $args: expr $(, $o: expr, $x: expr )* ) => {{
        let mut tmp = Vec::new();
        tmp.push($name);
        $(
            let v = if let Some(s) = $x.mk($o, &mut tmp) {
                s
            } else {
                String::new()
            };
            if !v.is_empty() {
                tmp.push($o);
                tmp.push(&v);
            }
        )*
        if !$args.is_empty() {
            for arg in $args {
                if !arg.is_empty() {
                    tmp.push(arg);
                }
            }
        }
        $client.runcommand(&tmp, Some(Box::new($prompt)))
    }};
}
