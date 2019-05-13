use std::collections::hash_map::Iter;
use std::collections::HashMap;

/// Represents a mapping between program labels, their
/// associated program counters, and the program instructions
/// containing jump targets
pub struct LabelMap {
    pc_map: HashMap<usize, usize>,
    inst_list: HashMap<usize, usize>,
}

impl LabelMap {
    /// Creates a new `LabelMap`
    pub fn new() -> Self {
        Self {
            pc_map: HashMap::new(),
            inst_list: HashMap::new(),
        }
    }

    /// Gets the corresponding program counter for a given label
    /// Returns `None` if the given label was never marked in the
    /// program
    pub fn get_pc(&self, label: usize) -> Option<usize> {
        self.pc_map.get(&label).cloned()
    }

    /// Iterates over the instruction list, yielding a tuple `(idx, label)`
    /// where `idx` is the index of the instruction in the current program
    /// and `label` is the label corresponding to that instruction
    pub fn iter_insts(&self) -> Iter<usize, usize> {
        self.inst_list.iter()
    }

    /// Adds a mapping from the index of a given jump instruction in the
    /// current program to its corresponding label that eventually needs
    /// to be resolved to a particular program counter
    pub fn add_inst(&mut self, idx: usize, label: usize) {
        self.inst_list.insert(idx, label);
    }

    /// Adds a mapping from a given label to its corresponding program counter
    pub fn add_label(&mut self, label: usize, pc: usize) {
        self.pc_map.insert(label, pc);
    }
}
