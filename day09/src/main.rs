use std::{borrow::BorrowMut, cmp, collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone, Copy)]
struct Span {
    length: usize,
    word: Option<usize>,
}

#[derive(Debug, Clone, )]
struct DiskMap {
    spans: Vec<Span>,
}

impl DiskMap {
    pub fn new(diskmap_string: &str) -> Self {
        let mut disk_map = DiskMap { spans: vec![] };
        let mut next_is_file = true;
        let mut next_file_id = 0;
        for c in diskmap_string.chars() {
            let length = c.to_string().parse().unwrap();
            if next_is_file {
                disk_map.spans.push(Span {length, word: Some(next_file_id), } );
                next_file_id += 1;
            } else {
                disk_map.spans.push(Span {length, word: None, } );
            }
            next_is_file = !next_is_file;
        }
        disk_map
    }

/*

    fn fully_fragmented(&self) -> bool {
        false
    }

    fn fragment(&mut self) {
        while !self.fully_fragmented() {
            let should_pop;
            let n;
            {
                let mut last_span = self.spans.last_mut().unwrap();
                match last_span.word {
                    Some(word) => {
                        n = word;
                        match last_span.length {
                            0 => panic!(),
                            1 =>  should_pop = true,
                            _ => {
                                last_span.length -= 1;
                                should_pop = false;
                            },
                        };
                    },
                    None => continue
                }
            }
            self.insert_first_open_spot(n);
            if should_pop {
                self.spans.pop();
            }
        }
    }

*/
    /*
    fn insert_first_open_spot(&mut self, word: usize) {
        // Assume we have an open spot for our word
        if !self.words.contains(&None) {
            panic!();
        }

        let first_open_spot = self.words.iter().position(|w| w==&None).unwrap();
        self.words[first_open_spot] = Some(word);
        
    }
    */

    fn find_lowest_open_span_index(&self, span_width: usize) -> Option<usize> {
        for (index, span) in self.spans.iter().enumerate() {
            match span.word {
                Some(_) => continue,
                None => {
                    if span.length >= span_width {
                        return Some(index);
                    }
                }
            }
        }
        None
    }

    fn place_span_in_open_index(&mut self, span: &Span, index: usize) {
        if !(self.spans[index].word == None || self.spans[index].length < span.length) {
            panic!();
        }
        self.spans.insert(index, *span);
        self.spans[index+1].length -= span.length;
        if self.spans[index+1].length == 0 {
            self.spans.remove(index+1);
        }
    }
    

    fn condense(&mut self) {
        let maximum_word = self.spans.iter().map(|s| s.word.unwrap_or(0)).max().unwrap();
        for i_word in (0..=maximum_word).rev() {  // Changed to inclusive range with =
            println!("Condensing {i_word}");
            // First find the span and its index
            let (span_index, span) = self.spans.iter()
                .enumerate()
                .find(|(_, s)| s.word == Some(i_word))
                .unwrap();
            let span = span.clone();  // Clone the span separately

            if let Some(index) = self.find_lowest_open_span_index(span.length) {
                if (index < span_index) {
                    self.spans[span_index] = Span { word: None, length: span.length };
                    self.place_span_in_open_index(&span, index);
                }
            }
            
            /*
            for span in &self.spans {
                println!("----{span:?}");
            }
            */
        }
    }

    fn get_check_sum(&self) -> usize {
        let mut checksum: usize = 0;
        let mut index = 0;
        for span in self.spans.iter() {
            for _ in 0..span.length {
                match span.word {
                    None => (),
                    Some(w) => {
                        checksum += w*index;
                    }
                }
                index += 1;
            }
        }
        checksum
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    // Read each line from input.txt
    let line = read_lines("input.txt");
    let mut diskmap1 = DiskMap::new(&line[0]);
    let mut diskmap2 = diskmap1.clone();

/*

    // println!("Before: {diskmap:?}");
    diskmap1.fragment();
    // println!("After: {diskmap:?}");
    let part1 = diskmap1.get_check_sum();
    println!("{part1}");

*/
    diskmap2.condense();
    let part2 = diskmap2.get_check_sum();
    println!("{part2}");

}
