#![allow(non_snake_case)]
//#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
extern crate encoding;
//use encoding::all::GBK;
//use encoding::{EncoderTrap, Encoding};

use SoraText::lib::*;
// use std::slice::Iter;
use std::env;

fn main() {
    //参数数组，检验参数是否合法
    let arg: Vec<String> = env::args().collect();
    //创建读取汉化文件,读取日文文件和输出tsv文件的标头
    let (input, inputJ, mut output) = open_file_from_argument(&arg);
    //创建双语缓冲jbuff,cbuff
    let mut jbuff: Vec<String> = Vec::new();
    let mut cbuff: Vec<String> = Vec::new();
    let mut a: i32 = 0; //行计数
    let mut sign = Signal::Standing; //状态标记
                                     //    for line in input.lines() {
    let mut line = String::new();
    let mut lineJ = String::new();
    let mut iter_input = input.lines();
    let mut iter_inputJ = inputJ.lines();
    loop {
        line = match iter_input.next() {
            Some(i) => i.to_string(),
            None => break,
        };
        lineJ = match iter_inputJ.next() {
            Some(i) => i.to_string(),
            None => break,
        };
        a += 1;
        match sign {
            Signal::Standing => println!("{},Standing", a),
            Signal::Japanese => println!("{},Japanese", a),
            Signal::Chinese => println!("{},Chinese", a),
        }
        match sign {
            Signal::Standing => match standing(&line) {
                Ok(s) => sign = s,
                Err(_) => errordeal(Errinfo::FormatError, &vec![a]),
            },
            Signal::Japanese => sign = japanese(&line, &lineJ, &mut jbuff),
            Signal::Chinese => {
                if let Ok(false) = chinese(&line, &mut cbuff) {
                    match writing(&mut jbuff.iter(), &mut cbuff.iter(), &mut output) {
                        Ok(s) => {
                            //写入成功就清空双语缓存并进入待机状态
                            sign = s;
                            jbuff.clear();
                            cbuff.clear();
                        }
                        Err(_) => errordeal(
                            Errinfo::LineError,
                            &vec![
                                jbuff.len().try_into().unwrap(),
                                cbuff.len().try_into().unwrap(),
                                a,
                            ],
                        ),
                    }
                }
            }
        }
    }
}
