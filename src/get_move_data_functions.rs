use rustemon::model::machines::Machine;
use rustemon::model::moves::Move;


async fn get_move_by_name(name: String) -> Move {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let move_ = rustemon::moves::move_::get_by_name(&name, &rustemon_client).await;
    return move_.unwrap();
}

async fn get_machine_by_id(id: i64) -> Machine {
    let rustemon_client = rustemon::client::RustemonClient::default();
    let machine = rustemon::machines::machine::get_by_id(id, &rustemon_client).await;
    return machine.unwrap();
}


