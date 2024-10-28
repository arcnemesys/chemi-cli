use crate::element{exec, ElementsArgs, create_periodic_table, get_members};
fn main() {
    let periodic_table = create_periodic_table();
    let periodic_table_obj = periodic_table.as_object();
    println!("Hello, world!");
}
