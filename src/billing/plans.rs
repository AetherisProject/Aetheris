//! Family and Enterprise plan features for Aetheris.
//!
//! Implements family sharing, enterprise features, and plan management.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::vault::store::VaultStore;
use crate::crypto::CryptoEngine;

/// Plan type for Aetheris.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlanType {
    /// Free plan with limited features
    Free,
    /// Premium plan with full features
    Premium,
    /// Family plan with sharing
    Family,
    /// Enterprise plan with team features
    Enterprise,
}

impl PlanType {
    /// Get the plan name.
    pub fn name(&self) -> String {
        match self {
            PlanType::Free => "Free".to_string(),
            PlanType::Premium => "Premium".to_string(),
            PlanType::Family => "Family".to_string(),
            PlanType::Enterprise => "Enterprise".to_string(),
        }
    }

    /// Get the plan price.
    pub fn price(&self) -> f64 {
        match self {
            PlanType::Free => 0.0,
            PlanType::Premium => 4.99,
            PlanType::Family => 9.99,
            PlanType::Enterprise => 19.99,
        }
    }

    /// Get the plan features.
    pub fn features(&self) -> Vec<String> {
        match self {
            PlanType::Free => vec![
                "Basic vault".to_string(),
                "Single device".to_string(),
                "Limited items".to_string(),
            ],
            PlanType::Premium => vec![
                "Unlimited vault".to_string(),
                "Multi-device sync".to_string(),
                "Priority support".to_string(),
                "Advanced security".to_string(),
            ],
            PlanType::Family => vec![
                "All Premium features".to_string(),
                "Family sharing".to_string(),
                "Shared vaults".to_string(),
                "Family admin".to_string(),
            ],
            PlanType::Enterprise => vec![
                "All Family features".to_string(),
                "Team management".to_string(),
                "SSO integration".to_string(),
                "Audit logging".to_string(),
                "Dedicated support".to_string(),
            ],
        }
    }
}

/// Plan for a user or organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    /// Plan ID
    pub id: Uuid,
    /// User ID or organization ID
    pub user_id: String,
    /// Plan type
    pub plan_type: PlanType,
    /// Start date
    pub start_date: DateTime<Utc>,
    /// End date
    pub end_date: DateTime<Utc>,
    /// Status
    pub status: PlanStatus,
    /// Billing cycle
    pub billing_cycle: BillingCycle,
    /// Number of users (for Family/Enterprise)
    pub user_count: u32,
    /// Max users (for Family/Enterprise)
    pub max_users: u32,
}

impl Plan {
    /// Create a new plan.
    pub fn new(user_id: String, plan_type: PlanType, billing_cycle: BillingCycle, user_count: u32, max_users: u32) -> Self {
        Plan {
            id: Uuid::new_v4(),
            user_id,
            plan_type,
            start_date: Utc::now(),
            end_date: Utc::now() + chrono::Duration::days(30),
            status: PlanStatus::Active,
            billing_cycle,
            user_count,
            max_users,
        }
    }

    /// Check if the plan is active.
    pub fn is_active(&self) -> bool {
        self.status == PlanStatus::Active && self.end_date > Utc::now()
    }

    /// Check if the plan is expired.
    pub fn is_expired(&self) -> bool {
        self.end_date <= Utc::now()
    }
}

/// Plan status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlanStatus {
    /// Plan is active
    Active,
    /// Plan is expired
    Expired,
    /// Plan is canceled
    Canceled,
    /// Plan is suspended
    Suspended,
}

/// Billing cycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BillingCycle {
    /// Monthly billing
    Monthly,
    /// Annual billing
    Annual,
}

/// Plan manager for Aetheris.
pub struct PlanManager {
    vault_store: VaultStore,
    crypto_engine: CryptoEngine,
}

impl PlanManager {
    /// Create a new plan manager.
    pub fn new(vault_store: VaultStore) -> Self {
        PlanManager {
            vault_store,
            crypto_engine: CryptoEngine::new().unwrap(),
        }
    }

    /// Create a new plan.
    pub fn create_plan(&self, user_id: String, plan_type: PlanType, billing_cycle: BillingCycle, user_count: u32, max_users: u32) -> Result<Plan> {
        let plan = Plan::new(user_id, plan_type, billing_cycle, user_count, max_users);
        self.vault_store.insert_session(plan.clone())?;
        Ok(plan)
    }

    /// Get a plan by ID.
    pub fn get_plan(&self, plan_id: Uuid) -> Result<Option<Plan>> {
        let plans: Vec<Plan> = self.vault_store.list_items::<Plan>()?;
        for plan in plans {
            if plan.id == plan_id {
                return Ok(Some(plan));
            }
        }
        Ok(None)
    }

    /// Get a plan by user ID.
    pub fn get_plan_by_user(&self, user_id: String) -> Result<Option<Plan>> {
        let plans: Vec<Plan> = self.vault_store.list_items::<Plan>()?;
        for plan in plans {
            if plan.user_id == user_id {
                return Ok(Some(plan));
            }
        }
        Ok(None)
    }

