use wasm_runner::agent::SolveOneTask;
use yew_agent::Registrable;

fn main() {
    SolveOneTask::registrar().register();
}
