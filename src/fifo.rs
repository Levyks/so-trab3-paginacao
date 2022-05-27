#[derive(Debug)]
struct Page {
    number: u32,
}

type Frames = Vec<Option<Page>>;

fn get_page(frames: &Frames, page_number: u32) -> Option<&Page> {
    for frame in frames {
        if let Some(page) = frame {
            if page.number == page_number {
                return Some(page);
            }
        }
    }
    return None;
}

fn load_page(frames: &mut Frames, page_number: u32) {
    frames.pop();
    frames.insert(
        0,
        Some(Page {
            number: page_number,
        }),
    );
}

pub fn simulate(refs: &Vec<u32>, number_of_frames: usize) -> u32 {
    let mut frames: Frames = (0..number_of_frames).map(|_| None).collect();
    let mut page_faults: u32 = 0;

    for reference in refs {
        let page = get_page(&frames, *reference);

        if page.is_none() {
            page_faults += 1;
            load_page(&mut frames, *reference);
        }
    }

    return page_faults;
}
