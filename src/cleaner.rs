use std::error::Error;

use crate::{
    cli::CliArgs,
    git::{delete_branch, get_current_name, get_local_branches, get_merged_branches, get_origin},
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
    let base = match args.base.as_str() {
        "main" => get_origin()?,
        _ => args.base,
    };
    println!("{base}");
    let merged_branches = get_merged_branches(&base)?;
    let mut msg: String = "Branches that can be safely deleted: \n".to_string();
    for i in 0..merged_branches.len() {
        msg = msg + &merged_branches[i] + "\n";
    }
    info(&msg);
    if args.dry_run {
        return Ok(());
    }
    for i in 0..merged_branches.len() {
        if merged_branches[i] != base {
            delete_branch(&merged_branches[i], args.force)?;
        }
    }
    Ok(())
}
