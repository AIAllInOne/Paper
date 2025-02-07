use std::fmt;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use indexmap::IndexMap;

pub type BBId = usize;
pub type InstructionId = usize;
pub type RegisterId = usize;

pub type BBs = IndexMap<BBId, Block>;
pub type Instructions = IndexMap<InstructionId, Instruction>;

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub id: BBId,
    pub succ: Vec<BBId>,
    pub instructions: Vec<InstructionId>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InstructionKind {
    Load,
    FixPointAssign,
    FixPointCompare,
    Branch,
    BranchTrue,
    BranchFalse,
}

impl InstructionKind {
    pub fn to_string(&self) -> String {
        match self {
            InstructionKind::Load => "load".to_string(),
            InstructionKind::FixPointAssign => "assign".to_string(),
            InstructionKind::FixPointCompare => "compare".to_string(),
            InstructionKind::Branch => "branch".to_string(),
            InstructionKind::BranchTrue => "branchtrue".to_string(),
            InstructionKind::BranchFalse => "branchfalse".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Register {
    R(RegisterId),
    CR(RegisterId),
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::R(val) => write!(f, "r{}", val), // 自定义 R 类型的打印格式
            Register::CR(val) => write!(f, "cr{}", val), // 自定义 CR 类型的打印格式
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Target {
    Register(Register),
    BB(BBId),
}

impl fmt::Debug for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Register(register) => write!(f, "{:?}", register),
            Target::BB(bb) => write!(f, "BB {:?}", bb),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Instruction {
    pub id: InstructionId,
    pub kind: InstructionKind,
    pub target: Target,
    pub dep_register: Vec<Register>,
    pub bb_id: BBId,
}

impl Instruction {
    pub fn new(
        id: InstructionId,
        kind: InstructionKind,
        target: Target,
        dep_register: Vec<Register>,
        bb_id: BBId,
    ) -> Self {
        Self {
            id,
            kind,
            target,
            dep_register,
            bb_id,
        }
    }

    pub fn delay(&self, o: &Instruction) -> usize {
        match (&self.kind, &o.kind) {
            (InstructionKind::Load, InstructionKind::FixPointAssign) => match &self.target {
                Target::Register(register) => {
                    if o.dep_register.contains(register) {
                        1
                    } else {
                        0
                    }
                }
                Target::BB(_) => 0,
            },
            (InstructionKind::FixPointCompare, InstructionKind::BranchTrue) => match &self.target {
                Target::Register(register) => {
                    if o.dep_register.contains(register) {
                        3
                    } else {
                        0
                    }
                }
                Target::BB(_) => 0,
            },
            (InstructionKind::FixPointCompare, InstructionKind::BranchFalse) => {
                match &self.target {
                    Target::Register(register) => {
                        if o.dep_register.contains(register) {
                            3
                        } else {
                            0
                        }
                    }
                    Target::BB(_) => 0,
                }
            }
            _ => 0,
        }
    }
}

pub struct PDG {
    pub bbs: BBs,
    pub instructions: Instructions,
    pub bbid: BBId,
    pub instruction_id: InstructionId,
}

impl PDG {
    pub fn new() -> Self {
        Self {
            bbs: IndexMap::new(),
            instructions: IndexMap::new(),
            bbid: 0,
            instruction_id: 0,
        }
    }

    pub fn insert_block(&mut self) {
        self.bbid += 1;
        let bb = Block {
            id: self.bbid,
            succ: vec![],
            instructions: vec![],
        };
        self.bbs.insert(self.bbid, bb);
    }

    pub fn insert_idx_block(&mut self, id: usize) {
        let bb = Block {
            id,
            succ: vec![],
            instructions: vec![],
        };
        self.bbs.insert(id, bb);
    }

    pub fn insert_inst_to_bb(
        &mut self,
        bb_id: BBId,
        kind: InstructionKind,
        target: Target,
        dep_register: Vec<Register>,
    ) {
        self.instruction_id += 1;
        let inst = Instruction::new(self.instruction_id, kind, target, dep_register, bb_id);
        self.instructions.insert(self.instruction_id, inst);
        self.bbs
            .get_mut(&bb_id)
            .unwrap()
            .instructions
            .push(self.instruction_id);
    }

    pub fn insert_load(&mut self, bb_id: BBId, target_r: RegisterId, r: RegisterId) {
        self.insert_inst_to_bb(
            bb_id,
            InstructionKind::Load,
            Target::Register(Register::R(target_r)),
            vec![Register::R(r)],
        );
    }
    pub fn insert_comp(
        &mut self,
        bb_id: BBId,
        target_r: RegisterId,
        r1: RegisterId,
        r2: RegisterId,
    ) {
        self.insert_inst_to_bb(
            bb_id,
            InstructionKind::FixPointCompare,
            Target::Register(Register::CR(target_r)),
            vec![Register::R(r1), Register::R(r2)],
        );
    }
    pub fn insert_branch_true(&mut self, bb_id: BBId, target_id: BBId, cr: RegisterId) {
        self.insert_inst_to_bb(
            bb_id,
            InstructionKind::BranchTrue,
            Target::BB(target_id),
            vec![Register::CR(cr)],
        );
    }

    pub fn insert_branch_false(&mut self, bb_id: BBId, target_id: BBId, cr: RegisterId) {
        self.insert_inst_to_bb(
            bb_id,
            InstructionKind::BranchFalse,
            Target::BB(target_id),
            vec![Register::CR(cr)],
        );
    }

    pub fn insert_uncondition_branch(&mut self, bb_id: BBId, target_id: BBId) {
        self.insert_inst_to_bb(
            bb_id,
            InstructionKind::Branch,
            Target::BB(target_id),
            vec![],
        );
    }

    pub fn insert_assign(&mut self, bb_id: BBId, target_id: RegisterId, r: RegisterId) {
        self.insert_inst_to_bb(
            bb_id,
            InstructionKind::FixPointAssign,
            Target::Register(Register::R(target_id)),
            vec![Register::R(r)],
        );
    }

    fn get_succ(&mut self) {
        let bb_num = self.bbs.len();
        for (id, bb) in &mut self.bbs {
            let mut uncondition_branch = false;
            for inst_id in bb.instructions.iter() {
                let inst = self.instructions.get(inst_id).unwrap();
                match &inst.kind {
                    InstructionKind::BranchTrue | InstructionKind::BranchFalse => {
                        match &inst.target {
                            Target::Register(_) => panic!(),
                            Target::BB(succ_id) => {
                                bb.succ.push(*succ_id);
                                if inst.dep_register.is_empty() {
                                    uncondition_branch = true;
                                    break;
                                }
                            }
                        }
                    }
                    InstructionKind::Branch => match &inst.target {
                        Target::Register(_) => panic!(),
                        Target::BB(succ_id) => {
                            bb.succ.push(*succ_id);
                            if inst.dep_register.is_empty() {
                                uncondition_branch = true;
                                break;
                            }
                        }
                    },
                    _ => {}
                }
            }
            if *id != bb_num && !uncondition_branch {
                bb.succ.insert(0, id + 1);
            }
        }
    }

    pub fn cfg(&mut self) {
        self.get_succ();
        let entry = Block {
            id: 0,
            succ: vec![1],
            instructions: vec![],
        };
        self.bbs.insert(0, entry);
        let exit_id = self.bbid + 1;
        let exit = Block {
            id: exit_id,
            succ: vec![],
            instructions: vec![],
        };
        self.bbs.insert(exit_id, exit);

        self.bbs.get_mut(&self.bbid).unwrap().succ.push(exit_id);
    }

    pub fn test_data() -> Self {
        let mut pdg = PDG::new();
        // BB1
        pdg.insert_block();
        pdg.insert_load(pdg.bbid, 12, 31);
        pdg.insert_load(pdg.bbid, 0, 31);
        pdg.insert_comp(pdg.bbid, 7, 12, 0);
        pdg.insert_branch_false(pdg.bbid, 6, 7);
        // BB2
        pdg.insert_block();
        pdg.insert_comp(pdg.bbid, 6, 12, 30);
        pdg.insert_branch_false(pdg.bbid, 4, 6);
        // BB3
        pdg.insert_block();
        pdg.insert_assign(pdg.bbid, 30, 12);
        // BB4
        pdg.insert_block();
        pdg.insert_comp(pdg.bbid, 7, 0, 28);
        pdg.insert_branch_false(pdg.bbid, 10, 7);
        // BB5
        pdg.insert_block();
        pdg.insert_assign(pdg.bbid, 28, 0);
        pdg.insert_uncondition_branch(pdg.bbid, 10);
        // BB6
        pdg.insert_block();
        pdg.insert_comp(pdg.bbid, 6, 0, 30);
        pdg.insert_branch_false(pdg.bbid, 8, 6);
        // BB7
        pdg.insert_block();
        pdg.insert_assign(pdg.bbid, 30, 0);
        // BB8
        pdg.insert_block();
        pdg.insert_comp(pdg.bbid, 7, 12, 28);
        pdg.insert_branch_false(pdg.bbid, 10, 7);
        // BB9
        pdg.insert_block();
        pdg.insert_assign(pdg.bbid, 28, 12);
        // BB10
        pdg.insert_block();
        pdg.insert_assign(pdg.bbid, 29, 29);
        pdg.insert_comp(pdg.bbid, 4, 29, 27);
        pdg.insert_branch_true(pdg.bbid, 1, 4);

        pdg.cfg();
        pdg
    }

    pub fn print(&self) {
        for (id, bb) in &self.bbs {
            println!("BB {}: succ: {:?}", id, bb.succ);
            for inst_id in bb.instructions.iter() {
                let inst = self.instructions.get(inst_id).unwrap();
                print!("(I{}):", inst_id);
                match inst.kind {
                    InstructionKind::Load => {
                        print!("{:?} = load({:?})", inst.target, inst.dep_register[0],)
                    }
                    InstructionKind::FixPointAssign => {
                        print!("{:?} = {:?}", inst.target, inst.dep_register[0],)
                    }
                    InstructionKind::FixPointCompare => {
                        print!(
                            "{:?} = compare({:?}, {:?})",
                            inst.target, inst.dep_register[0], inst.dep_register[1]
                        )
                    }
                    InstructionKind::BranchTrue => {
                        print!(
                            "BT, target: {:?}, cond: {:?}",
                            inst.target, inst.dep_register[0]
                        );
                    }

                    InstructionKind::BranchFalse => {
                        print!(
                            "BF target: {:?}, cond: {:?}",
                            inst.target, inst.dep_register[0]
                        );
                    }
                    InstructionKind::Branch => {
                        print!("B target: {:?}", inst.target);
                    }
                }
                println!()
            }
            println!("---------- End BB{id}");
        }
    }
}
