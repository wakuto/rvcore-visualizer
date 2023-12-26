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

struct Bus {
    signal: Vector,
}

impl Bus {
    fn new() -> Self {
        Self {
            signal: Vec::new().into(),
        }
    }
    
    fn new_with_value(signal: Vec<Value>) -> Self {
        Self {
            signal: signal.into(),
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


fn read_clocked_vcd(path: PathBuf) -> io::Result<Vec<Regfiles>> {
    let val = [Value::V0, Value::V1, Value::X, Value::Z];
    let x: u32 = (&Bus::new_with_value(val.into())).into();

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

    let mut commit_map_val = (0..32)
        .map(|_| Bus::new())
        .collect::<Vec<Bus>>();
    let mut rename_map_val = (0..32)
        .map(|_| Bus::new())
        .collect::<Vec<Bus>>();
    let mut physical_regfile_val = (0..64)
        .map(|_| Bus::new())
        .collect::<Vec<Bus>>();
    let mut clock_val = Value::X;
    
    let mut signal = Vec::new();
    
    for command_result in parser {
        let command = command_result?;
        use vcd::Command::*;
        match command {
            ChangeScalar(i, v) if i == clock => {
                // posedge
                if clock_val == Value::V0 && v == Value::V1 {
                    signal.push(Regfiles {
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
                    });
                }
                clock_val = v;
            },
            ChangeVector(i, v) => {
                if let Some(commit_map_reg_num) = commit_map_table.iter().position(|&id| id == i) {
                    commit_map_val[commit_map_reg_num].signal = v;
                } else if let Some(rename_map_reg_num) = rename_map_table.iter().position(|&id| id == i) {
                    rename_map_val[rename_map_reg_num].signal = v;
                } else if let Some(physical_regfile_reg_num) = physical_regfile.iter().position(|&id| id == i) {
                    physical_regfile_val[physical_regfile_reg_num].signal = v;
                }
            }
            _ => (),
        }
    }

    
    Ok(signal)
}