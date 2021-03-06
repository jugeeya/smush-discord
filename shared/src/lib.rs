#![feature(const_fn)]
#![feature(const_fn_union)]
use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};
use serde::{Serialize, Deserialize};

mod atomic_f32;
pub use atomic_f32::AtomicF32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub remaining_frames: AtomicU32,
    pub is_match: AtomicBool,
    pub stage: AtomicU32,
    pub players: [Player; 8]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub character: AtomicU32,
    pub stocks: AtomicU32,
    pub damage: AtomicF32,
    pub is_cpu: AtomicBool
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum Character {
    None = 0,
    Bayonetta,	
    Brave,	
    Buddy,	
    Captain,	
    Chrom,	
    Cloud,	
    Daisy,	
    Dedede,	
    Diddy,	
    Dolly,	
    Donkey,	
    Duckhunt,	
    Falco,	
    Fox,	
    Fushigisou,	
    Gamewatch,	
    Ganon,	
    Gaogaen,	
    Gekkouga,	
    Ike,	
    Inkling,	
    Jack,	
    Kamui,	
    Ken,	
    Kirby,	
    Koopa,	
    Koopag,	
    Koopajr,	
    Krool,	
    Link,	
    Littlemac,	
    Lizardon,	
    Lucario,	
    Lucas,	
    Lucina,	
    Luigi,	
    Mario,	
    Mariod,	
    Marth,	
    Master,	
    Metaknight,	
    Mewtwo,	
    Miienemyf,	
    Miienemyg,	
    Miienemys,	
    Miifighter,	
    Miigunner,	
    Miiswordsman,	
    Murabito,	
    Nana,	
    Ness,	
    Packun,	
    Pacman,	
    Palutena,	
    Peach,	
    Pfushigisou,	
    Pichu,	
    Pikachu,	
    Pikmin,	
    Pit,	
    Pitb,	
    Plizardon,	
    Popo,	
    Purin,	
    Pzenigame,	
    Reflet,	
    Richter,	
    Ridley,	
    Robot,	
    Rockman,	
    Rosetta,	
    Roy,	
    Ryu,	
    Samus,	
    Samusd,	
    Sheik,	
    Shizue,	
    Shulk,	
    Simon,	
    Snake,	
    Sonic,	
    Szerosuit,	
    Toonlink,	
    Wario,	
    Wiifit,	
    Wolf,	
    Yoshi,	
    Younglink,	
    Zelda,	
    Zenigame,
    Max,
}

