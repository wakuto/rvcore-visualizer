use std::fs::File;
use std::io::{self, BufReader};
use std::io::ErrorKind::InvalidInput;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use vcd::{Value, Vector};
use tauri::api::dialog::FileDialogBuilder;

#[tauri::command]
pub fn open_vcd_file(window: tauri::Window) {
    FileDialogBuilder::new()
        .set_title("Open VCD File")
        .add_filter("VCD Files", &["vcd"])
        .pick_file(move |path_opt| match path_opt {
            Some(path) => {
                let sig = read_clocked_vcd(path).unwrap();
                window.emit::<String>("vcd_file_selected", serde_json::to_string(&sig).unwrap()).unwrap();
            },
            _ => (),
        });
 }

#[derive(Debug, Serialize, Deserialize)]
struct Regfiles {
    commit_map_table: [u32; 32],
    rename_map_table: [u32; 32],
    physical_regfile: Vec::<u32>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct RobEntry {
    entry_valid: bool,
    phys_rd: u32,
    arch_rd: u32,
    commit_ready: bool,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct IsqEntry {
    entry_valid: bool,
    alu_cmd: u32,
    op1_valid: bool,
    op1_data: u32,
    op2_valid: bool,
    op2_type: u32,
    op2_data: u32,
    phys_rd: u32,
    bank_addr: u32,
    rob_addr: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct StepSignal {
    regfiles: Regfiles,
    rob: Vec<Vec<RobEntry>>,
    isq: Vec<IsqEntry>,
}

struct Bus {
    signal: Vector,
}

impl Bus {
    fn new() -> Self {
        Self {
            signal: Vec::new().into(),
        }
    }
    
    fn new_with_value(signal: &Vec<Value>) -> Self {
        Self {
            signal: signal.clone().into(),
        }
    }
}

impl From<&Bus> for u32 {
    fn from(bus: &Bus) -> Self {
        let mut ret = 0;
        for (i, v) in bus.signal.iter().enumerate() {
            ret |= match v { Value::V1 => 1 << (bus.signal.len() - i - 1), _ => 0 };
        }
        ret
    }
}


fn read_clocked_vcd(path: PathBuf) -> io::Result<Vec<StepSignal>> {
    let f = File::open(path)?;
    let mut parser = vcd::Parser::new(BufReader::new(f));

    let header = parser.parse_header()?;
    let clock = header.find_var(&["TOP", "clk"])
        .ok_or_else(|| io::Error::new(InvalidInput, "no wire top.clock"))?.code;
    let mut commit_map_table = Vec::new();
    for i in 0..32 {
        let sig = header.find_var(&["TOP", "core", "commit_map_table", &format!("regfile[{}]", i)]).unwrap().code;
        commit_map_table.push(sig);
    }
    let mut rename_map_table = Vec::new();
    for i in 0..32 {
        let sig = header.find_var(&["TOP", "core", "rename_map_table", &format!("regfile[{}]", i)]).unwrap().code;
        rename_map_table.push(sig);
    }
    let mut physical_regfile = Vec::new();
    for i in 0..64 {
        let sig = header.find_var(&["TOP", "core", "phys_regfile", &format!("regfile[{}]", i)]).unwrap().code;
        physical_regfile.push(sig);
    }
    
    let mut rob = Vec::new();
    for i in 0..16 {
        let mut row = Vec::new();
        for bank in 0..2 {
            let entry = header.find_scope(&["TOP", "core", "rob", &format!("rob_entry[{}][{}]", i, bank)]).unwrap();
            let entry_valid = entry.find_var("entry_valid").unwrap().code;
            let arch_rd = entry.find_var("arch_rd").unwrap().code;
            let phys_rd = entry.find_var("phys_rd").unwrap().code;
            let commit_ready = entry.find_var("commit_ready").unwrap().code;
            
            row.push(vec![entry_valid, arch_rd, phys_rd, commit_ready]);
        }
        rob.push(row);
    }
    
    let mut isq = Vec::new();
    for i in 0..32 {
        let entry = header.find_scope(&["TOP", "core", "issue_queue", &format!("issue_queue[{}]", i)]).unwrap();

        let entry_valid = entry.find_var("entry_valid").unwrap().code;
        let alu_cmd = entry.find_var("alu_cmd").unwrap().code;
        let op1_valid = entry.find_var("op1_valid").unwrap().code;
        let op1_data = entry.find_var("op1_data").unwrap().code;
        let op2_valid = entry.find_var("op2_valid").unwrap().code;
        let op2_type = entry.find_var("op2_type").unwrap().code;
        let op2_data = entry.find_var("op2_data").unwrap().code;
        let phys_rd = entry.find_var("phys_rd").unwrap().code;
        let bank_addr = entry.find_var("bank_addr").unwrap().code;
        let rob_addr = entry.find_var("rob_addr").unwrap().code;
        
        isq.push(vec![entry_valid, alu_cmd, op1_valid, op1_data, op2_valid, op2_type, op2_data, phys_rd, bank_addr, rob_addr]);
    }

    let mut clock_val = Value::X;

    // Regfile 作成時に u32 に変換する
    let mut commit_map_val = (0..32)
        .map(|_| Bus::new())
        .collect::<Vec<Bus>>();
    let mut rename_map_val = (0..32)
        .map(|_| Bus::new())
        .collect::<Vec<Bus>>();
    let mut physical_regfile_val = (0..64)
        .map(|_| Bus::new())
        .collect::<Vec<Bus>>();

    // 代入時に RobEntry に変換する
    let mut rob_val = (0..16)
        .map(|_| (0..2)
             .map(|_| RobEntry {
                 entry_valid: false,
                 phys_rd: 0,
                 arch_rd: 0,
                 commit_ready: false,
             })
            .collect::<Vec<RobEntry>>())
        .collect::<Vec<Vec<RobEntry>>>();
    let mut isq_val = (0..32)
        .map(|_| IsqEntry {
            entry_valid: false,
            alu_cmd: 0,
            op1_valid: false,
            op1_data: 0,
            op2_valid: false,
            op2_type: 0,
            op2_data: 0,
            phys_rd: 0,
            bank_addr: 0,
            rob_addr: 0,
        })
        .collect::<Vec<IsqEntry>>();
    
    let mut signal = Vec::new();
    
    for command_result in parser {
        let command = command_result?;
        use vcd::Command::*;
        match command {
            ChangeScalar(i, v) if i == clock => {
                // posedge
                if clock_val == Value::V0 && v == Value::V1 {
                    signal.push(StepSignal {
                        regfiles: Regfiles {
                            commit_map_table: commit_map_val
                                .iter()
                                .map(u32::from)
                                .collect::<Vec<u32>>()
                                .try_into().unwrap(),
                            rename_map_table: rename_map_val
                                .iter()
                                .map(u32::from)
                                .collect::<Vec<u32>>()
                                .try_into().unwrap(),
                            physical_regfile: physical_regfile_val
                                .iter()
                                .map(u32::from)
                                .collect::<Vec<u32>>(),
                        },
                        rob: rob_val.clone(),
                        isq: isq_val.clone(),
                    });
                }
                clock_val = v;
            },
            ChangeVector(i, v) => {
                // commit_map_table
                if let Some(commit_map_reg_num) = commit_map_table.iter().position(|&id| id == i) {
                    commit_map_val[commit_map_reg_num].signal = v;
                // rename_map_table
                } else if let Some(rename_map_reg_num) = rename_map_table.iter().position(|&id| id == i) {
                    rename_map_val[rename_map_reg_num].signal = v;
                // physical_regfile
                } else if let Some(physical_regfile_reg_num) = physical_regfile.iter().position(|&id| id == i) {
                    physical_regfile_val[physical_regfile_reg_num].signal = v;
                // issue_queue
                } else if let Some(isq_entry_num) = isq.iter().position(|field| field.iter().any(|&id| id == i)) {
                    let isq_entry = isq_val.get_mut(isq_entry_num).unwrap();
                    let field_num = isq[isq_entry_num].iter().position(|&id| id == i).unwrap();
                    match field_num {
                        1 => isq_entry.alu_cmd = (&Bus::new_with_value(&v.into())).into(),
                        3 => isq_entry.op1_data = (&Bus::new_with_value(&v.into())).into(),
                        5 => isq_entry.op2_type = (&Bus::new_with_value(&v.into())).into(),
                        6 => isq_entry.op2_data = (&Bus::new_with_value(&v.into())).into(),
                        7 => isq_entry.phys_rd = (&Bus::new_with_value(&v.into())).into(),
                        8 => isq_entry.bank_addr = (&Bus::new_with_value(&v.into())).into(),
                        9 => isq_entry.rob_addr = (&Bus::new_with_value(&v.into())).into(),
                        _ => (),
                    }
                // rob
                } else {
                    rob.iter().enumerate().any(|(entry_num, row)| {
                        row.iter().enumerate().any(|(bank_num, field)| {
                            if let Some(field_num) = field.iter().position(|&id| id == i) {
                                match field_num {
                                    1 => rob_val[entry_num][bank_num].phys_rd = (&Bus::new_with_value(&v.clone().into())).into(),
                                    2 => rob_val[entry_num][bank_num].arch_rd = (&Bus::new_with_value(&v.clone().into())).into(),
                                    _ => (),
                                }
                                true
                            } else {
                                false
                            }
                        })
                    });
                }
            },
            ChangeScalar(i, v) => {
                // issue_queue
                if let Some(isq_entry_num) = isq.iter().position(|field| field.iter().any(|&id| id == i)) {
                    let isq_entry = isq_val.get_mut(isq_entry_num).unwrap();
                    let field_num = isq[isq_entry_num].iter().position(|&id| id == i).unwrap();
                    match field_num {
                        0 => isq_entry.entry_valid = v == Value::V1,
                        2 => isq_entry.op1_valid = v == Value::V1,
                        4 => isq_entry.op2_valid = v == Value::V1,
                        _ => (),
                    }
                // rob
                } else {
                    rob.iter().enumerate().any(|(entry_num, row)| {
                        row.iter().enumerate().any(|(bank_num, field)| {
                            if let Some(field_num) = field.iter().position(|&id| id == i) {
                                match field_num {
                                    0 => rob_val[entry_num][bank_num].entry_valid = v == Value::V1,
                                    3 => rob_val[entry_num][bank_num].commit_ready = v == Value::V1,
                                    _ => (),
                                }
                                true
                            } else {
                                false
                            }
                        })
                    });
                }
                
            },
            _ => (),
        }
    }

    
    Ok(signal)
}
