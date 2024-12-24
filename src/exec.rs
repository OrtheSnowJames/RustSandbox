use std::process::Command;
use std::process::Output;

#[cfg(target_family = "unix")]
const SHELL: &str = "bash";
#[cfg(target_family = "windows")]
const SHELL: &str = "cmd";

pub struct Exec {
    last_output: Option<Output>,
}

/// Returns a reference to the last output of the execution, if available.
///
/// # Examples
///
/// ```
/// let exec = Exec::new();
/// // Execute some command
/// exec.run("echo Hello, world!");
///
/// if let Some(output) = exec.last_output() {
///     println!("Last output: {:?}", output);
/// } else {
///     println!("No output available.");
/// }
/// ```
impl Exec {
    pub fn last_output(&self) -> Option<&Output> {
        self.last_output.as_ref()
    }
}

impl Exec {
    pub fn new() -> Self {
        Exec { last_output: None }
    }

    pub fn run(&mut self, command: &str) -> std::io::Result<&Output> {
        #[cfg(target_family = "windows")]
        let translated_command = Self::translate_to_windows(command);
        #[cfg(target_family = "unix")]
        let translated_command = command.to_string();

        let output = if cfg!(target_family = "windows") {
            Command::new(SHELL)
                .args(["/C", &translated_command])
                .output()?
        } else {
            Command::new(SHELL)
                .args(["-c", &translated_command])
                .output()?
        };

        self.last_output = Some(output);
        Ok(self.last_output.as_ref().unwrap())
    }

    #[cfg(target_family = "windows")]
    fn translate_to_windows(command: &str) -> String {
        let mut parts = command.split_whitespace();
        match parts.next().unwrap_or("") {
            "ls" => "dir".to_string(),
            "rm" => command.replace("rm", "del"),
            "cp" => command.replace("cp", "copy"),
            "mv" => command.replace("mv", "move"),
            "cat" => command.replace("cat", "type"),
            "pwd" => "cd".to_string(),
            "clear" => "cls".to_string(),
            "export" => {
                let var = parts.next().unwrap_or("");
                let value = parts.collect::<Vec<&str>>().join(" ");
                format!("set {}={}", var, value)
            }
            _ => command.to_string(),
        }
    }
}
/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_creation() {
        let exec = Exec::new();
        assert!(exec.last_output.is_none());
    }

    #[test]
    //                        ATTENTION
    //
    //
    //        this WILL error on linux, but it is a windows test
    fn test_translate_to_windows() {
        assert_eq!(Exec::translate_to_windows("ls"), "dir");
        assert_eq!(Exec::translate_to_windows("rm file.txt"), "del file.txt");
        assert_eq!(Exec::translate_to_windows("cp file1.txt file2.txt"), "copy file1.txt file2.txt");
        assert_eq!(Exec::translate_to_windows("mv file1.txt file2.txt"), "move file1.txt file2.txt");
        assert_eq!(Exec::translate_to_windows("cat file.txt"), "type file.txt");
        assert_eq!(Exec::translate_to_windows("pwd"), "cd");
        assert_eq!(Exec::translate_to_windows("clear"), "cls");
        assert_eq!(Exec::translate_to_windows("export VAR=value"), "set VAR=value");
    }
}
*/