// see `Character` for how this should be used
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum Stage {
    None = 0,
    Animal_City,
    Animal_Island,
    Animal_Village,
    BalloonFight,
    BattleField,
    BattleField_L,
    Battle_75m,
    Battle_Animal_City,
    Battle_Animal_Island,
    Battle_Animal_Village,
    Battle_BalloonFight,
    Battle_Bayo_Clock,
    Battle_Brave_Altar,
    Battle_Buddy_Spiral,
    Battle_DK_Jungle,
    Battle_DK_Lodge,
    Battle_DK_WaterFall,
    Battle_Dolly_Stadium,
    Battle_Dracula_Castle,
    Battle_DuckHunt,
    Battle_End,
    Battle_FE_Arena,
    Battle_FE_Colloseum,
    Battle_FE_Shrine,
    Battle_FE_Siege,
    Battle_FF_Midgar,
    Battle_FlatZoneX,
    Battle_Fox_Corneria,
    Battle_Fox_LylatCruise,
    Battle_Fox_Venom,
    Battle_Fzero_Bigblue,
    Battle_Fzero_Mutecity3DS,
    Battle_Fzero_Porttown,
    Battle_Icarus_Angeland,
    Battle_Icarus_SkyWorld,
    Battle_Icarus_Uprising,
    Battle_Ice_Top,
    Battle_Jack_Mementoes,
    Battle_Kart_CircuitFor,
    Battle_Kart_CircuitX,
    Battle_Kirby_Cave,
    Battle_Kirby_Fountain,
    Battle_Kirby_Gameboy,
    Battle_Kirby_Greens,
    Battle_Kirby_Halberd,
    Battle_Kirby_Pupupu64,
    Battle_LuigiMansion,
    Battle_MG_Shadowmoses,
    Battle_MarioBros,
    Battle_Mario_3DLand,
    Battle_Mario_Castle64,
    Battle_Mario_CastleDx,
    Battle_Mario_Dolpic,
    Battle_Mario_Galaxy,
    Battle_Mario_Maker,
    Battle_Mario_NewBros2,
    Battle_Mario_Odyssey,
    Battle_Mario_Paper,
    Battle_Mario_Past64,
    Battle_Mario_PastUsa,
    Battle_Mario_PastX,
    Battle_Mario_Rainbow,
    Battle_Mario_Uworld,
    Battle_Metroid_Kraid,
    Battle_Metroid_Norfair,
    Battle_Metroid_Orpheon,
    Battle_Metroid_ZebesDx,
    Battle_Mother_Fourside,
    Battle_Mother_Magicant,
    Battle_Mother_Newpork,
    Battle_Mother_Onett,
    Battle_Nintendogs,
    Battle_Pac_Land,
    Battle_Pictochat2,
    Battle_Pikmin_Garden,
    Battle_Pikmin_Planet,
    Battle_Pilotwings,
    Battle_Plankton,
    Battle_Poke_Kalos,
    Battle_Poke_Stadium,
    Battle_Poke_Stadium2,
    Battle_Poke_Tengam,
    Battle_Poke_Tower,
    Battle_Poke_Unova,
    Battle_Poke_Yamabuki,
    Battle_PunchOutSB,
    Battle_PunchOutW,
    Battle_Rock_Wily,
    Battle_SF_Suzaku,
    Battle_Sonic_Greenhill,
    Battle_Sonic_Windyhill,
    Battle_Spla_Parking,
    Battle_StreetPass,
    Battle_Tomodachi,
    Battle_Wario_Gamer,
    Battle_Wario_Madein,
    Battle_WiiFit,
    Battle_WreckingCrew,
    Battle_WufuIsland,
    Battle_Xeno_Gaur,
    Battle_Yoshi_CartBoard,
    Battle_Yoshi_Island,
    Battle_Yoshi_Story,
    Battle_Yoshi_Yoster,
    Battle_Zelda_Gerudo,
    Battle_Zelda_Greatbay,
    Battle_Zelda_Hyrule,
    Battle_Zelda_Oldin,
    Battle_Zelda_Pirates,
    Battle_Zelda_Skyward,
    Battle_Zelda_Temple,
    Battle_Zelda_Tower,
    Battle_Zelda_Train,
    Bayo_Clock,
    BonusGame,
    BossStage_Dracula,
    BossStage_Final1,
    BossStage_Final2,
    BossStage_Final3,
    BossStage_Galleom,
    BossStage_GanonBoss,
    BossStage_Marx,
    BossStage_Rathalos,
    Brave_Altar,
    Buddy_Spiral,
    CampaignMap,
    DK_Jungle,
    DK_Lodge,
    DK_WaterFall,
    Dolly_Stadium,
    Dracula_Castle,
    DuckHunt,
    End,
    End_75m,
    End_Animal_City,
    End_Animal_Island,
    End_Animal_Village,
    End_BalloonFight,
    End_BattleField,
    End_Bayo_Clock,
    End_Brave_Altar,
    End_Buddy_Spiral,
    End_DK_Jungle,
    End_DK_Lodge,
    End_DK_WaterFall,
    End_Dolly_Stadium,
    End_Dracula_Castle,
    End_DuckHunt,
    End_FE_Arena,
    End_FE_Colloseum,
    End_FE_Shrine,
    End_FE_Siege,
    End_FF_Midgar,
    End_FlatZoneX,
    End_Fox_Corneria,
    End_Fox_LylatCruise,
    End_Fox_Venom,
    End_Fzero_Bigblue,
    End_Fzero_Mutecity3DS,
    End_Fzero_Porttown,
    End_Icarus_Angeland,
    End_Icarus_SkyWorld,
    End_Icarus_Uprising,
    End_Ice_Top,
    End_Jack_Mementoes,
    End_Kart_CircuitFor,
    End_Kart_CircuitX,
    End_Kirby_Cave,
    End_Kirby_Fountain,
    End_Kirby_Gameboy,
    End_Kirby_Greens,
    End_Kirby_Halberd,
    End_Kirby_Pupupu64,
    End_LuigiMansion,
    End_MG_Shadowmoses,
    End_MarioBros,
    End_Mario_3DLand,
    End_Mario_Castle64,
    End_Mario_CastleDx,
    End_Mario_Dolpic,
    End_Mario_Galaxy,
    End_Mario_Maker,
    End_Mario_NewBros2,
    End_Mario_Odyssey,
    End_Mario_Paper,
    End_Mario_Past64,
    End_Mario_PastUsa,
    End_Mario_PastX,
    End_Mario_Rainbow,
    End_Mario_Uworld,
    End_Metroid_Kraid,
    End_Metroid_Norfair,
    End_Metroid_Orpheon,
    End_Metroid_ZebesDx,
    End_Mother_Fourside,
    End_Mother_Magicant,
    End_Mother_Newpork,
    End_Mother_Onett,
    End_Nintendogs,
    End_Pac_Land,
    End_Pictochat2,
    End_Pikmin_Garden,
    End_Pikmin_Planet,
    End_Pilotwings,
    End_Plankton,
    End_Poke_Kalos,
    End_Poke_Stadium,
    End_Poke_Stadium2,
    End_Poke_Tengam,
    End_Poke_Tower,
    End_Poke_Unova,
    End_Poke_Yamabuki,
    End_PunchOutSB,
    End_PunchOutW,
    End_Rock_Wily,
    End_SF_Suzaku,
    End_Sonic_Greenhill,
    End_Sonic_Windyhill,
    End_Spla_Parking,
    End_StreetPass,
    End_Tomodachi,
    End_Wario_Gamer,
    End_Wario_Madein,
    End_WiiFit,
    End_WreckingCrew,
    End_WufuIsland,
    End_Xeno_Gaur,
    End_Yoshi_CartBoard,
    End_Yoshi_Island,
    End_Yoshi_Story,
    End_Yoshi_Yoster,
    End_Zelda_Gerudo,
    End_Zelda_Greatbay,
    End_Zelda_Hyrule,
    End_Zelda_Oldin,
    End_Zelda_Pirates,
    End_Zelda_Skyward,
    End_Zelda_Temple,
    End_Zelda_Tower,
    End_Zelda_Train,
    FE_Arena,
    FE_Colloseum,
    FE_Shrine,
    FE_Siege,
    FF_Midgar,
    FlatZoneX,
    Fox_Corneria,
    Fox_LylatCruise,
    Fox_Venom,
    Fzero_Bigblue,
    Fzero_Mutecity3DS,
    Fzero_Porttown,
    HomerunContest,
    Icarus_Angeland,
    Icarus_SkyWorld,
    Icarus_Uprising,
    Ice_Top,
    Invalid,
    Jack_Mementoes,
    Kart_CircuitFor,
    Kart_CircuitX,
    Kirby_Cave,
    Kirby_Fountain,
    Kirby_Gameboy,
    Kirby_Greens,
    Kirby_Halberd,
    Kirby_Pupupu64,
    LuigiMansion,
    MG_Shadowmoses,
    MarioBros,
    Mario_3DLand,
    Mario_Castle64,
    Mario_CastleDx,
    Mario_Dolpic,
    Mario_Galaxy,
    Mario_Maker,
    Mario_NewBros2,
    Mario_Odyssey,
    Mario_Paper,
    Mario_Past64,
    Mario_PastUsa,
    Mario_PastX,
    Mario_Rainbow,
    Mario_Uworld,
    Metroid_Kraid,
    Metroid_Norfair,
    Metroid_Orpheon,
    Metroid_ZebesDx,
    Mother_Fourside,
    Mother_Magicant,
    Mother_Newpork,
    Mother_Onett,
    Nintendogs,
    Pac_Land,
    PhotoStage,
    Pictochat2,
    Pikmin_Garden,
    Pikmin_Planet,
    Pilotwings,
    Plankton,
    Poke_Kalos,
    Poke_Stadium,
    Poke_Stadium2,
    Poke_Tengam,
    Poke_Tower,
    Poke_Unova,
    Poke_Yamabuki,
    PunchOutSB,
    PunchOutW,
    ResultStage,
    ResultStage_Jack,
    Rock_Wily,
    SF_Suzaku,
    SP_Edit,
    SettingStage,
    ShamFight,
    Sonic_Greenhill,
    Sonic_Windyhill,
    SpiritsRoulette,
    Spla_Parking,
    Staffroll,
    StreetPass,
    TestStage,
    Tomodachi,
    Training,
    Wario_Gamer,
    Wario_Madein,
    WiiFit,
    WreckingCrew,
    WufuIsland,
    Xeno_Gaur,
    Yoshi_CartBoard,
    Yoshi_Island,
    Yoshi_Story,
    Yoshi_Yoster,
    Zelda_Gerudo,
    Zelda_Greatbay,
    Zelda_Hyrule,
    Zelda_Oldin,
    Zelda_Pirates,
    Zelda_Skyward,
    Zelda_Temple,
    Zelda_Tower,
    Zelda_Train,
    _75m,
    Max
}

