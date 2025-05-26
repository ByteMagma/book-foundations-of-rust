#[derive(Debug)]
enum StatusState {
    Bootup,
    Active,
    Shutdown
}

#[derive(Debug)]
enum SystemStatus<T> {
    Online(StatusState, T),
    Offline
}

fn main() {
    let status1: SystemStatus<&str> = SystemStatus::Online(StatusState::Bootup, "Initial checks");
    let status2: SystemStatus<&str> = SystemStatus::Offline;

    for status in [status1, status2].iter() {
        match status {
            SystemStatus::Online(state, data) => {
                println!("Online in state {:?} with data: {}", state, data);
            }
            SystemStatus::Offline => {
                println!("System is offline.");
            }
        }
    }
}
