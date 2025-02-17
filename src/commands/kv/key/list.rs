extern crate serde_json;

use crate::commands::kv;
use crate::commands::kv::key::KeyList;
use crate::settings::global_user::GlobalUser;
use crate::settings::target::Target;

// Note: this function only prints keys in json form, given that
// the number of entries in each json blob is variable (so csv and tsv
// representation won't make sense)
pub fn list(
    target: &Target,
    user: GlobalUser,
    namespace_id: &str,
    prefix: Option<&str>,
) -> Result<(), failure::Error> {
    let key_list = KeyList::new(target, user, namespace_id, prefix)?;

    print!("["); // Open json list bracket

    let mut first_key = true;

    for key_result in key_list {
        match key_result {
            Ok(key) => {
                if first_key {
                    first_key = false;
                } else {
                    print!(",");
                }

                print!("{}", serde_json::to_string(&key)?);
            }
            Err(e) => print!("{}", kv::format_error(e)),
        }
    }

    print!("]"); // Close json list bracket

    Ok(())
}
