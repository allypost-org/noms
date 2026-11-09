pub mod send;
#[cfg(feature = "server")]
pub mod server;

pub struct ExitCode(i32);
impl ExitCode {
    pub fn exit_process(&self) {
        std::process::exit(self.0);
    }
}
impl From<i32> for ExitCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

pub type CmdResult = Result<(), (String, ExitCode)>;
