use std::cmp::min;

use crate::pdg::{BBId, InstructionId, PDG};

pub type BBPath = Vec<BBId>;

impl PDG {
    pub fn get_paths(&mut self) -> Vec<BBPath> {
        let succ = self.bbs.get(&10).unwrap().succ.clone();
        self.bbs.get_mut(&10).unwrap().succ.clear();
        let mut paths = vec![];
        self.dfs(&1, &mut vec![], &mut paths);
        self.bbs.get_mut(&10).unwrap().succ = succ;
        paths
    }

    fn dfs(&self, id: &BBId, path: &mut Vec<BBId>, paths: &mut Vec<BBPath>) {
        let bb = self.bbs.get(id).unwrap();
        let succ = &bb.succ;
        path.push(*id);
        if succ.is_empty() {
            paths.push(path.clone());
        } else {
            for id in succ {
                self.dfs(id, path, paths);
            }
        }
        path.pop();
    }

    pub fn print_path(&self, path: &BBPath) {
        let result: String = path
            .iter()
            .map(|&x| format!("BB{}", x))
            .collect::<Vec<String>>()
            .join(", ");

        println!("Path: [{:?}]", result);
    }

    // fn min_index_in_path(&self, id: &InstructionId, instrs: &Vec<InstructionId>) -> usize {
    //     // let instr = self.instructions.get(id).unwrap();
    //     // let mut min_idx = 0;
    //     // for (i, instr_id) in instrs.iter().enumerate() {
    //     //     let other_instr = self.instructions.get(instr_id).unwrap();
    //     //     if instr_id == &id {
    //     //         return min_idx;
    //     //     } else {
    //     //         let delay = instr.delay(other_instr);
    //     //         min_idx = min(min_idx, i + delay)
    //     //     }
    //     // }
    //     // return min_idx;
    // }

    // pub fn path_time_clock(&self, path: &BBPath) {
    //     let mut fix_point = vec![];
    //     let mut branch = vec![];

    //     let instrs: Vec<InstructionId> = path.into_iter().flat_map(|bb_id| self.bbs.get(bb_id).unwrap().instructions.clone()).collect();

    //     for instr_id in &instrs {
    //         let instr = self.instructions.get(instr_id).unwrap();
    //         // let min_index = self.min_index_in_path(instr_id, &instrs);
    //         match instr.kind {
    //             crate::pdg::InstructionKind::BranchTrue | crate::pdg::InstructionKind::BranchFalse => branch.push(instr_id.clone()),
    //             _ => {
    //                 fix_point.push(instr_id.clone());
    //                 branch.push(0);
    //             }
    //         }
    //     }

    //     for &value in &fix_point {
    //         let formatted_value = format!("{: >2}", value); // 如果值为1-9，则在前面加一个空格
    //         print!("{} ", formatted_value);
    //     }
    //     println!();

    //     for &value in &branch {
    //         let formatted_value = format!("{: >2}", value); // 如果值为1-9，则在前面加一个空格
    //         print!("{} ", formatted_value);
    //     }
    //     println!();
    // }
}
