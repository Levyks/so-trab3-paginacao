use rand::{distributions::Uniform, Rng};
use std::env;
use std::fs::File;
use std::io::Write;

mod aging;
mod fifo;

fn create_references(number_of_references: usize, number_of_pages: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    // Range for the random number generator
    let range = Uniform::<u32>::new(0, number_of_pages);

    return (0..number_of_references)
        .map(|_| rng.sample(&range))
        .collect();
}

fn write_refs_to_file(path: String, refs: &Vec<u32>, number_of_pages: u32) {
    let mut file = File::create(&path).unwrap();

    let header = format!("{} {}\n", refs.len(), number_of_pages);
    file.write_all(header.as_bytes()).unwrap();

    for reference in refs {
        let page = format!("{}\n", reference);
        file.write_all(page.as_bytes()).unwrap();
    }

    println!("References written to {}", path);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!(
            "Usage: {} <number_of_frames> <number_of_pages> <number_of_references> <path_to_save_refs_file>\n",
            args[0]
        );
        return;
    }

    let number_of_frames: usize = args[1]
        .parse()
        .expect("Invalid number of frames (1st argument)");
    let number_of_pages: u32 = args[2]
        .parse()
        .expect("Invalid number of pages (2nd argument)");
    let number_of_references: usize = args[3]
        .parse()
        .expect("Invalid number of references (3rd argument)");
    let path_refs_file = (&args[4]).to_string();

    let refs = create_references(number_of_references, number_of_pages);

    write_refs_to_file(path_refs_file, &refs, number_of_pages);

    let fifo_page_faults = fifo::simulate(&refs, number_of_frames);
    let aging_page_faults = aging::simulate(&refs, number_of_frames);

    println!("FIFO page faults: {}", fifo_page_faults);
    println!("Aging page faults: {}", aging_page_faults);
}
