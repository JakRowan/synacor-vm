#[derive(PartialEq)]
pub enum EvalStatus {
    CommandNotFound,
    Success,
    _Failure,
}

pub fn eval(command: &str) -> EvalStatus {
    let command_name = command.split_whitespace().nth(0);
    let command_args = command.split_whitespace().skip(1).collect::<Vec<_>>();

    match command_name {
        Some("ping") => ping(command_args),
        _ => EvalStatus::CommandNotFound,
    }
}

fn ping(_args: Vec<&str>) -> EvalStatus {
    println!("Pong!\n");
    EvalStatus::Success
}
