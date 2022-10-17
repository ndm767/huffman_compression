#[derive(Clone, Debug)]
pub struct BinaryNode {
    symbol: Option<u8>,
    prob: f32,
    left: Option<Box<BinaryNode>>,
    right: Option<Box<BinaryNode>>,
}

impl BinaryNode {
    pub fn new(
        prob: f32,
        symbol: Option<u8>,
        left: Option<Box<BinaryNode>>,
        right: Option<Box<BinaryNode>>,
    ) -> BinaryNode {
        BinaryNode {
            symbol,
            prob,
            left,
            right,
        }
    }

    pub fn get_prob(&self) -> f32 {
        self.prob
    }

    pub fn get_huffman_code(&self) -> Vec<(u8, String)> {
        let mut ret: Vec<(u8, String)> = vec![];
        let has_children = self.right.is_some() && self.left.is_some();
        if has_children {
            let mut right_ret = self.right.as_ref().unwrap().get_huffman_code();
            for r in right_ret.iter_mut() {
                r.1 = format!("1{}", r.1);
            }
            ret.append(&mut right_ret);

            let mut left_ret = self.left.as_ref().unwrap().get_huffman_code();
            for l in left_ret.iter_mut() {
                l.1 = format!("0{}", l.1);
            }
            ret.append(&mut left_ret);
        } else {
            match &self.symbol {
                Some(c) => ret.push((*c, String::from(""))),
                None => panic!("Node has no children or symbol!"),
            }
        }

        return ret;
    }

    pub fn get_comp_size(&self, alphabet: Vec<(u8, f32)>, orig_len: usize) -> f32 {
        let mut lookup: Vec<f32> = Vec::new();
        for a in alphabet {
            while a.0 as usize >= lookup.len() {
                lookup.push(0.0);
            }
            lookup[a.0 as usize] = a.1 * (orig_len as f32);
        }

        let code = self.get_huffman_code();
        let mut out_size: f32 = 0.0;
        for c in code {
            out_size += lookup[c.0 as usize] * c.1.len() as f32;
        }

        out_size /= 8.0;
        return out_size;
    }
}
