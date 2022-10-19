use dependsql;
fn main() {
    let sql = dependsql::SQL::new("src/sample.sql".to_string());
    let tables = sql.get_ref_tables();
    let query = sql.get_rendered_query();
    // println!("{:?}", sql);
    println!("{:?}", tables);
    println!("{}", query);
}
