use seahorse::{App, Context, Command};
use std::env;

fn main() {



    let args: Vec<String> = env::args().collect();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(help_action)
        .command(reverse_command());
    app.run(args);

}    

fn help_action(_c: &Context) {
    println!("usage:");
    println!(" reverse <MOVELIST>");
}

fn reverse_command() -> Command {
  Command::new("reverse").action(reverse_action)
}

fn reverse_action(c: &Context) {
    let move_list : Vec<&str> = c.args.iter().map(|n| n.as_str()).collect();
    rubiks_tool::three_by_three::reverse(move_list);
}
