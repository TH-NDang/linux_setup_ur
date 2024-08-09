use std::process;

use crate::utils::Status;

use super::ErrorHandler;

pub trait CommandRunner: ErrorHandler {
    fn setup_command(&self) -> process::Command;

    fn is_run_spawn(&self) -> bool {
        false
    }

    fn run(&self) -> Status {
        if self.is_run_spawn() {
            let mut child = match self.setup_command().spawn() {
                Ok(child) => child,
                Err(e) => {
                    Self::handle_command_error(&format!("{}", e));
                    return Status::Failure;
                }
            };

            match &child.wait() {
                Ok(status) => {
                    if status.success() {
                        Status::Success
                    } else {
                        Status::Failure
                    }
                }
                Err(e) => {
                    Self::handle_command_error(&format!("{}", e));
                    Status::Failure
                }
            }
        } else {
            match self.setup_command().output() {
                Ok(output) => {
                    if output.status.success() {
                        Status::Success
                    } else {
                        Self::handle_command_error(&format!("{:?}", output));
                        Status::Failure
                    }
                }
                Err(e) => {
                    Self::handle_command_error(&format!("{}", e));
                    Status::Failure
                }
            }
        }
    }
}

pub trait ProcessRunner: CommandRunner {
    fn before_run(&self) -> Status;
    fn after_run(&self, command_status: Status) -> Status;
    fn print_pre_run_info(&self);
    fn execute(&self) -> Status {
        match self.before_run() {
            Status::Passed => return Status::Passed,
            Status::Failure => return Status::Failure,
            Status::Skipped => return Status::Skipped,
            _ => (),
        };

        self.print_pre_run_info();
        let status = self.run();

        match self.after_run(status) {
            Status::Passed => return Status::Passed,
            Status::Failure => return Status::Failure,
            Status::Skipped => return Status::Skipped,
            _ => Status::Success,
        }
    }
}
