//
// main.rs
//
// Copyright (C) 2017 pigfromchina <fedora-opensuse@outlook.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

extern crate regex;
extern crate uuid;
mod utils;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use regex::Match;

use utils::genguid::genguid::getguid;
use utils::winpaste;

fn main() {
    match File::open("Database.xml") {
        Ok(mut f) => { 
            let mut buf = String::new();
            let f = f.read_to_string(&mut  buf)
                .expect("[e] 无法将dbxml读取到字符串...");
            freshdbxml(&mut buf);
        },
        Err(e) => {
            println!("[-] 在当前目录未找到数据库文件，尝试写入剪贴板...");
            //然而
            println!("[-] 剪贴板功能未实现，将写入标准输出...");
            println!("{}",getguid());
        },
    }
}
fn freshdbxml(mut dbxml:&mut String) -> () {
    println!("{}",dbxml);
}
