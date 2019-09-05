use cloudflare::endpoints::workerskv::delete_key::DeleteKey;
use cloudflare::framework::apiclient::ApiClient;

use crate::commands::kv;
use crate::settings::global_user::GlobalUser;
use crate::settings::project::Project;
use crate::terminal::message;

pub fn delete_key(
    project: &Project,
    user: GlobalUser,
    id: &str,
    key: &str,
) -> Result<(), failure::Error> {
    let client = kv::api_client(user)?;

    match kv::interactive_delete(&format!("Are you sure you want to delete key \"{}\"?", key)) {
        Ok(true) => (),
        Ok(false) => {
            message::info(&format!("Not deleting key \"{}\"", key));
            return Ok(());
        }
        Err(e) => failure::bail!(e),
    }

    let msg = format!("Deleting key \"{}\"", key);
    message::working(&msg);

    let response = client.request(&DeleteKey {
        account_identifier: &project.account_id,
        namespace_identifier: id,
        key: key, // this is url encoded within cloudflare-rs
    });

    match response {
        Ok(_success) => message::success("Success"),
        Err(e) => kv::print_error(e),
    }

    Ok(())
}