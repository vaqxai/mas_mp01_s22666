use crate::soldier_extent::SoldierKey;

pub struct Squad {
    name: String,
    leader: SoldierKey,
    members: Vec<SoldierKey>,
}

impl Squad {
    pub fn new(name: String, leader: SoldierKey) -> Squad {
        Squad {
            name: name,
            leader: leader,
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: SoldierKey) {
        self.members.push(member);
    }

    pub fn remove_member(&mut self, member: SoldierKey) {
        self.members.retain(|&x| x != member);
    }

    pub fn get_members(&self) -> &Vec<SoldierKey> {
        &self.members
    }

    pub fn get_leader(&self) -> &SoldierKey {
        &self.leader
    }

    pub fn set_leader(&mut self, new_leader: SoldierKey) {

        // old leader becomes a member
        // maybe we should expell the old leader?
        self.members.push(self.leader);

        if self.members.contains(&new_leader) {
            // remove new leader from members if they are a member to prevent duplication
            self.members.retain(|&val| val != new_leader);
        }

        self.leader = new_leader;
    }

    // pochodny
    pub fn get_soldier_count(&self) -> usize {
        self.members.len() + 1
    }
}