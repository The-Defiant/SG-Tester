use std::collections::VecDeque;
use std::io::{BufWriter, Write};

pub struct WritePlanner<'a> {
    queue: VecDeque<&'a [u8]>,
}

impl<'a> WritePlanner<'a> {
    pub fn new() -> Self {
        let vec_queue: VecDeque<&'a [u8]> = VecDeque::new();
        Self { queue: vec_queue }
    }
    pub fn add(&mut self, text: &'a [u8]) -> &mut Self {
        self.queue.push_back(text);
        self
    }
    pub fn write<T: Write>(&mut self, writable: T) {
        let mut output = BufWriter::new(writable);

        let mut can_pop: bool = !self.queue.is_empty();
        while can_pop {
            can_pop = match self.queue.pop_front() {
                Some(arr) => {
                    let _ = output.write_all(arr);
                    let _ = output.write_all(b"\n");
                    true
                }
                None => false,
            };
        }
        output.flush().unwrap()
    }
}

#[test]
fn write_planner_adds_to_queue() {
    let mut write_planner = WritePlanner::new();
    let test_value = b"some_binary";
    write_planner.add(test_value);
    write_planner.add(test_value);

    for value in write_planner.queue.iter() {
        assert_eq!(*value, test_value);
    }
}
#[test]
fn write_planner_writes_to_file() {
    use std::io::{Read, Seek, SeekFrom};
    use tempfile::tempfile;

    let mut write_planner = WritePlanner::new();
    let write_planner = write_planner.add(b"Some value").add(b"Another value");

    let mut tmpfile = tempfile().unwrap();
    write_planner.write(&tmpfile);
    // reset the cursor
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    let mut contents = String::new();
    let _ = tmpfile.read_to_string(&mut contents);
    assert_eq!(contents, "Some value\nAnother value\n");
}