impl Info {
    pub fn remaining_frames(&self) -> u32 {
        self.remaining_frames.load(Ordering::SeqCst)
    }

    pub fn is_match(&self) -> bool {
        self.is_match.load(Ordering::SeqCst)
    }

    pub fn stage(&self) -> Stage {
        let s = self.stage.load(Ordering::SeqCst);
        if (0..Stage::Max as u32).contains(&s) {
            unsafe {
                core::mem::transmute(s)
            }
        } else {
            Stage::None
        }
    }
}

impl Player {
    pub const fn new() -> Self {
        Self {
            character: AtomicU32::new(Character::None as u32),
            damage: AtomicF32::new(0.),
            stocks: AtomicU32::new(0),
            is_cpu: AtomicBool::new(false)
        }
    }

    pub fn character(&self) -> Character {
        let c = self.character.load(Ordering::SeqCst);
        if (0..Character::Max as u32).contains(&c) {
            unsafe {
                core::mem::transmute(c)
            }
        } else {
            Character::None
        }
    }

    pub fn damage(&self) -> f32 {
        self.damage.load(Ordering::SeqCst)
    }

    pub fn stocks(&self) -> u32 {
        self.stocks.load(Ordering::SeqCst)
    }

    pub fn is_cpu(&self) -> bool {
        self.is_cpu.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod shared_tests {
    use super::*;

    #[test]
    fn character_test() {
        // Test Character out of bounds
        let player = Player {
            character: AtomicU32::new(Character::Max as u32),
            ..Player::new()
        };

        // Invalid character
        assert_eq!(player.character(), Character::None);
        
        let player = Player {
            character: AtomicU32::new(u32::MAX),
            ..Player::new()
        };

        // Invalid character
        assert_eq!(player.character(), Character::None);
        
        let player = Player {
            character: AtomicU32::new(Character::Zenigame as u32),
            ..Player::new()
        };

        // Valid character
        assert_eq!(player.character(), Character::Zenigame);
    }

    #[test]
    fn stage_test() {
        const TEST_INFO: Info = Info {
            is_match: AtomicBool::new(true),
            remaining_frames: AtomicU32::new(3),
            stage: AtomicU32::new(Stage::Plankton as u32),
            players: [
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
            ]
        };

        assert_eq!(TEST_INFO.stage(), Stage::Plankton);
        assert_eq!(TEST_INFO.remaining_frames(), 3);
        assert!(TEST_INFO.is_match());

        let new_info = Info {
            stage: AtomicU32::new(Stage::Max as u32),
            ..TEST_INFO
        };

        // test invalid state
        assert_eq!(new_info.stage(), Stage::None);
        
        let new_info = Info {
            stage: AtomicU32::new(u32::MAX),
            ..TEST_INFO
        };

        // test invalid state
        assert_eq!(new_info.stage(), Stage::None);
        
        let new_info = Info {
            stage: AtomicU32::new(Stage::_75m as u32),
            ..TEST_INFO
        };

        assert_eq!(new_info.stage(), Stage::_75m);
    }
}
