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
    //创建读取汉化文件,读取日文文件和输出tsv文件,差异行记录文件info的标头
    let (input, inputJ, mut output, mut info) = open_file_from_argument(&arg);
    //创建双语缓冲jbuff,cbuff
    let mut jbuff: Vec<String> = Vec::new();
    let mut cbuff: Vec<String> = Vec::new();
    let mut a: i32 = 0; //行计数
    let mut sign = Signal::Standing; //状态标记
    let mut line = String::new(); //汉化的处理行
    let mut lineJ = String::new(); //日文文件的处理行，根据分割线和原文对齐
    let mut iter_input = input.lines(); //汉化的迭代器
    let mut iter_inputJ = inputJ.lines(); //日文文件的迭代器
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
        //输出提示信息
        match sign {
            Signal::Standing => println!("{},Standing", a),
            Signal::Japanese => println!("{},Japanese", a),
            Signal::Chinese => println!("{},Chinese", a),
        }
        //根据sign判断状态
        match sign {
            Signal::Standing => match standing(&line) {
                Ok(s) => sign = s,
                Err(_) => errordeal(Errinfo::FormatError, &vec![a]),
            },

            Signal::Japanese => sign = japanese(&line, &lineJ, &mut jbuff),

            Signal::Chinese => {
                //如果返回false则进入writing步骤
                if let Ok(false) = chinese(&line, &mut cbuff) {
                    //如果日语行比译文行多，则认为译文合并了行，将行数信息输出到info，并补全译文的行数
                    if jbuff.len() > cbuff.len() {
                        printinfo(
                            &vec![
                                jbuff.len().try_into().unwrap(),
                                cbuff.len().try_into().unwrap(),
                                a,
                            ],
                            &mut info,
                        );
                        while jbuff.len() != cbuff.len() {
                            cbuff.push(String::from(""));
                            iter_inputJ.next(); //补全时要让日文文件的迭代器前进，以保证与译文文件的对齐
                        }
                    } else if jbuff.len() < cbuff.len() {
                        //如果日语行比译文行多，则认为译文拆分了行，进入错误处理步骤，需视情况自行修改译文文件
                        errordeal(
                            Errinfo::LineError,
                            &vec![
                                jbuff.len().try_into().unwrap(),
                                cbuff.len().try_into().unwrap(),
                                a,
                            ],
                        )
                    }
                    if let Ok(s) = writing(&mut jbuff.iter(), &mut cbuff.iter(), &mut output) {
                        //写入成功就清空双语缓存并进入待机状态
                        sign = s;
                        jbuff.clear();
                        cbuff.clear();
                    }
                }
            }
        }
    }
}
