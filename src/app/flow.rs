use std::io;

use crate::ipc::IpcClient;
use crate::layout::resize;
use crate::state::{restore_if_present, state_path};

use super::AppCommand;
use super::arrange::arrange;
use super::context::current_workspace;

pub fn run(client: &mut impl IpcClient, command: AppCommand) -> io::Result<()> {
    match command {
        AppCommand::GrowMaster | AppCommand::ShrinkMaster => run_resize(client, command),
        AppCommand::Toggle => run_toggle(client),
    }
}

fn run_resize(client: &mut impl IpcClient, command: AppCommand) -> io::Result<()> {
    let Some(context) = current_workspace(client, state_path)? else {
        return Ok(());
    };

    resize(client, context.workspace_id, &context.state_path, command)
}

fn run_toggle(client: &mut impl IpcClient) -> io::Result<()> {
    let Some(context) = current_workspace(client, state_path)? else {
        return Ok(());
    };

    if restore_if_present(client, &context.state_path)? {
        return Ok(());
    }

    let Some(context) = current_workspace(client, state_path)? else {
        return Ok(());
    };

    arrange(client, &context)
}
