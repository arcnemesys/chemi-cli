use clap::Args;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Args,)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ElementsArgs {
    pub atomic_symbol: String,
    #[arg(short = 'n', long, alias("atmn"), help = "Include the atomic number of a given element.")]
    pub atomic_number: bool,
    #[arg(long, short = 'w', alias("atmw"), help = "Include the atomic weight of a given element.")]
    pub atomic_weight: bool,
    #[arg(long, short = 'l', alias("elvls"), help = "Include the energy levels of a given element.")]
    pub energy_levels: bool,
    #[arg(long, short = 'e', alias("eneg"), help = "Include the electronegativity of a given element.")]
    pub electronegativity: bool,
    #[arg(long, short, alias("ser"),  help = "Include the series of a given element.")]
    pub series: bool,
    #[arg(long, short = 'd', alias("ddisc"), help = "Include the discovery date of a given element.")]
    pub date_discovered: bool,
    #[arg(long, short = 'D', alias("dscvr"), help = "Include the discoverer of a given element.")]
    pub discoverer: bool,
    #[arg(long, short = 'g', help = "Include the group of a given element.")]
    pub group: bool,
    #[arg(long, short = 'p', help = "Include the period of a given element.")]
    pub period: bool,
}

pub fn exec(args: &ElementsArgs) {
    let periodic_table = create_periodic_table();
    let atomic_sym = &args.atomic_symbol;
    let element = periodic_table.get(atomic_sym).unwrap();
    let element_name = periodic_table.get(atomic_sym).unwrap()["element_name"]
        .to_string()
        .replace('\"', "");

    let set_atomic_number = || -> u64 {
        if args.atomic_number {
            periodic_table.get(atomic_sym).unwrap()["atomic_number"]
                .as_u64()
                .unwrap()
        } else {
            0
        }
    };

    let set_atomic_weight = || -> f64 {
        if args.atomic_weight {
            periodic_table.get(atomic_sym).unwrap()["atomic_weight"]
                .as_f64()
                .unwrap()
        } else {
            0.0
        }
    };

    let set_electronegativity = || -> f64 {
        if args.electronegativity {
            periodic_table.get(atomic_sym).unwrap()["electronegativity"]
                .as_f64()
                .unwrap()
        } else {
            0.0
        }
    };

    let set_series = || -> String {
        if args.series {
            periodic_table.get(atomic_sym).unwrap()["series"].to_string()
        } else {
            String::from("None")
        }
    };

    let set_date_discovered = || -> String {
        if args.date_discovered {
            periodic_table.get(atomic_sym).unwrap()["date_discovered"].to_string()
        } else {
            String::from("None")
        }
    };
    let set_discoverer = || -> String {
        if args.discoverer {
            periodic_table.get(atomic_sym).unwrap()["discoverer"].to_string()
        } else {
            String::from("None")
        }
    };
    let set_group = || -> String {
        if args.group {
            periodic_table.get(atomic_sym).unwrap()["group"]
                .to_string()
                .replace('\"', "")
        } else {
            format!("The element {element_name} is not a member of any group.")
        }
    };
    let set_period = || -> u64 {
        if args.period {
            periodic_table.get(atomic_sym).unwrap()["period"]
                .as_u64()
                .unwrap()
        } else {
            0
        }
    };
    let element_atomic_number = set_atomic_number();
    let element_atomic_weight = set_atomic_weight();
    let energy_levels = element["energy_levels"]
        .as_array()
        .unwrap()
        .to_owned()
        .into_iter()
        .collect::<Vec<Value>>()
        .iter()
        .map(|e| e.to_string())
        .map(|e| e.replace(&['\"', '\\'], ""))
        .collect::<Vec<String>>();

    let element_electronegativity = set_electronegativity();
    let element_series = set_series();
    let element_date_discovered = set_date_discovered().replace('\"', "");
    let element_discoverer = set_discoverer();
    let element_group = set_group();
    let element_period = set_period();

    if args.atomic_number {
        println!(
            "The atomic number of {} is {}.",
            element_name, element_atomic_number
        );
    }

    if args.atomic_weight {
        println!(
            "The atomic weight of {} is {}.",
            element_name, element_atomic_weight
        );
    }

    if args.energy_levels {
        let mut e_lvls = String::new();
        let num_e_lvls = energy_levels.len();

        if num_e_lvls == 1 {
            e_lvls.push_str(&(energy_levels[0].to_owned()));
        } else if num_e_lvls == 2 {
            e_lvls.push_str(&(energy_levels[0].to_owned() + ", "));
            e_lvls.push_str("and ");
            e_lvls.push_str(&(energy_levels[1].to_owned()));
        } else {
            for (idx, ele) in energy_levels.iter().enumerate() {
                e_lvls.push_str(&(ele.to_owned() + ", "));
                if idx == num_e_lvls - 2 {
                    e_lvls.push_str("and ");
                    e_lvls.push_str(ele);
                    break;
                }
            }
        }
        println!("The energy levels of {} are {}.", element_name, e_lvls);
    }

    if args.electronegativity {
        println!(
            "The electronegativity of {} is {}.",
            element_name, element_electronegativity
        );
    }
    if args.series {
        println!(
            "The element {} is a member of the {} series.",
            element_name, element_series
        );
    }
    if args.date_discovered {
        println!(
            "The element {} was discovered in {}.",
            element_name, element_date_discovered
        );
    }
    if args.discoverer {
        println!(
            "The person who discovered {} is {}.",
            element_name, element_discoverer
        );
    }
    if args.group {
        if element_group == "None".to_string() {
            println!("The element {} is not a member of any group.", element_name);
        } else {
            println!(
                "The element {} is a member of group {}.",
                element_name, element_group
            );
        }
    }
    if args.period {
        println!(
            "The element {} is a member of period {}.",
            element_name, element_period
        );
    }
}

pub fn create_periodic_table() -> serde_json::Value {
    let file = std::fs::read_to_string("./data/periodic_table.json")
        .expect("Was not able to open .json file.");
    let table = serde_json::from_str::<Value>(&file).expect("Unable to read value.");
    table
}

pub fn get_members(series: String, p_table: &Map<String, Value>) -> Vec<String> {
    let mut members = Vec::new();
    for i in p_table.iter() {
        let val = i.1["series"].to_string().replace('\"', "");
        if val == series {
            members.push(i.1["element_name"].clone().to_string())
        }
    }

    members
}
