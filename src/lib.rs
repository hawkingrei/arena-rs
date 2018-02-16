struct Arena {
    blocks_: Vec<u8>,
    alloc_bytes_remaining_: usize,
}

const kBlockSize: int = 4096;

impl Arena {
    fn new(size: usize) -> Arena {
        Arena {
            blocks_: Vec::with_capacity(kBlockSize),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
