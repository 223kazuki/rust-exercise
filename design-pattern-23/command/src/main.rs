// T型のtargetに対するコマンドtrait
trait Command<T> {
    // target にコマンド実行
    fn execute(&self, &mut T);
    // コマンド戻す
    fn undo(&self, &mut T);
}

// Command 実行管理
struct Invoker<'a, T: 'a> {
    // Stackされたコマンド
    // Command trait を実装して、属する参照が'a より寿命の短い要素を持つ型のVec
    // T 型に対するコマンド
    commands: Vec<Box<Command<T> + 'a>>,
    // T 型のコマンド実行対象
    target: &'a mut T,
    // 現在の実行コマンドインデックス
    current_index: usize,
}
impl<'a, T> Invoker<'a, T> {
    // target を指定して Invoker 生成
    // t はInvoker と同じライフタイム
    // 可変参照
    fn new(t: &'a mut T) -> Self {
        Invoker {
            commands: Vec::new(),
            target: t,
            current_index: 0,
        }
    }

    // 可変参照型のtarget を返す
    fn target(&self) -> &T {
        self.target
    }

    // コマンド追加
    // Command trait を実装して、属する参照が'a より寿命の短い要素を持つ型を受け取る
    fn append_command<U: Command<T> + 'a>(&mut self, c: U) {
        self.commands.push(Box::new(c));
    }

    // コマンド実行
    fn execute_command(&mut self) {
        if self.commands.len() <= self.current_index {
            // 全コマンド実行済みなら終了
            return;
        }

        // 現在のインデックスのコマンドをアンボックスした結果の参照
        let c = &*self.commands[self.current_index];
        // target の可変参照
        let t = &mut *self.target;
        // コマンド実行
        c.execute(t);
        // インデックスインクリメント
        self.current_index += 1;
    }

    // 全コマンド実行
    fn execute_all_commands(&mut self) {
        // 現在のインデックスから最後まで
        for _ in self.current_index..self.commands.len() {
            self.execute_command();
        }
    }

    // コマンド実行取り消し
    fn undo(&mut self) {
        if 0 == self.current_index {
            return;
        }
        // 一つ前のコマンド
        self.current_index -= 1;
        let c = &*self.commands[self.current_index];
        let t = &mut *self.target;
        // 一つ前のコマンドundo 呼び出し
        c.undo(t);
    }
}

// Debug, Eq, PartialEq Traitを継承する構造体
// 位置とベクトル
#[derive(Debug, Eq, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    }
    fn move_forward(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
    fn set_direction(&mut self, d: (i32, i32)) {
        self.dx = d.0;
        self.dy = d.1;
    }
    fn get_direction(&self) -> (i32, i32) {
        (self.dx, self.dy)
    }
}

// 一つ前に進むコマンド
struct CommandMoveForward;
impl Command<Robot> for CommandMoveForward {
    fn execute(&self, r: &mut Robot) {
        // 対象を一つすすめる
        r.move_forward();
    }

    fn undo(&self, r: &mut Robot) {
        let c1 = CommandTurnRight;
        c1.execute(r);
        c1.execute(r);
        self.execute(r);
        c1.execute(r);
        c1.execute(r);
    }
}

struct CommandTurnRight;
impl Command<Robot> for CommandTurnRight {
    fn execute(&self, r: &mut Robot) {
        let (dx, dy) = r.get_direction();
        r.set_direction((dy, -dx));
    }
    fn undo(&self, r: &mut Robot) {
        let c = CommandTurnLeft;
        c.execute(r);
    }
}

struct CommandTurnLeft;
impl Command<Robot> for CommandTurnLeft {
    fn execute(&self, r: &mut Robot) {
        let (dx, dy) = r.get_direction();
        r.set_direction((-dy, dx));
    }
    fn undo(&self, r: &mut Robot) {
        let c = CommandTurnRight;
        c.execute(r);
    }
}

fn main() {
    let mut r = Robot::new();

    let mut invoker = Invoker::new(&mut r);
    // target はどうなったか
    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    );

    invoker.append_command(CommandTurnRight);
    invoker.append_command(CommandTurnLeft);
    invoker.append_command(CommandMoveForward);

    invoker.execute_all_commands();
    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 1,
            dx: 0,
            dy: 1,
        }
    );

    invoker.undo();
    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    );

    invoker.undo();
    assert_eq!(
        *invoker.target(),
        Robot {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
        }
    );
}
