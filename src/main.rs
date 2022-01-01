use structopt::StructOpt;
use std::fs;

#[derive(StructOpt)]
#[structopt(about = "the sin tracker")]
#[derive(Debug)]
enum Sin {
    Init {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf
    }
}

fn sin_init(path: std::path::PathBuf) {
    let sin_path = path.join(".git");
    let subdirs = vec!["objects", "refs"];
    for d in subdirs.iter() {
        let result = fs::create_dir_all(sin_path.join(d));
        if result.is_err() {
            panic!("{}", result.unwrap_err());
        }
    }
    println!("Initialized empty sin repository in {}", sin_path.display());
}

fn main() {
    println!("Hello, world!");
    let args = Sin::from_args();
    println!("args: {:?}", args);
    match args {
        Sin::Init {
            path
        } => {
            sin_init(path);
        }
    }
}
