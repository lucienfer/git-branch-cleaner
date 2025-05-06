use std::error::Error;

use crate::{
    cli::{CliArgs, confirm},
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

    let mut merged_branches = get_merged_branches(&base)?;
    merged_branches.retain(|x| x != &base);
    if merged_branches.is_empty() {
        info("No merged branches to delete.");
        return Ok(());
    }

    let msg: String = format!(
        "Branches that can be safely deleted:\n{}",
        merged_branches.join("\n")
    );
    info(&msg);
    if args.dry_run {
        return Ok(());
    }

    let mut count = 0;
    if args.interactive {
        for branch in &merged_branches {
            if confirm(&format!("Delete branch {}", branch)) {
                delete_branch(&branch, args.force)?;
                count += 1;
            }
        }
    } else {
        if confirm("Do you want delete this branch ?") {
            for branch in merged_branches {
                if branch != base {
                    delete_branch(&branch, args.force)?;
                    count += 1;
                }
            }
        }
    }
    info(&format!("{} branches deleted.", count));
    Ok(())
}
