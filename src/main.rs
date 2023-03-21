use std::fs;
use toml::Value;

fn main() {
    // Load the configuration file
    let config_file = fs::read_to_string("config.toml").expect("Error reading configuration file");
    let config: Value = toml::from_str(&config_file).expect("Error parsing configuration file");

    // Get the communication settings
    let protocol = config["communication"]["protocol"].as_str().unwrap();
    let cipher = config["communication"]["cipher"].as_str().unwrap();
    let port = config["communication"]["port"].as_integer().unwrap();

    // Print the communication settings
    println!("Communication Settings:");
    println!("Protocol: {}", protocol);
    println!("Cipher: {}", cipher);
    println!("Port: {}", port);

    // Get the list of repositories
    let repositories = config["repositories"].as_array().unwrap();

    // Print the list of repositories
    println!("\nRepositories:");
    for repository in repositories {
        let name = repository["name"].as_str().unwrap();
        let url = repository["url"].as_str().unwrap();
        let access_role = repository["access_role"].as_str().unwrap();

        println!("Name: {}", name);
        println!("URL: {}", url);
        println!("Access Role: {}", access_role);
    }

    // Get the list of dependencies
    let dependencies = config["dependencies"].as_array().unwrap();

    // Print the list of dependencies
    println!("\nDependencies:");
    for dependency in dependencies {
        let name = dependency["name"].as_str().unwrap();
        let version = dependency["version"].as_str().unwrap();

        println!("Name: {}", name);
        println!("Version: {}", version);
    }

    // Get the workflow stages and actions
    let stages = config["stages"].as_array().unwrap();

    // Print the workflow stages and actions
    println!("\nWorkflow Stages and Actions:");
    for stage in stages {
        let name = stage["name"].as_str().unwrap();
        let order = stage["order"].as_integer().unwrap();

        println!("Name: {}", name);
        println!("Order: {}", order);

        let actions = stage["actions"].as_array().unwrap();

        for action in actions {
            let name = action["name"].as_str().unwrap();
            let action_type = action["type"].as_str().unwrap();

            println!("Action Name: {}", name);
            println!("Action Type: {}", action_type);
        }
    }
}