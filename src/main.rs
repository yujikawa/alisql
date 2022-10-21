use dependsql;

fn main() {
    let root_dir = "src/sample_sqls";
    let lineage = dependsql::Lineage::get_lineage(root_dir);
    println!("{:?}", lineage);

}
