pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        
        if self.health >= 1 {
            return None;
        }

        let mana = if self.mana.is_none() {
            None
        }else {
            Some(100)
        };

        Some(Player {
            health: 100,
            mana,
            level: self.level
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.level < 10 {
            self.health = if self.health > mana_cost {
                self.health - mana_cost
            } else {
                0
            };
            return 0;
        }

        let mana = self.mana.unwrap_or(0);
        if mana < mana_cost {
            return 0;
        }

        self.mana = Some(mana - mana_cost);
        mana_cost * 2
    }
}
