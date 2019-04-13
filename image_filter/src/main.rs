extern crate image;

use std::env;
use std::process::{exit, Command};
use std::fs;
use std::path::Path;
use image::GenericImageView;
use std::fs::DirEntry;


fn main() {
    println!(" ############## image filter ################");
    let help_doc = "\tImageFilter.exe width height path new_dir_name\n\t\
ImageFilter.exe auto path 自动处理
    ";

    let mut arg_buffer = vec![];
    for i in env::args() {
        arg_buffer.push(i);
    }

    match arg_buffer.len() {
        3 => {
            if arg_buffer[1] == "auto" {
                auto()
            }
        }
        4 => {
            normal()
        }
        _ => {
            println!("{}", help_doc);
            exit(0);
        }
    }
}

fn check_or_init_dir(p: &str) {
    fs::create_dir(p.to_string() + "/GoodSize");
    fs::create_dir(p.to_string() + "/NormalSize");
    fs::create_dir(p.to_string() + "/Vertical");
}


fn auto() {
    let args: Vec<_> = env::args().collect();
    let target_path = &args[2];
    check_or_init_dir(target_path);
    println!("[*] 文件夹就绪");

    let target_size = TargetSize { large: (1800, 800), normal: (1500, 800) };
    let to_dir = ToDir { large: "GoodSize", normal: "NormalSize", vertical: "Vertical" };
    filter_image(target_size, target_path, to_dir);
}

fn normal() {}


struct TargetSize {
    large: (u32, u32),
    normal: (u32, u32),
}

struct ToDir<'a> {
    large: &'a str,
    normal: &'a str,
    vertical: &'a str,
}

fn filter_image(target_size: TargetSize, target_path: &str, to_dir: ToDir) {
    let paths = fs::read_dir(target_path).unwrap();

    for i in paths {  // 遍历指定文件夹
        match i {
            Ok(i) => {
                if Path::new(&i.path()).is_dir() { // 跳过文件夹
                    println!("[*] 文件夹, 跳过: {:?}", &i.path());
                    continue;
                }

                let image_buffer = image::open(i.path());
                match image_buffer {
                    Ok(image_buffer) => {
                        let dimension = image_buffer.dimensions();
//                        println!("[*] w, h: {:?}", dimension);
                        if dimension.0 >= target_size.large.0 && dimension.1 >= target_size.large.1 {
                            // 符合 good size
                            if dimension.0 < dimension.1 {  // 符合竖图
                                move_img(i, target_path, to_dir.vertical);
                            } else {
                                // 不符合竖图
                                move_img(i, target_path, to_dir.large);
                            }
                        } else if dimension.0 >= target_size.normal.0 && dimension.1 >= target_size.normal.1 {
                            // 不符合good size 但符合normal size
                            if dimension.0 < dimension.1 { // 竖图
                                move_img(i, target_path, to_dir.vertical);
                            } else { // 不符合竖图
                                move_img(i, target_path, to_dir.normal);
                            }
                        } else {
                            // 都不符合
                        }
                    }
                    Err(_e) => {
                        println!("[x] 图片有问题打不开 : {:?}", i.file_name())
                    }
                }
            }
            Err(_e) => {}
        }
    }
}

fn move_img(i: DirEntry, target_path: &str, to_dir: &str) {
    // ###################################################
    let f = format!("{}/{}", target_path, to_dir);
    let c = format!("mv {:?} {:?}", i.path(), f);  // 先暂时这么写，我也不清楚该咋么办
//    println!("check cmd: {}", c);
    if cfg!(target_os = "windows") {
        Command::new("powershell").args(&["/C", &c])
            .output()
            .expect("failed to execute process!")
    } else {
        println!("暂时未实现");
        // 有空在linux上改
        Command::new("sh").arg("-c")
            .output()
            .expect("failed to execute process!")
    };
    // ###################################################
}
