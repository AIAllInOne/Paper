use indexmap::IndexMap;

use crate::pdg::{BBId, PDG};

pub type CSPDGBBs = IndexMap<BBId, CSPDGBlock>;

#[derive(Debug, PartialEq, Eq)]
pub struct CSPDGBlock {
    pub id: BBId,
    pub true_succ: Vec<BBId>,
    pub false_succ: Vec<BBId>,
    pub equivalent_nodes: Vec<BBId>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct CSPDG {
    pub bbs: CSPDGBBs,
}

impl CSPDG {
    pub fn print(&self) {
        println!("-----------------------------------");
        println!("CSPDG");
        for (id, bb) in &self.bbs {
            println!("BB {}", id);
            println!("TrueSucc: {:?}", bb.true_succ);
            println!("FalseSucc: {:?}", bb.false_succ);
            println!("Equivalent Nodes{:?}", bb.equivalent_nodes);
        }
    }

    fn equivalent_nodes(&mut self) {
        for (id, bb) in &self.bbs {
            let true_succ = bb.true_succ.clone();
            for b in &true_succ {
                let eq_node: Vec<usize> =
                    true_succ.iter().filter(|id| *id != b).map(|i| *i).collect();

            }
        }
    }
}

impl PDG {
    pub fn cspdg(&self) -> CSPDG {
        let mut bbs = IndexMap::new();
        // for (bb_id, bb) in &self.bbs {
        //     if bb.instructions.is_empty(){
        //         continue;
        //     }
        //     let instr_id = bb.instructions.last().unwrap();
        //     let instr = self.instructions.get(instr_id).unwrap();
        //     match instr.kind {
        //         crate::pdg::InstructionKind::BranchTrue => match &instr.target {
        //             crate::pdg::Target::Register(_) => panic!(),
        //             crate::pdg::Target::BB(target) => {
        //                 let succ: Vec<usize> = bb.succ.clone();
        //                 let false_succ: Vec<usize> =
        //                     succ.iter().filter(|i| *i != target).map(|i| *i).collect();
        //                 bbs.insert(
        //                     *bb_id,
        //                     CSPDGBlock {
        //                         id: *bb_id,
        //                         TrueSucc: vec![*target],
        //                         FalseSucc: false_succ,
        //                         EquivalentNodes: vec![],
        //                     },
        //                 );
        //             }
        //         },
        //         crate::pdg::InstructionKind::BranchFalse => match &instr.target {
        //             crate::pdg::Target::Register(_) => panic!(),
        //             crate::pdg::Target::BB(target) => {
        //                 let succ: Vec<usize> = bb.succ.clone();
        //                 let true_succ: Vec<usize> =
        //                     succ.iter().filter(|i| *i != target).map(|i| *i).collect();
        //                 bbs.insert(
        //                     *bb_id,
        //                     CSPDGBlock {
        //                         id: *bb_id,
        //                         TrueSucc: true_succ,
        //                         FalseSucc: vec![*target],
        //                         EquivalentNodes: vec![],
        //                     },
        //                 );
        //             }
        //         },
        //         _ => {}
        //     }
        // }

        let cspdg = CSPDG { bbs };

        cspdg
    }
}
