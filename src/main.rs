// 外部ファイルを読み込む
// モジュールを読み込む
mod block;
mod game;

// スレッド間で同じ変数を参照するには、Arcを使用します。
// 複数のスレッドから同じ変数にアクセスするために使用されます。
use std::sync::{Arc, Mutex};
use std::{thread, time};
// キー入力の取得にはgetch-rsクレートを使用
use getch_rs::{Getch, Key};
// 外部ファイルで定義しているものを取り込む
// use game::*;は、gameモジュール内で定義されたすべての公開されたアイテム（構造体、列挙型、関数、定数など）を現在のスコープにインポートするためのコードです。
use game::*;

fn main() {
    // スレッド間で同じ変数を参照するには、Arcを使用します。
    // これはRcのスレッドセーフ版です。さらに、その変数の中身を変更したいので、
    // Mutexも使用します。これで、スレッド間で安全に変数の参照/変更ができます。
    // Game::newは新しいゲームの状態を生成する

    // Gameインスタンスを作成し、
    // Mutexは、複数のスレッドが同時に変数にアクセスできないようにするために使用する
    // Arcは複数のスレッドが"同じ"オブジェクトにアクセスできるようにするために使用
    // game変数は複数のスレッドからアクセスされ、Mutexを介して排他的にアクセスされる必要がある
    let game = Arc::new(Mutex::new(Game::new()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");
    // フィールドを描画
    draw(&game.lock().unwrap());

    // 自然落下処理
    {
        let game = Arc::clone(&game);
        //新規スレッドを生成するには、thread::spawn関数を呼び出し、新規スレッドで走らせたいコードを含むクロージャ（無名関数）を渡します。
        let _ = thread::spawn(move || {
            loop {
                // nミリ秒間スリーブする
                let sleep_msec = match 1000u64.saturating_sub((game.lock().unwrap().line as u64 / 10) * 100) {
                    0 => 100,
                    msec => msec,
                };
                thread::sleep(time::Duration::from_millis(sleep_msec));
                // 自然落下
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                if !is_collision(&game.field, &new_pos, &game.block) {
                    // posの座標を更新
                    game.pos = new_pos;
                } else {
                    // ブロック落下後の処理
                    if landing(&mut game).is_err() {
                        // ブロックを生成できないならゲームオーバー
                        gameover(&game);
                        break;
                    }
                }
                // フィールドを描画
                draw(&game);
            }
        });
    }

    // キー入力処理
    let g = Getch::new();
    loop {
        // キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                    y: game.pos.y,
                };
                // 現在のブロックを指定された位置に移動する
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Char('z')) => {
                // 左回転
                let mut game = game.lock().unwrap();
                // ブロックを左に回転
                rotate_left(&mut game);
                draw(&game);
            }
            Ok(Key::Char('x')) => {
                // 右回転
                let mut game = game.lock().unwrap();
                                //　ブロックを右に回転
                rotate_right(&mut game);
                draw(&game);
            }
            Ok(Key::Up) => {
                // ハードドロップ
                let mut game = game.lock().unwrap();
                hard_drop(&mut game);
                if landing(&mut game).is_err() {
                    // ブロックを生成できないならゲームオーバー
                    gameover(&game);
                    break;
                }
                draw(&game);
            }
            Ok(Key::Char(' ')) => {
                // ホールド
                let mut game = game.lock().unwrap();
                // 現在のブロックを保持し、新しいブロックを生成する。
                hold(&mut game);
                draw(&game);
            }
            Ok(Key::Char('q')) => {
                break;
            }
            _ => (),  // 何もしない
        }
    }

    // 終了処理
    quit();
}
