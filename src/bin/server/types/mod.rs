use entity::sea_orm_active_enums::UserRole;
use anyhow::Result;
use entity::permissions::Model as Permissions;
use sea_orm::prelude::Uuid;
use std::cmp::Ordering;
pub struct Auth(pub Option<entity::permissions::Model>);

impl Auth{
    pub fn can_issue_promotion(&self,user_role:UserRole) -> bool {
        if let Some(permission) = &self.0 {
            // A greater role can issue a promotion to a lesser role.
            OrdRoles(permission.user_role) > OrdRoles(user_role)
        } else {
            // Someone with no permissions can't promote.
            false
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct OrdRoles(UserRole);
impl PartialOrd for OrdRoles {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_val = match self.0 {
            UserRole::Listener => 0,
            UserRole::Poet => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3,
            UserRole::SuperAdmin => 4,
        };
        let other_val = match other.0 {
            UserRole::Listener => 0,
            UserRole::Poet => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3,
            UserRole::SuperAdmin => 4,
        };
        if self_val < other_val {
            Some(Ordering::Less)
        } else if self_val > other_val {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}



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
