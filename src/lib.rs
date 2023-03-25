#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
pub mod lib {
    extern crate encoding;
    use encoding::all::GBK;
    use encoding::{EncoderTrap, Encoding};
    use std::fs;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::process::exit;
    use std::slice::Iter;
    pub enum Signal {
        //用于处理状态的枚举
        Standing,
        Japanese,
        Chinese,
    }
    pub enum Errinfo {
        //用于错误状态的枚举
        ArgumentWrong,
        FileOpen,
        FileCreate,
        FormatError,
        LineError,
    }

    pub fn open_file_from_argument(
        arg: &Vec<String>,
    ) -> (String, String, BufWriter<File>, BufWriter<File>) {
        if arg.len() != 3 {
            errordeal(Errinfo::ArgumentWrong, &Vec::<i32>::new());
        }
        //从参数数组提取读取汉化文件的名称并检验是否打开成功
        let input_file_name = &arg[1];
        let input = match fs::read_to_string(input_file_name) {
            Ok(f) => f,
            Err(_) => errordeal(Errinfo::FileOpen, &vec![input_file_name]),
        };
        //从参数数组提取读取日文文件的名称并检验是否打开成功
        let japanese_file_name = &arg[2];
        let inputJ = match fs::read_to_string(japanese_file_name) {
            Ok(f) => f,
            Err(_) => errordeal(Errinfo::FileOpen, &vec![japanese_file_name]),
        };
        //创建写入文件
        let mut output_file_name = input_file_name.clone();
        output_file_name.push_str(".tsv");
        let output_file = match File::create(output_file_name.clone()) {
            Ok(f) => f,
            Err(_) => errordeal(Errinfo::FileCreate, &vec![output_file_name]),
        };
        let output = BufWriter::new(output_file);
        //创建信息文件
        let mut info_file_name = input_file_name.clone();
        info_file_name.push_str("_info.txt");
        let info_file = match File::create(info_file_name.clone()) {
            Ok(f) => f,
            Err(_) => errordeal(Errinfo::FileCreate, &vec![info_file_name]),
        };
        let mut info = BufWriter::new(info_file);
        //写入抬头，方便查阅
        info.write(b"JPN\t\tCHN\t\tLine\n");
        info.flush();
        return (input, inputJ, output, info);
    }
    //错误处理函数，根据Errinfo判断错误类型，并输出相应信息
    pub fn errordeal<T: std::fmt::Display>(info: Errinfo, arg: &Vec<T>) -> ! {
        match info {
            Errinfo::ArgumentWrong => {
                println!("Error:Wrong argument.");
            }
            Errinfo::FileOpen => {
                println!("Error:Failed to open file:{}", arg[0]);
            }
            Errinfo::FileCreate => {
                println!("Error:Failed to create file:{}", arg[0]);
            }
            Errinfo::FormatError => {
                println!("Error:Unformated text.Line:{}", arg[0]);
            }
            Errinfo::LineError => {
                println!(
                    "Error:Unmatch lines.\n
                            Size of Japanese buffer:{}\n
                            Size of Chinese buffer:{}
                            Line:{}",
                    arg[0], arg[1], arg[2]
                );
            }
        }
        exit(0);
    }
    pub fn standing(line: &str) -> Result<Signal, String> {
        //待机状态
        println!("{}", line);
        if line.contains("】") {
            //检测到】就转换到日语状态
            Ok(Signal::Japanese)
        } else if line.contains("-------------") {
            //检测到减号行就继续待机
            Ok(Signal::Standing)
        } else {
            //其他状态报错
            Err(String::from("error:unformated text.Line:"))
        }
    }

    pub fn japanese(line: &str, lineJ: &str, jbuff: &mut Vec<String>) -> Signal {
        //日语处理状态
        println!("{}", line);
        if line.contains("============") {
            //检测到等号行就转换到中文状态
            return Signal::Chinese;
        } else {
            jbuff.push(lineJ.to_string());
            return Signal::Japanese;
        }
    }
    pub fn chinese(line: &str, cbuff: &mut Vec<String>) -> Result<bool, String> {
        println!("{}", line);
        if line.contains("-------------") {
            //检测到减号行就开始写入
            Ok(false)
        } else {
            cbuff.push(line.to_string());
            Ok(true)
        }
    }

    pub fn writing(
        jbuff: &mut Iter<String>,
        cbuff: &mut Iter<String>,
        input: &mut BufWriter<File>,
    ) -> Result<Signal, String> {
        let mut buff = Vec::new(); //用于转码的缓冲区
        let mut jline = String::new(); //当前写入的日文行
        let mut cline = String::new(); //当前写入的译文行
        loop {
            jline = match jbuff.next() {
                Some(s) => s.to_string(),
                None => break,
            };
            cline = match cbuff.next() {
                Some(s) => s.to_string(),
                None => break,
            };
            //提示信息
            println!("writing:\n{}\t\t{}", jline, cline);
            input.write(b"on\t");
            GBK.encode_to(&jline, EncoderTrap::Ignore, &mut buff);
            input.write_all(&buff);
            input.write(b"\t");
            buff.clear();
            GBK.encode_to(&cline, EncoderTrap::Ignore, &mut buff);
            input.write_all(&buff);
            input.write(b"\t\n");
            buff.clear();
            input.flush().unwrap();
        }

        Ok(Signal::Standing) //写入完毕后切换为待机状态
    }
    //将行数信息写入译文文件
    pub fn printinfo<T: std::fmt::Display>(arg: &Vec<T>, info: &mut BufWriter<File>) {
        info.write(&arg[0].to_string().into_bytes());
        info.write(b"\t\t");
        info.write(&arg[1].to_string().into_bytes());
        info.write(b"\t\t");
        info.write(&arg[2].to_string().into_bytes());
        info.write(b"\n");
        info.flush();
    }
}