    /// Update a plan.
    pub fn update_plan(&self, plan: Plan) -> Result<()> {
        self.vault_store.update_session(plan.id, plan)?;
        Ok(())
    }

    /// Delete a plan.
    pub fn delete_plan(&self, plan_id: Uuid) -> Result<()> {
        self.vault_store.delete_session(&plan_id)?;
        Ok(())
    }

    /// List all plans.
    pub fn list_plans(&self) -> Result<Vec<Plan>> {
        self.vault_store.list_items::<Plan>()
    }

    /// Upgrade a plan.
    pub fn upgrade_plan(&self, user_id: String, new_plan_type: PlanType) -> Result<Plan> {
        let plan = self.get_plan_by_user(user_id.clone())?;
        
        if let Some(mut plan) = plan {
            plan.plan_type = new_plan_type;
            plan.start_date = Utc::now();
            plan.end_date = Utc::now() + chrono::Duration::days(30);
            self.update_plan(plan.clone())?;
            Ok(plan)
        } else {
            // Create a new plan if none exists
            let plan = Plan::new(user_id, new_plan_type, BillingCycle::Monthly, 1, 5);
            self.vault_store.insert_session(plan.clone())?;
            Ok(plan)
        }
    }

    /// Downgrade a plan.
    pub fn downgrade_plan(&self, user_id: String, new_plan_type: PlanType) -> Result<Plan> {
        let plan = self.get_plan_by_user(user_id.clone())?;
        
        if let Some(mut plan) = plan {
            plan.plan_type = new_plan_type;
            plan.start_date = Utc::now();
            plan.end_date = Utc::now() + chrono::Duration::days(30);
            self.update_plan(plan.clone())?;
            Ok(plan)
        } else {
            // Create a new plan if none exists
            let plan = Plan::new(user_id, new_plan_type, BillingCycle::Monthly, 1, 1);
            self.vault_store.insert_session(plan.clone())?;
            Ok(plan)
        }
    }

    /// Check if a user can add more users.
    pub fn can_add_user(&self, user_id: String) -> Result<bool> {
        let plan = self.get_plan_by_user(user_id)?;
        
        if let Some(plan) = plan {
            return Ok(plan.user_count < plan.max_users);
        }
        
        Ok(false)
    }

    /// Add a user to a plan.
    pub fn add_user(&self, user_id: String, new_user_id: String) -> Result<()> {
        let plan = self.get_plan_by_user(user_id.clone())?;
        
        if let Some(mut plan) = plan {
            if plan.user_count >= plan.max_users {
                return Err(anyhow::anyhow!("Max users reached"));
            }
            plan.user_count += 1;
            self.update_plan(plan)?;
        }
        
        Ok(())
    }

    /// Remove a user from a plan.
    pub fn remove_user(&self, user_id: String) -> Result<()> {
        let plan = self.get_plan_by_user(user_id.clone())?;
        
        if let Some(mut plan) = plan {
            if plan.user_count > 0 {
                plan.user_count -= 1;
                self.update_plan(plan)?;
            }
        }
        
        Ok(())
    }
}

/// Family sharing for Aetheris.
pub struct FamilyManager {
    vault_store: VaultStore,
    plan_manager: PlanManager,
}

impl FamilyManager {
    /// Create a new family manager.
    pub fn new(vault_store: VaultStore) -> Self {
        FamilyManager {
            vault_store,
            plan_manager: PlanManager::new(vault_store),
        }
    }

    /// Create a family group.
    pub fn create_family(&self, owner_id: String, name: String) -> Result<FamilyGroup> {
        let group = FamilyGroup::new(owner_id, name);
        self.vault_store.insert_session(group.clone())?;
        Ok(group)
    }

    /// Add a member to a family group.
    pub fn add_member(&self, group_id: Uuid, user_id: String, role: FamilyRole) -> Result<()> {
        let group = self.get_family(group_id)?;
        
        if let Some(mut group) = group {
            group.members.push(FamilyMember {
                user_id,
                role,
                joined_at: Utc::now(),
            });
            self.vault_store.update_session(group_id, group)?;
        }
        
        Ok(())
    }

    /// Remove a member from a family group.
    pub fn remove_member(&self, group_id: Uuid, user_id: String) -> Result<()> {
        let group = self.get_family(group_id)?;
        
        if let Some(mut group) = group {
            group.members.retain(|m| m.user_id != user_id);
            self.vault_store.update_session(group_id, group)?;
        }
        
        Ok(())
    }

    /// Get a family group.
    pub fn get_family(&self, group_id: Uuid) -> Result<Option<FamilyGroup>> {
        let groups: Vec<FamilyGroup> = self.vault_store.list_items::<FamilyGroup>()?;
        for group in groups {
            if group.id == group_id {
                return Ok(Some(group));
            }
        }
        Ok(None)
    }

    /// List all family groups.
    pub fn list_families(&self) -> Result<Vec<FamilyGroup>> {
        self.vault_store.list_items::<FamilyGroup>()
    }
}

