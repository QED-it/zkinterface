use std::slice;

fn bytes_to_words(buf: &[u8]) -> &[u64] {
    unsafe {
        slice::from_raw_parts(buf.as_ptr() as *const u64,
                              buf.len() / 8)
    }
}


#[derive(Debug)]
struct Segment {
    extra_segments: u32,
    first_segment_words: u32,
}

fn parse_segments_table(w: u64) -> Segment {
    Segment {
        extra_segments: w as u32,
        first_segment_words: (w >> 32) as u32,
    }
}

#[derive(Debug)]
enum Pointer {
    Null {},
    Struct {
        offset: i32,
        data_words: i32,
        pointer_words: i32,
    },
    List {
        offset: i32,
        element_size: u8,
        element_count: i32,
    },
    FarPointer {},
}

fn parse_pointer(w: u64) -> Pointer {
    if w == 0 {
        return Pointer::Null {};
    }
    let ptr_type = w & (1 | 2); // A = 2 bits.
    match ptr_type {
        0 => Pointer::Struct {
            offset: ((w >> 2) & 0x3FFFFFFF) as i32, // B = 30 bits.
            data_words: ((w >> 32) & 0xFFFF) as i32, // C = 16 bits,
            pointer_words: (w >> 48) as i32, // D = last 16 bits.
        },
        1 => {
            Pointer::List {
                offset: ((w >> 2) & 0x3FFFFFFF) as i32, // B = 30 bits.
                element_size: ((w >> 32) & (1 | 2 | 4)) as u8, // C = 3 bits.
                element_count: (w >> 35) as i32, // D = last 29 bits.
            }
        }
        2 => Pointer::FarPointer {},
        _ => panic!("Illegal pointer type 3"),
    }
}

#[derive(Debug)]
struct ListOfStruct {
    struct_count: i32,
    data_words: i32,
    pointer_words: i32,
}

fn parse_list_tag(w: u64) -> ListOfStruct {
    let tag = parse_pointer(w);
    match tag {
        Pointer::Struct { offset, data_words, pointer_words } => {
            ListOfStruct { struct_count: offset, data_words, pointer_words }
        }
        _ => { panic!("Invalid tag!"); }
    }
}

fn traverse(segment: &[u64], pos: i32, depth: usize) {
    let indent = "\t".repeat(depth);
    let ptr = parse_pointer(segment[pos as usize]);
    println!("{}traverse {}: {:?}", indent, pos, ptr);
    match ptr {
        Pointer::Null {} => {}
        Pointer::Struct { offset, data_words, pointer_words } => {
            let start = pos + 1 + offset;
            for p in start..start + data_words {
                println!("{}\tField {}", indent, segment[p as usize]);
            }
            for p in start + data_words..start + data_words + pointer_words {
                traverse(segment, p, depth + 1);
            }
        }
        Pointer::List { offset, element_size, element_count } => {
            let list_start = pos + 1 + offset;
            match element_size {
                7 => {
                    let tag = parse_list_tag(segment[list_start as usize]);
                    let ListOfStruct { struct_count, data_words, pointer_words } = tag;
                    let struct_words = data_words + pointer_words;
                    for s in 0..tag.struct_count {
                        println!("{}\t- {:?}", indent, tag);
                        let start = list_start + 1 + s * struct_words;
                        for p in start..start + data_words {
                            println!("{}\t\tField {}", indent, segment[p as usize]);
                        }
                        for p in start + data_words..start + data_words + pointer_words {
                            traverse(segment, p, depth + 2);
                        }
                    }
                }
                _ => {}
            }
        }
        Pointer::FarPointer {} => {}
    }
}

fn parse_message(buf: &[u8]) {
    let words = bytes_to_words(buf);
    let segments_table = parse_segments_table(words[0]);
    let segment = &words[1..];
    let root = parse_pointer(segment[0]);

    println!("{} words", words.len());
    println!("words {:?}", words);
    println!("{:?}", segments_table);
    println!("root {:?}", root);

    println!("\nTraversing…");
    traverse(segment, 0, 0);
}

#[test]
fn test_adhoc_parse() {
    let mut buf = vec![];
    request_assignments(&mut buf).unwrap();

    handle_assignments(&buf[..]).unwrap();

    println!("");
    parse_message(&buf);
    println!("");
}
