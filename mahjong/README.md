
# 麻雀ゲームのロジック

麻雀ゲームを作成するにあたって、基本的に必要となる機能としては以下の通りである。

- 麻雀台が表示される
- 麻雀牌が表示される
- 麻雀牌は14枚、一枚捨てるとその牌が河に追加され、牌は13枚になる。
- 13枚の状態で1枚ツモると自動で理牌され、14枚表示される

pistonで開発を進める（BevyはWSL2と相性が悪く、fyroxは重すぎる）

```bash
cargo new mahjong
cd mahjong
cargo add piston_window
```

```rust

use piston_window::*;

fn main() {
    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // recd ..d
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}
```

フレームが画面に表示される。
```bash
cargo run
```