/// Family group for sharing vault items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyGroup {
    /// Group ID
    pub id: Uuid,
    /// Group name
    pub name: String,
    /// Owner ID
    pub owner_id: String,
    /// Members
    pub members: Vec<FamilyMember>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl FamilyGroup {
    /// Create a new family group.
    pub fn new(owner_id: String, name: String) -> Self {
        FamilyGroup {
            id: Uuid::new_v4(),
            name,
            owner_id,
            members: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

/// Family member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMember {
    /// User ID
    pub user_id: String,
    /// Role
    pub role: FamilyRole,
    /// Joined timestamp
    pub joined_at: DateTime<Utc>,
}

/// Family role.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FamilyRole {
    /// Owner
    Owner,
    /// Admin
    Admin,
    /// Member
    Member,
}

/// Enterprise team for Aetheris.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseTeam {
    /// Team ID
    pub id: Uuid,
    /// Team name
    pub name: String,
    /// Owner ID
    pub owner_id: String,
    /// Members
    pub members: Vec<EnterpriseMember>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl EnterpriseTeam {
    /// Create a new enterprise team.
    pub fn new(owner_id: String, name: String) -> Self {
        EnterpriseTeam {
            id: Uuid::new_v4(),
            name,
            owner_id,
            members: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

/// Enterprise member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseMember {
    /// User ID
    pub user_id: String,
    /// Role
    pub role: EnterpriseRole,
    /// Joined timestamp
    pub joined_at: DateTime<Utc>,
}

/// Enterprise role.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnterpriseRole {
    /// Owner
    Owner,
    /// Admin
    Admin,
    /// Member
    Member,
}

/// Enterprise manager for Aetheris.
pub struct EnterpriseManager {
    vault_store: VaultStore,
    plan_manager: PlanManager,
}

impl EnterpriseManager {
    /// Create a new enterprise manager.
    pub fn new(vault_store: VaultStore) -> Self {
        EnterpriseManager {
            vault_store,
            plan_manager: PlanManager::new(vault_store),
        }
    }

    /// Create an enterprise team.
    pub fn create_team(&self, owner_id: String, name: String) -> Result<EnterpriseTeam> {
        let team = EnterpriseTeam::new(owner_id, name);
        self.vault_store.insert_session(team.clone())?;
        Ok(team)
    }

    /// Add a member to an enterprise team.
    pub fn add_member(&self, team_id: Uuid, user_id: String, role: EnterpriseRole) -> Result<()> {
        let team = self.get_team(team_id)?;
        
        if let Some(mut team) = team {
            team.members.push(EnterpriseMember {
                user_id,
                role,
                joined_at: Utc::now(),
            });
            self.vault_store.update_session(team_id, team)?;
        }
        
        Ok(())
    }

    /// Remove a member from an enterprise team.
    pub fn remove_member(&self, team_id: Uuid, user_id: String) -> Result<()> {
        let team = self.get_team(team_id)?;
        
        if let Some(mut team) = team {
            team.members.retain(|m| m.user_id != user_id);
            self.vault_store.update_session(team_id, team)?;
        }
        
        Ok(())
    }

    /// Get an enterprise team.
    pub fn get_team(&self, team_id: Uuid) -> Result<Option<EnterpriseTeam>> {
        let teams: Vec<EnterpriseTeam> = self.vault_store.list_items::<EnterpriseTeam>()?;
        for team in teams {
            if team.id == team_id {
                return Ok(Some(team));
            }
        }
        Ok(None)
    }

    /// List all enterprise teams.
    pub fn list_teams(&self) -> Result<Vec<EnterpriseTeam>> {
        self.vault_store.list_items::<EnterpriseTeam>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_plan_creation() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_plans")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let plan_manager = PlanManager::new(vault_store);
        
        let plan = plan_manager.create_plan("user123".to_string(), PlanType::Premium, BillingCycle::Monthly, 1, 5)?;
        assert_eq!(plan.plan_type, PlanType::Premium);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_plan_upgrade() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_plans_upgrade")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let plan_manager = PlanManager::new(vault_store);
        
        let plan = plan_manager.create_plan("user123".to_string(), PlanType::Free, BillingCycle::Monthly, 1, 1)?;
        let upgraded = plan_manager.upgrade_plan("user123".to_string(), PlanType::Premium)?;
        assert_eq!(upgraded.plan_type, PlanType::Premium);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_family_creation() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_family")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let family_manager = FamilyManager::new(vault_store);
        
        let group = family_manager.create_family("user123".to_string(), "My Family".to_string())?;
        assert_eq!(group.name, "My Family");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_enterprise_creation() -> Result<()> {
        let vault_store = VaultStore::new(".vault_test_enterprise")?;
        let crypto_engine = CryptoEngine::new()?;
        let master_key = crypto_engine.generate_key()?;
        vault_store.initialize(master_key)?;
        
        let enterprise_manager = EnterpriseManager::new(vault_store);
        
        let team = enterprise_manager.create_team("user123".to_string(), "My Team".to_string())?;
        assert_eq!(team.name, "My Team");
        
        Ok(())
    }
}