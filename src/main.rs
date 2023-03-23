use getch_rs::{Getch, Key};
use std::{thread, time};
// ブロックの種類
// 列挙型(enum)
#[derive(Clone, Copy)]
enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

// フィールドサイズ
const FIELD_WIDTH: usize = 11 + 2; // フィールド＋壁
const FIELD_HEIGHT: usize = 20 + 1; // フィールド＋底
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];
// ブロックの形状
type BlockShape = [[usize; 4]; 4];

// 3次元配列
// constを宣言するときは型を明示しなくてはなりません。
// 型を明示するときは変数（定数）名の右隣に : 型名 と書きます。
// 配列の型を明示するときは、 [型名; 要素数]
const BLOCKS: [BlockShape; 7] = [
    // Iブロック
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
    // Oブロック
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    // Sブロック
    [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
    // Zブロック
    [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    // Jブロック
    [[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
    // Lブロック
    [[0, 0, 0, 0], [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
    // Tブロック
    [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
];

struct Position {
    x: usize,
    y: usize,
}
// ブロックがフィールドに衝突する場合は"true"を返す
fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if (field[y + pos.y][x + pos.x] & BLOCKS[block as usize][y][x] == 1) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let field = [
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let mut pos = Position { x: 4, y: 0 };
    let g = Getch::new();
    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    loop {
        // 描画用フィールドの生成
        let mut field_buf = field;
        // 自然落下
        let new_pos = Position {
            x: pos.x,
            y: pos.y + 1,
        };
        // 当たり判定
        if !is_collision(&field, &new_pos, BlockKind::I) {
            pos = new_pos;
        }
        // 描画用フィールドにブロックの情報を書き込む
        for y in 0..4 {
            for x in 0..4 {
                // enumを添字にするには、usizeにキャスト
                // field_buf[y + 2][x + 2] = BLOCKS[BlockKind::I as usize][y][x];
                // field_buf[y + 2][x + 7] = BLOCKS[BlockKind::O as usize][y][x];
                // field_buf[y + 6][x + 2] = BLOCKS[BlockKind::S as usize][y][x];
                // field_buf[y + 6][x + 7] = BLOCKS[BlockKind::Z as usize][y][x];
                // field_buf[y + 10][x + 2] = BLOCKS[BlockKind::J as usize][y][x];
                // field_buf[y + 10][x + 7] = BLOCKS[BlockKind::L as usize][y][x];
                // field_buf[y + 14][x + 2] = BLOCKS[BlockKind::T as usize][y][x];
                if BLOCKS[BlockKind::I as usize][y][x] == 1 {
                    field_buf[y + pos.y][x + pos.x] = 1;
                }
            }
        }

        // フィールド描画
        println!("\x1b[H");
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                if field_buf[y][x] == 1 {
                    print!("[]");
                } else {
                    print!(" .");
                }
            }
            println!()
        }
        //1病患スリープする
        thread::sleep(time::Duration::from_millis(1000));
        //  キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let new_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                // 当たり判定
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    // posのy座標を更新
                    pos = new_pos;
                }
            }
            Ok(Key::Down) => {
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                // 当たり判定
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    // posのy座標を更新
                    pos = new_pos;
                }
            }
            Ok(Key::Right) => {
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                // 当たり判定
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    // posのy座標を更新
                    pos = new_pos;
                }
            }
            Ok(Key::Char('q')) => break,
            _ => (), // 何もしない
        }
    }
    // カーソルを再表示
    println!("\x1b[?25h");
}
