use std::io;

use crate::ipc::IpcClient;
use crate::layout::resize;
use crate::state::{restore_if_present, state_path};

use super::arrange::arrange;
use super::context::current_workspace;
use super::AppCommand;

pub fn run(client: &mut impl IpcClient, command: AppCommand) -> io::Result<()> {
    let Some(initial_context) = current_workspace(client, state_path)? else {
        return Ok(());
    };

    match command {
        AppCommand::GrowMaster | AppCommand::ShrinkMaster => {
            return resize(
                client,
                initial_context.workspace_id,
                &initial_context.state_path,
                command,
            );
        }
        AppCommand::Toggle => {
            if restore_if_present(client, &initial_context.state_path)? {
                return Ok(());
            }
        }
    }

    let Some(context) = current_workspace(client, state_path)? else {
        return Ok(());
    };

    arrange(client, &context)
}
