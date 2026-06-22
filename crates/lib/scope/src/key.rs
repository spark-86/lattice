use std::collections::HashMap;

use anyhow::Result;
use rhex::data::RhexData;

use crate::{Scope, member::Member};

impl Scope {
    /// # key_grant
    /// I really don't feel like this is should be attached to the
    /// scope. But the alternative is to give the transforms the
    /// ability to mod the access table and YIKES.
    ///
    pub fn parse_key_grant(&mut self, rhex_data: RhexData) -> Result<()> {
        let (groups, members) = match rhex_data {
            RhexData::Mixed { meta, binary } => {
                // Parse JSON
                let json_groups = meta["groups"].as_array().unwrap().clone();
                let mut build_groups = HashMap::new();
                let mut build_members = HashMap::new();
                let eff = meta["eff"].as_u64().unwrap();
                let exp = meta["exp"].as_u64().unwrap();
                let issued = meta["issued"].as_u64().unwrap();
                let tags = meta["tags"].as_array().unwrap().clone();
                let names = meta["names"].as_array().unwrap().clone();

                // die if we didn't provide a name for each key
                if names.len() > 0 && (names.len() != binary.len()) {
                    anyhow::bail!("Must have a name for each key or none.")
                }

                for i in 0..binary.len() {
                    let key: [u8; 32] = binary[i][..32].try_into().unwrap();
                    let mut member = Member::new(key);
                    member.eff = eff;
                    member.exp = exp;
                    member.issued = issued;
                    for tag in &tags {
                        member.tags.push(tag.as_str().unwrap().to_string());
                    }
                    if names.len() > 0 {
                        member.name = Some(names[i].as_str().unwrap().to_string());
                    }

                    build_members.insert(key, member);
                    for group in &json_groups {
                        let group_mapping = build_groups
                            .entry(group.as_str().unwrap().to_string())
                            .or_insert(Vec::new());
                        group_mapping.push(key);
                    }
                }
                (build_groups, build_members)
            }
            _ => anyhow::bail!("Wrong data type"),
        };
        self.groups = groups;
        self.members = members;
        Ok(())
    }

    /// # key_revoke
    /// Pretty straight forward. Revokes all the keys in `binary`.
    /// I don't really have a place to put `reason` but it's here.
    ///
    pub fn parse_key_revoke(&mut self, rhex_data: RhexData) -> Result<()> {
        match rhex_data {
            RhexData::Mixed { meta, binary } => {
                // I dunno... maybe return a Rhex with the reason?
                let _reason = meta["reason"].as_str();
                // Extract groups from JSON
                let groups_json = meta["groups"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("groups is not an array"))?;

                for key_data in binary {
                    let key: [u8; 32] = key_data[..32]
                        .try_into()
                        .map_err(|_| anyhow::anyhow!("Invalid key length"))?;

                    if groups_json.first().and_then(|v| v.as_str()) == Some("*") {
                        // Global removal from members
                        self.members.remove(&key);

                        // Remove key from ALL groups
                        for members in self.groups.values_mut() {
                            members.retain(|k| *k != key);
                        }
                    } else {
                        // Specific group removal
                        for group_val in groups_json {
                            if let Some(group_name) = group_val.as_str() {
                                if let Some(members) = self.groups.get_mut(group_name) {
                                    members.retain(|k| *k != key);
                                }
                            }
                        }
                    }
                }

                // Cleanup: Remove any groups that are now empty
                self.groups.retain(|_name, members| !members.is_empty());
            }
            _ => anyhow::bail!("Wrong data type"),
        }
        Ok(())
    }
}
