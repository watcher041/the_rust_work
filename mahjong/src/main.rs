
// フレームを表示させるクレートを指定
extern crate piston_window;
use piston_window::*;

// メイン関数
fn main() {

    // フレームの外枠を設定
    let mut window: PistonWindow = 
        WindowSettings::new("麻雀ゲーム", [630, 630])
            .exit_on_esc(true) // escでフレームを終了
            .build()
            .unwrap();
    
    // フレームの内容を設定
    while let Some(event) = window.next() {

        // 2Dで画像を描写
        window.draw_2d(&event, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}