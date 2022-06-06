#[derive(Debug)]
struct Page {
    number: u32,
    counter: u8,
}

const COUNTER_SIZE: u32 = 8;

type Frames = Vec<Page>;

fn get_page(frames: &mut Frames, page_number: u32) -> Option<&mut Page> {
    for page in frames {
        if page.number == page_number {
            return Some(page);
        }
    }
    None
}

fn get_idx_smallest_counter(frames: &mut Frames) -> usize {
    let mut smallest_counter = frames[0].counter;
    let mut smallest_idx = 0;
    for (idx, page) in frames.iter().enumerate() {
        if page.counter < smallest_counter {
            smallest_counter = page.counter;
            smallest_idx = idx;
        }
    }
    return smallest_idx;
}

fn load_page(frames: &mut Frames, page_number: u32, number_of_frames: usize) -> &mut Page {
    let page = Page {
        number: page_number,
        counter: 0,
    };

    if frames.len() >= number_of_frames {
        let idx_smallest_counter = get_idx_smallest_counter(frames);
        frames[idx_smallest_counter] = page;
        return frames.get_mut(idx_smallest_counter).unwrap();
    } else {
        frames.insert(0, page);
        return frames.first_mut().unwrap();
    };
}

fn age_pages(frames: &mut Frames) {
    for page in frames {
        // Shifts 1 bit in all counters to the right
        page.counter = page.counter >> 1;
    }
}

fn use_page(page: &mut Page) {
    // Sets MSB to 1
    page.counter += u8::pow(2, COUNTER_SIZE - 1);
}

pub fn simulate(refs: &Vec<u32>, number_of_frames: usize) -> u32 {
    let mut frames: Frames = Vec::new();
    let mut page_faults: u32 = 0;

    for reference in refs {
        age_pages(&mut frames);
        let page_option = get_page(&mut frames, *reference);
        let page: &mut Page;

        if page_option.is_none() {
            page_faults += 1;
            page = load_page(&mut frames, *reference, number_of_frames);
        } else {
            page = page_option.unwrap();
        }

        use_page(page);
    }

    return page_faults;
}
