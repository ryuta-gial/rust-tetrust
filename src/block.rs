use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng,
    Rng,
};
use block_kind::{I, O, S, Z, J, L, T};

pub type BlockColor = usize;

// block_kindモジュールを定義しています。
// このモジュールには、ブロックの色（BlockColor）に関する定数が含まれています。
//  それぞれの定数は、特定のブロックの種類に対応しており、
// NONEは何もない、WALLは壁、GHOSTはゴーストブロック、
// IはI字型ブロック、OはO字型ブロック、SはS字型ブロック、ZはZ字型ブロック、JはJ字型ブロック、LはL字型ブロック、TはT字型ブロックを表しています。

pub mod block_kind {
    // superキーワードは、親モジュール（ここではblockモジュール）を指します。pubキーワードは、他のモジュールからアクセス可能であることを示しています。
    // superを使用してBlockColor型を参照する必要があります。
    pub const NONE:  super::BlockColor = 0;
    pub const WALL:  super::BlockColor = 1;
    pub const GHOST: super::BlockColor = 2;
    pub const I:     super::BlockColor = 3;
    pub const O:     super::BlockColor = 4;
    pub const S:     super::BlockColor = 5;
    pub const Z:     super::BlockColor = 6;
    pub const J:     super::BlockColor = 7;
    pub const L:     super::BlockColor = 8;
    pub const T:     super::BlockColor = 9;
}

// 表示ブロックの色/文字定義
pub const COLOR_TABLE: [&str; 10] = [
    "\x1b[48;2;000;000;000m  ",  // 何もなし
    "\x1b[48;2;127;127;127m__",  // 壁
    "\x1b[48;2;000;000;000m[]",  // ゴースト
    "\x1b[48;2;000;000;255m__",  // I
    "\x1b[48;2;000;255;000m__",  // O
    "\x1b[48;2;000;255;255m__",  // S
    "\x1b[48;2;255;000;000m__",  // Z
    "\x1b[48;2;255;000;255m__",  // J
    "\x1b[48;2;255;127;000m__",  // L
    "\x1b[48;2;255;255;000m__",  // T
];

// ブロックの種類
// ブロックの種類の最大値を表す定数を定義しています。
const BLOCK_KIND_MAX: usize = 7;
#[derive(Clone, Copy)]
pub enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

impl Distribution<BlockKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockKind {
        match rng.gen_range(0..=6) {
            0 => BlockKind::I,
            1 => BlockKind::O,
            2 => BlockKind::S,
            3 => BlockKind::Z,
            4 => BlockKind::J,
            5 => BlockKind::L,
            _ => BlockKind::T,
        }
    }
}

// ブロックの形状
pub type BlockShape = [[usize; 4]; 4];
// 各ブロックの形状を表す二次元配列を定義しています。
pub const BLOCKS: [BlockShape; BLOCK_KIND_MAX] = [
    // Iブロック
    [
        [0,0,0,0],
        [0,0,0,0],
        [I,I,I,I],
        [0,0,0,0],
    ],
    // Oブロック
    [
        [0,0,0,0],
        [0,O,O,0],
        [0,O,O,0],
        [0,0,0,0],
    ],
    // Sブロック
    [
        [0,0,0,0],
        [0,S,S,0],
        [S,S,0,0],
        [0,0,0,0],
    ],
    // Zブロック
    [
        [0,0,0,0],
        [Z,Z,0,0],
        [0,Z,Z,0],
        [0,0,0,0],
    ],
    // Jブロック
    [
        [0,0,0,0],
        [J,0,0,0],
        [J,J,J,0],
        [0,0,0,0],
    ],
    // Lブロック
    [
        [0,0,0,0],
        [0,0,L,0],
        [L,L,L,0],
        [0,0,0,0],
    ],
    // Tブロック
    [
        [0,0,0,0],
        [0,T,0,0],
        [T,T,T,0],
        [0,0,0,0],
    ],
];

// シャッフルされた7種のブロックを生成
pub fn gen_block_7() -> [BlockShape; BLOCK_KIND_MAX] {
    let mut rng = thread_rng();
    let mut que = [
        BlockKind::I,
        BlockKind::O,
        BlockKind::S,
        BlockKind::Z,
        BlockKind::J,
        BlockKind::L,
        BlockKind::T,
    ];
    que.shuffle(&mut rng);
    que.map(|block| BLOCKS[block as usize])
}
