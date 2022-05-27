#[derive(Debug)]
struct Page {
    number: u32,
}

type Frames = Vec<Page>;

fn get_page(frames: &Frames, page_number: u32) -> Option<&Page> {
    return frames.iter().find(|page| page.number == page_number);
}

fn load_page(frames: &mut Frames, page_number: u32, number_of_frames: usize) {
    if frames.len() >= number_of_frames {
        frames.pop();
    }
    frames.insert(
        0,
        Page {
            number: page_number,
        },
    );
}

pub fn simulate(refs: &Vec<u32>, number_of_frames: usize) -> u32 {
    let mut frames: Frames = Vec::new();
    let mut page_faults: u32 = 0;

    for reference in refs {
        let page = get_page(&frames, *reference);

        if page.is_none() {
            page_faults += 1;
            load_page(&mut frames, *reference, number_of_frames);
        }
    }

    return page_faults;
}
