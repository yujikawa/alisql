use dependsql;
use walkdir::WalkDir;
use regex::Regex;
fn main() {
    let root_dir = "src/sample_sqls";
    for entry in WalkDir::new(root_dir) {
        let entry = entry.unwrap();
        let re = Regex::new(r"(\w*)\.sql").unwrap();
        
        match entry.path().file_name() {
            Some(path) => { 
                if let Some(p) = path.to_str() {
                    if re.is_match(p) {
                        let sql = dependsql::SQL::new(format!("{}/{}", root_dir, p).to_string());
                        let tables = sql.get_ref_tables();
                        let query = sql.get_rendered_query();
                        println!("============");
                        println!("{:?}", tables);
                        println!("{}", query);
                        println!("============");
                    }
                }
                
            },
            None => {
                println!("Skip")
            }
        }

        // let sql = dependsql::SQL::new("src/sample.sql".to_string());
        // let tables = sql.get_ref_tables();
        // let query = sql.get_rendered_query();
        // println!("{:?}", tables);
        // println!("{}", query);
        

    }
    // println!("{:?}", sql);

}
