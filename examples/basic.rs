extern crate riker;
use riker::actors::*;
use std::time::Duration;
use agnostik::block_on;

#[derive(Default)]
struct MyActor;

// implement the Actor trait
impl Actor for MyActor {
    type Msg = String;

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, _sender: Sender) {
        println!("{} received: {}", ctx.myself.name(), msg);
        let handle = ctx.run(async move {
            println!("print from async before sleep");
            std::thread::sleep(Duration::from_millis(5000));
            println!("print from async after sleep");
        });
        block_on(handle);
    }
}

// start the system and create an actor
fn main() {
    let sys = ActorSystem::new().unwrap();

    let my_actor = sys.actor_of::<MyActor>("my-actor").unwrap();

    my_actor.tell("Hello my actor!".to_string(), None);

    std::thread::sleep(Duration::from_millis(50000));


}
