use std::error::Error;

use crate::{
    cli::CliArgs,
    git::{delete_branch, get_current_name, get_local_branches, get_merged_branches},
    utils::info,
};

///Run the logic of the CLI tools
///
/// # Argument
/// * `args` - CliArgs
///
/// # Return
/// `Ok(())` if the function is a success or an Error
pub fn run(args: CliArgs) -> Result<(), Box<dyn Error>> {
    info("Tools start");
    let local_branch = get_current_name()?;
    Ok(())
}
