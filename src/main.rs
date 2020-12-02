use tera::{Tera, Context};
use structopt::StructOpt;
use std::result::Result;

use std::fs;

#[macro_use]
extern crate serde_derive;


#[derive(StructOpt, Debug)]
struct CLI {
    /// switch on verbosity
    #[structopt(short, long)]
    name: String,
    #[structopt(short, long)]
    port: i32,
    #[structopt(short, long)]
    domain: String,
}


#[derive(Serialize)]
struct ProjectVars {
    name: String,
    port: i32,
    domain: String
}

fn render_template(tera: &Tera, template_path: &str, vars: &ProjectVars, output_path: &str) -> Result<(), tera::Error> {
    let result = tera.render(template_path, &Context::from_serialize(&vars)?)?;
    fs::write(output_path, result).unwrap();
    Ok(())
}


fn render_templates(vars:  ProjectVars) -> Result<(), tera::Error> {

    let output_dir = format!("/srv/{}/", vars.name);
    println!("++ rendering templates to {:?}", output_dir);

    let tera = match Tera::new("templates/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    println!("++ creating directory {}", output_dir);
    fs::create_dir_all(&output_dir).unwrap();

    let uploads_dir = format!("{}/wp-content/uploads", output_dir);
    println!("++ creating directory {}", uploads_dir);
    fs::create_dir_all(&uploads_dir).unwrap();

    println!("++ writing files");
    let output_path = format!("{}/.env", output_dir);
    render_template(&tera, ".env", &vars, &output_path);

    let output_path = format!("{}/docker-compose.yml", output_dir);
    render_template(&tera, "docker-compose.yml", &vars, &output_path);

    let output_path = format!("{}/php.ini", output_dir);
    render_template(&tera, "php.ini", &vars, &output_path);

    let output_path = format!("{}/run.md", output_dir);
    render_template(&tera, "run.md", &vars, &output_path);

    let output_path = format!("/etc/nginx/sites-enabled/{}", vars.domain);
    render_template(&tera, "nginx.conf", &vars, &output_path);

    Ok(())
}


fn main() {
    println!("Hello, world!");

    // parse cli arguments
    let opt = CLI::from_args();

    let project_vars = ProjectVars{name:opt.name, port: opt.port, domain: opt.domain};

    let result = render_templates(project_vars);
    match result {
        Ok(()) => println!("++ successfully rendered templates"),
        Err(e) => println!("++ error: {:?}", e)
    }

}
