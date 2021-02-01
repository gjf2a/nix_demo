use nix::unistd::{fork, ForkResult, execvp};
use std::io;
use std::ffi::CString;
use nix::sys::wait::waitpid;

fn main() -> io::Result<()> {
    match unsafe {fork()}.unwrap() {
        ForkResult::Parent { child } => {
            println!("I'm the parent! The child is: {}", child);
            waitpid(child, None);
        }
        ForkResult::Child => {
            println!("I'm the child.");
            let ls = CString::new("ls").unwrap();
            execvp(ls.as_c_str(), &[ls.as_c_str()]).unwrap();
        }
    }
    Ok(())
}
