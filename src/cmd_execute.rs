use std::process::Command;

pub trait DebugCommand {
    fn execute_and_print_errors(&mut self, program_name: Option<&str>, description: &str);
}

impl DebugCommand for Command {
    #[track_caller]
    fn execute_and_print_errors(&mut self, program_name: Option<&str>, description: &str) {
        let program = program_name
            .map(|x| x.to_string())
            .unwrap_or_else(|| self.get_program().to_string_lossy().to_string());

        let Ok(process) = self.spawn() else {
            panic!("ERROR: {program} failed to execute.\n  Hint: make sure it exists in your path")
        };
        let output = process.wait_with_output().unwrap();

        if !output.status.success() {
            eprintln!("ERROR: {program} failed when trying to: {description}");
            eprintln!("------ {program} (STDOUT) -----");
            eprintln!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("------ {program} (STDERR) -----");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            panic!("{} failed", program);
        }
    }
}
