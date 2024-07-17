use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static SFZ_FILE_HEADER: &str =
    "<global>\n\
     loop_mode=loop_sustain\n\
     // tune=-0\n\n";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
	print_usage();
	std::process::exit(1);
    }

    let sfz_file_name = &args[1];
    let sfz_file_path = Path::new(sfz_file_name);
    let sfz_file_path_display = sfz_file_path.display();
    let mut sfz_file = match File::create(&sfz_file_path) {
	Err(why) => panic!("Couldn't create file {}: {}",
			   sfz_file_path_display, why),
	Ok(file) => file,
    };

    let mut sfz_file_contents: String = String::from("");

    sfz_file_contents.push_str(SFZ_FILE_HEADER);

    let key_start = 36;
    let num_samples = args.len() - 2;
    let key_end = key_start + num_samples;
    let group_line_string: String = format!("<group> lokey={} hikey={}\n",
				     key_start, key_end);
    sfz_file_contents.push_str(&group_line_string);

    let mut key_idx = key_start;
    for sample_path in &args[2..] {
	let sample_line_string = format!("  <region> key={} sample={}\n",
					 key_idx, sample_path);
	sfz_file_contents.push_str(&sample_line_string);
	key_idx += 1;
    }

    match sfz_file.write_all(&sfz_file_contents.as_bytes()) {
	Err(why) => panic!("Couldn't write to file {}: {}",
			   sfz_file_path_display, why),
	Ok(_) => println!("Successfully wrote to file {}",
			  sfz_file_path_display),
    };

    std::process::exit(0);
}

fn print_usage() {
    println!("Usage: sfzgen SFZ_FILE_NAME [SAMPLE_PATHS]...")
}

