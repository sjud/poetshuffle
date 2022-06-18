use entity::sea_orm_active_enums::UserRole;
use anyhow::Result;
use entity::permissions::Model as Permissions;
use sea_orm::prelude::Uuid;
use std::cmp::Ordering;

pub struct Auth(pub Option<entity::permissions::Model>);

impl Auth{
    pub fn can_edit_set(&self,set:&entity::sets::Model) -> bool {
        // Can only edit sets that haven't been approved.
        if !set.approved {
            if let Some(permission) = &self.0 {
                // If you created the set you can edit the title.
              set.originator_uuid == permission.user_uuid
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn can_issue_promotion(&self,user_role:UserRole) -> bool {
        if let Some(permission) = &self.0 {
            // A greater role can issue a promotion to a lesser role.
            OrdRoles(permission.user_role) > OrdRoles(user_role)
        } else {
            // Someone with no permissions can't promote.
            false
        }
    }
    pub fn can_read_pending_set(&self, set:&entity::sets::Model)
    -> bool {
        if let Some(permission) = &self.0 {
            if OrdRoles(permission.user_role) >= OrdRoles(UserRole::Moderator) {
                true
            } else {
                set.originator_uuid == permission.user_uuid
            }
        } else {
            false
        }
    }

    pub fn uuid(&self) -> Result<Uuid> {
        if let Some(permission) = &self.0 {
            // A greater role can issue a promotion to a lesser role.
            Ok(permission.user_uuid)
        } else {
            // Someone with no permissions can't promote.
            Err(anyhow::Error::msg("No permission, no uuid."))
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct OrdRoles(pub UserRole);
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