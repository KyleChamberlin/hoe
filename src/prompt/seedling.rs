use crate::seedling;

pub fn name() -> String {
    match demand::Input::new("What is the name of this seedling?")
        .validation(seedling::validations::name)
        .run()
    {
        Err(_) => todo!(),
        Ok(name) => name,
    }
}

pub fn description() -> String {
    match demand::Input::new("Please provide a short description of the seedling.")
        .validation(seedling::validations::description)
        .run()
    {
        Err(_) => todo!(),
        Ok(description) => description,
    }
}

pub fn tags() -> Vec<String> {
    let tag_list = match demand::Input::new("List applicable tags for this seedling.")
        .prompt("provide a semicolon (;) delimited list of tags:")
        // .validation(seedling::validations::tags)
        .run()
    {
        Err(_) => todo!(),
        Ok(tags) => tags,
    };

    tag_list.split(";").map(|s| s.trim().to_string()).collect()
}
