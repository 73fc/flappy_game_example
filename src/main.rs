use bracket_lib::prelude::*;

const SCREEN_WITDH: i32 = 80;
const SCREEN_HIGHT: i32 = 50;
const FRAME_DURATION: f32 = 50.0; //ms
const DRAGON_FRAMES: [u16; 6] = [64, 1, 2, 3, 2, 1];
struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
}

enum GameMode {
    Menu,
    Playing,
    End,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_HIGHT, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn restart(&mut self, _: &mut BTerm) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WITDH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are failed.");
        ctx.print_centered(6, &format!("you earned {} points", self.score));
        ctx.print_centered(8, "(P) play Again");
        ctx.print_centered(9, "(Q) play Again");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn paly(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        
        self.frame_time += ctx.frame_time_ms;
        if FRAME_DURATION < self.frame_time {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);

        ctx.print(0, 0, "Press Space to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.obstacle.x < self.player.x {
            self.score += 1;
            self.obstacle = Obstacle::new(
                self.player.x + SCREEN_WITDH, self.score
            );
        }

        if (SCREEN_HIGHT as f32) <  self.player.y || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.paly(ctx),
        }
    }
}

struct  Player {
    x: i32,
    y: f32,
    velocity: f32,
    frame: usize,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y: y as f32,
            velocity: 0.0,
            frame: 0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(0.0, self.y),
            1,
            Degrees::new(0.0), 
            PointF::new(2.0, 2.0),
            WHITE,
            NAVY, 
        DRAGON_FRAMES[self.frame]
        );
        ctx.set_active_console(0);
    }
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.1
        }
        self.y += self.velocity;
        self.x += 1;
        self.frame = (self.frame + 1) % 6;
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -1.0;
    }
}


struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(3, 20 - score)
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x, 
                y, 
                RED, 
                BLACK, 
                to_cp437('|')
            )
        }

        for y in self.gap_y + half_size..SCREEN_HIGHT {
            ctx.set(
                screen_x, 
                y, 
                RED, 
                BLACK, 
                to_cp437('|')
            )
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let dose_x_match = player.x == self.x;
        let player_above_gap = player.y < (self.gap_y - half_size) as f32;
        let player_below_gap = ((self.gap_y + half_size)  as f32) < player.y;
        dose_x_match && (player_above_gap || player_below_gap)
    }
}

fn main() -> BError {
    // let context = BTermBuilder::simple80x50()
    //     .with_title("Flappy Dragon")
    //     .build()?;
    let context = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WITDH, SCREEN_HIGHT, "../resources/flappy32.png")
        .with_fancy_console(SCREEN_WITDH, SCREEN_HIGHT, "../resources/flappy32.png")
        .with_title("Flappy Dragon Enhanced")
        .with_tile_dimensions(16, 16)
        .build()?;
    main_loop(context, State::new())
}
