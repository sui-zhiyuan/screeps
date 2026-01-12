// use crate::actor::CreepMemoryTrait;
// use crate::actor::creep_actor::CreepMemory;
// use anyhow::anyhow;
// use screeps::{
//     ConstructionSite, Creep, HasId, HasPosition, MoveToOptions, ObjectId, PolyStyle, ResourceType,
//     SharedCreepProperties, StructureSpawn, find,
// };
// use serde::{Deserialize, Serialize};
// 
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct CreepBuilderMemory {
//     spawn: ObjectId<StructureSpawn>,
//     target: Option<ObjectId<ConstructionSite>>,
// }
// 
// impl CreepBuilderMemory {
//     pub fn new_memory(spawn: &StructureSpawn) -> CreepMemory {
//         CreepMemory::Builder(CreepBuilderMemory {
//             spawn: spawn.id(),
//             target: None,
//         })
//     }
// }
// 
// impl CreepMemoryTrait for CreepBuilderMemory {
//     fn run(&mut self, creep: &Creep) -> anyhow::Result<()> {
//         if creep.spawning() {
//             return Ok(());
//         }
// 
//         if creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0 {
//             let spawn = &self.spawn.resolve().ok_or(anyhow!("spawn not found"))?;
//             if !creep.pos().is_near_to(spawn.pos()) {
//                 let line_style = PolyStyle::default().fill("#ffffff");
//                 creep.move_to_with_options(
//                     spawn,
//                     Some(MoveToOptions::new().visualize_path_style(line_style)),
//                 )?;
//                 creep.say("to spawn", true)?;
//                 return Ok(());
//             }
//             if creep.store().get_free_capacity(Some(ResourceType::Energy))
//                 > spawn.store().get_used_capacity(Some(ResourceType::Energy)) as i32
//             {
//                 creep.say("not enough energy", true)?;
//                 return Ok(());
//             }
// 
//             creep.withdraw(spawn, ResourceType::Energy, None)?;
//             creep.say("get energy", true)?;
//             Ok(())
//         } else {
//             let target = match &self.target {
//                 Some(target) => target.resolve().ok_or(anyhow!("target not found"))?,
//                 None => {
//                     let room = creep.room().ok_or(anyhow!("room not found"))?;
//                     let mut targets = room.find(find::CONSTRUCTION_SITES, None);
//                     if targets.is_empty() {
//                         creep.say("no target", true)?;
//                         return Ok(());
//                     }
//                     targets.swap_remove(0)
//                 }
//             };
// 
//             if !creep.pos().is_near_to(target.pos()) {
//                 let line_style = PolyStyle::default().fill("#ffffff");
//                 creep.move_to_with_options(
//                     target,
//                     Some(MoveToOptions::new().visualize_path_style(line_style)),
//                 )?;
//                 creep.say("to target", true)?;
//                 return Ok(());
//             }
// 
//             creep.build(&target)?;
//             creep.say("building", true)?;
//             Ok(())
//         }
//     }
// }
