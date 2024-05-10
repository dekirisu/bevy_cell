/// One way to use Timers without the need of bevy Resources
/// 
/// Downsides:
/// - Only usable in one ECS, since its a global variable
/// - You have to make sure there's no other system reading while it's beeing mutated
///     - This is always the case if mutated and read only happens in one system (unless async tasks are used)
///     - Otherwise You have to make sure by using .before() .after() or .chain() on affected systems

use bevy::prelude::*;
use bevy_cell::*;

fn main(){
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    // NOTE: .chain() is necessary to avoid reading and mutating at the same time!
    app.add_systems(Update,(progress,print).chain()); 
    app.run();
}

fn repeater(duration:f32) -> Timer {
    Timer::from_seconds(duration,TimerMode::Repeating)
}

bycell!{ 
    // On the Timer struct: add a method named 'one_second' which returns a
    // mutable reference to the static stored value {..} with the type Self 
    // (Self is assumed if nothing is set before the ':')
    Timer: [:mut one_second {repeater(1.)}];
    // On the Camera struct: add a method named 'five_seconds' which returns a
    // mutable reference to the static stored value {..} with the type Timer 
    Camera: [Timer:mut five_seconds {repeater(5.)}];
}

fn progress (time:Res<Time>) {
    Timer::one_second().tick(time.delta());
    Camera::five_seconds().tick(time.delta());
}

fn print () {
    if Camera::five_seconds().just_finished() {
        println!("every 5 seconds");
    }
    else if Timer::one_second().just_finished() {
        println!("every second");
    }
}