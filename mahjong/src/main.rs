
// フレームを表示するためのクレートを指定
use piston_window::*;

// メイン関数
fn main() {

    // フレームの外枠を定義
    let mut window: PistonWindow = 
        WindowSettings::new("麻雀ゲーム", [640, 640])
        .exit_on_esc(true).build().unwrap();

    // フレームの中身を定義
    // Some：タプル
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], //[左上から見て(0,0)から100××100の正方形になる]
                      c.transform, g);
        });
    }
}