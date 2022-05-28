#[derive(Debug)]
struct Page {
    number: u32,
}

type Frames = Vec<Page>;

fn get_page(frames: &Frames, page_number: u32) -> Option<&Page> {
    return frames.iter().find(|page| page.number == page_number);
}

fn load_page(frames: &mut Frames, page_number: u32, number_of_frames: usize) -> &Page {
    if frames.len() >= number_of_frames {
        frames.pop();
    }
    let page = Page {
        number: page_number,
    };
    frames.insert(0, page);
    return &frames[0];
}

fn use_page(_page: &Page) {}

pub fn simulate(refs: &Vec<u32>, number_of_frames: usize) -> u32 {
    let mut frames: Frames = Vec::new();
    let mut page_faults: u32 = 0;

    for reference in refs {
        let page_option = get_page(&frames, *reference);
        let page: &Page;

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
