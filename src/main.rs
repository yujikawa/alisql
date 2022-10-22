use dependsql;

fn main() {
    let root_dir = "src/sample_sqls";
    let lineage = dependsql::Lineage::get_dependencies(root_dir);
    dbg!(lineage);

}
