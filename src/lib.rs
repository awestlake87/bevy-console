#![doc = include_str ! ("../README.md")]
#![deny(missing_docs)]

use bevy::prelude::{App, Plugin};
pub use bevy_console_derive::ConsoleCommand;
use bevy_egui::{EguiPlugin, EguiSettings};

use crate::commands::clear::{clear_command, ClearCommand};
use crate::commands::exit::{exit_command, ExitCommand};
use crate::commands::help::{help_command, HelpCommand};
pub use crate::console::{
    AddConsoleCommand, Command, ConsoleCommand, ConsoleCommandEntered, ConsoleConfiguration,
    ConsoleOpen, NamedCommand, PrintConsoleLine, ToggleConsoleKey,
};
// pub use color::{Style, StyledStr};

use crate::console::{console_ui, receive_console_line, ConsoleState};

// mod color;
mod commands;
mod console;
mod macros;

/// Console plugin.
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConsoleConfiguration>()
            .init_resource::<ConsoleState>()
            .init_resource::<ConsoleOpen>()
            .add_event::<ConsoleCommandEntered>()
            .add_event::<PrintConsoleLine>()
            .add_console_command::<ClearCommand, _>(clear_command)
            .add_console_command::<ExitCommand, _>(exit_command)
            .add_console_command::<HelpCommand, _>(help_command)
            .add_system(console_ui)
            .add_system(receive_console_line);

        // Don't create an egui context if one already exists.
        // This can happen if another plugin is using egui and was installed before us.
        if !app.world.contains_resource::<EguiSettings>() {
            app.add_plugin(EguiPlugin);
        }
    }
}
