use schemars::schema::Schema;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let idx: usize = std::env::args().nth(2).unwrap().parse().unwrap();
    let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let group = &v[idx];
    eprintln!("group: {}", group["description"]);
    let schema: Schema = serde_json::from_value(group["schema"].clone()).unwrap();
    let mut ts = TypeSpace::new(&TypeSpaceSettings::default());
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ts.add_type_with_name(&schema, Some("Root".to_string()))
    }));
    match r {
        Ok(Ok(id)) => {
            let tsp = ts.to_typespace().unwrap();
            let ident = tsp.get_type(&id).ident();
            eprintln!("root ident: {}", ident);
            let stream = ts.to_stream().unwrap();
            let file = syn::parse2::<syn::File>(stream).unwrap();
            println!("{}", prettyplease::unparse(&file));
            eprintln!("root id {:?}", id);
        }
        Ok(Err(e)) => eprintln!("error: {e}"),
        Err(p) => eprintln!("panic: {:?}", p.downcast_ref::<String>()),
    }
}
