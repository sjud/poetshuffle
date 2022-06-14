pub(crate) mod auth;





/*
fn is_owner(perm: &Permissions, originator_uuid: Uuid) -> Result<bool> {
    if perm.user_uuid != originator_uuid {
        Err(anyhow::Error::msg("Originator_uuid != user_uuid"))
    } else {
        Ok(true)
    }
}

/// Poets or greater can create sets.
/// As long as that set's originator uuid is equal to their user uuid
pub fn can_create_set(perm: &Permissions, originator_uuid: Uuid) -> Result<bool> {
    let _ = is_owner(perm, originator_uuid)?;
    if OrdRoles(perm.user_role) < OrdRoles(UserRole::Poet) {
        Err(anyhow::Error::msg("permission.user_role < poet."))
    } else {
        Ok(true)
    }
}
pub fn can_edit_set(perm: &Permissions, originator_uuid: Uuid) -> Result<bool> {
    is_owner(perm, originator_uuid)
}
/// Moderators or greater can Approve
pub fn can_approve(perm: &Permissions) -> Result<bool> {
    if OrdRoles(perm.user_role) < OrdRoles(UserRole::Moderator) {
        Err(anyhow::Error::msg("permission.user_role < moderator."))
    } else {
        Ok(true)
    }
}
/// Originators can change status
pub fn can_change_status(perm: &Permissions, originator_uuid: Uuid) -> Result<bool> {
    is_owner(&perm, originator_uuid)
}
*/

/*
/// Moderators or greater can Comment on every thing
/// Poets can comment on their submitted sets.
pub fn can_comment(perm:&Permissions,set_originator:Uuid) -> Result<bool> {
    if OrdRoles(perm.user_role) >= OrdRoles(UserRole::Moderator) || perm.user_uuid == set_originator {
        Ok(true)
    } else {
        Err(anyhow::Error::msg("User role !>= to moderator OR user_uuid != set_originator"))
    }
}*/
