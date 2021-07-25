/*
 * chamkho - a word breaker written in Rust
 * Copyright (C) 2015  Vee Satayamas
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */
#[macro_use]
extern crate clap;
extern crate rayon;
extern crate core;

mod lib;

use std::io;
use std::io::BufRead;
use clap::App;
use std::path::Path;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::*;
use std::time::Instant;

// use core::panicking::panic;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let lang = matches.value_of("lang");
    let dict_path = match matches.value_of("dict_path") {
        Some(_dict_path) => Path::new(_dict_path),
        None => {
            match lang {
                Some("lao") => lib::lao_path(),
                Some("thai") | Some(_) | None => lib::default_path()
            }            
        }
    };
    let word_delim = match matches.value_of("word_delimiter") {
        Some(word_delim) => word_delim,
        None => " | "
    };
    let dict = lib::load_dict(dict_path).unwrap();  // unwrap() returns the value in Ok(value)
    let wordcut = lib::Wordcut::new(dict); // this dict contains all separated single words, right?

    let start = Instant::now();

// -------------- Original version -------------

    for line_opt in io::BufReader::new(io::stdin()).lines() {   //read the command
        let cleaned_line = match line_opt {
            Ok(line) => if line.len() > 0 {                                // for a fancy character, len() may return more than 1
                line.trim_end_matches('\n').to_string()                     //return the line with the new line character removed
            } else {
                line
            },
            Err(e) => panic!("Cannot read line {}", e)
        };

        let segmented_string = wordcut.put_delimiters(&cleaned_line, word_delim);
        println!("{}", segmented_string);
    }

// -------------- Parallel using rayon -------------
    let mut lines: Vec<String> = vec![];        // create vector
    io::BufReader::new(io::stdin()).lines() // read command line
        .map(|line|
            lines.push(line.unwrap())); // push to vector

    let cleaned_line_p = lines
        .par_iter()
        .map(|line| line.trim_end_matches('\n').to_string());
    //     .map(|line| wordcut.put_delimiters(&line, word_delim))  // I'm stuck here, it's almost done :(
    //     .for_each(|segmented_string| println!("{}", segmented_string));
    //
    // cleaned_line_p.iter();

    println!("Time: {:.2?}", start.elapsed());



    // My garbage below
    //         return panic!("Can't read line!");
    //     })
    // .map(|line_opt_p| )

        // let cleaned_line_p = match line_opt_p
    //     if line_opt_p.len() > 0 {
    //         line_opt_p = line_opt_p.trim_end_matches('\n').to_string()
    // .map(|line_p| line_p.delimeter())


    //     if cleaned_line_p.len() > 0 {
    //         Ok(cleaned_line_p.trim_end_matches('\n').to_string())
    //     } else {
    //         Ok(cleaned_line_p)
    //     });

    // let line_opt_p:Vec<String> = io::BufReader::new(io::stdin()).lines().par_iter();
    // let clean_line_p = line_opt_p
    //     .map(|x|
    //         if x.matches(line_opt_p)) {
    //     Ok(cleaned_line_p) => if cleaned_line_p.len() > 0 {
    //         cleaned_line_p.trim_end_matches('\n').to_string()
    //     } else {
    //                 cleaned_line_p
    //     }
    // };
}
