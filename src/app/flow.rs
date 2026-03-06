use std::io;

use crate::cli::AppCommand;
use crate::ipc::IpcClient;
use crate::state_file::state_file_path;

use super::context::focused_context;
use super::layout_phase::apply_master_stack_layout;
use super::resize_phase::resize_master_window;
use super::restore_phase::try_restore_layout;

pub fn run(client: &mut impl IpcClient, command: AppCommand) -> io::Result<()> {
    let Some(initial_context) = focused_context(client, state_file_path)? else {
        return Ok(());
    };

    match command {
        AppCommand::GrowMaster | AppCommand::ShrinkMaster => {
            return resize_master_window(client, &initial_context, command);
        }
        AppCommand::Toggle => {
            if try_restore_layout(client, &initial_context.state_path)? {
                return Ok(());
            }
        }
    }

    let Some(context) = focused_context(client, state_file_path)? else {
        return Ok(());
    };

    apply_master_stack_layout(client, &context)
}
