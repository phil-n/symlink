use std::env;
use std::process;
use std::os::windows::fs;

fn main() {
    let args: Vec<_> = env::args().collect();

    let (src, dst) = match args.len() {
        i if i == 2 => (&args[0], &args[1]),
        i if i == 3 => (&args[1], &args[2]),
        _ => {
            println!("usage: symlink src dst");
            process::exit(1);
        }
    };

    let link = fs::symlink_dir(src, dst);
        if let Err(e) = link {
            println!("failed to symlink, {}", e);
            process::exit(2);
        }

        println!("linked {} to {}", src, dst);
        process::exit(0);
}
