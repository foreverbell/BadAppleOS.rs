use alloc::boxed::Box;
use alloc::vec::Vec;
use ba::stream::Stream;
use core::slice::from_raw_parts;

struct TrieNode {
  child: [Option<Box<TrieNode>>; 2],
  label: Option<u8>,
}

impl TrieNode {
  fn new() -> TrieNode {
    TrieNode {
      child: [None, None],
      label: None,
    }
  }

  fn insert(&mut self, data: &[u8], label: u8) {
    if data.is_empty() {
      self.label = Some(label);
    } else {
      let (h, t) = data.split_first().unwrap();
      let node = &mut self.child[*h as usize];
      if node.is_none() {
        *node = Some(Box::new(TrieNode::new()));
      }
      node.as_mut().unwrap().insert(t, label);
    }
  }
}

pub struct Decompressed {
  pub n_frames: usize,
  pub buf: Vec<u8>,
}

fn read_u32(bytes: &[u8]) -> u32 {
  let mut ret: u32 = 0;
  for x in bytes {
    ret = (ret << 8) + (*x as u32);
  }
  ret
}

pub fn decompress(ptr_from: *const u8, ptr_to: *const u8) -> Decompressed {
  let len = ptr_from.offset_to(ptr_to).unwrap() as usize;
  let mut bytes: &[u8] = unsafe { from_raw_parts(ptr_from, len) };

  let n_frames = read_u32(&bytes[0..2]) as usize;
  let s_buf = read_u32(&bytes[2..6]) as usize;
  let n_keys = bytes[6] as usize;

  printf!("[decompressor] Frame count = {}.\n", n_frames);
  printf!("[decompressor] Buffer size = {}.\n", s_buf);
  printf!("[decompressor] Key count = {}.\n", n_keys);

  let mut buf = Vec::new();
  buf.resize(s_buf, 0);

  let mut trie = TrieNode::new();

  bytes = &bytes[7..];

  for _ in 0..n_keys {
    let len = bytes[1] as usize;
    let mut data = Vec::new();
    let mut reader = Stream::new(&bytes[2..2 + (len + 7) / 8]);

    data.resize(len, 0);
    for i in 0..len {
      data[i] = reader.next_byte();
    }
    trie.insert(data.as_slice(), bytes[0]);

    bytes = &bytes[2 + (len + 7) / 8..];
  }

  let mut node: &TrieNode = &trie;
  let mut reader = Stream::new(bytes);
  for i in 0..buf.len() {
    loop {
      let bit = reader.next_byte() as usize;
      node = node.child[bit].as_ref().unwrap();
      if node.label.is_some() {
        buf[i] = node.label.unwrap();
        node = &trie;
        break;
      }
    }
  }

  printf!("[decompressor] Remaining {} bits.\n", reader.remain());

  Decompressed { n_frames, buf }
}
