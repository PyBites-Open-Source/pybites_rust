// Download the exercises from https://rustplatform.com/
// and make them available locally.

use serde::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

#[derive(Deserialize, Debug)]
struct Bite {
    name: String,
    slug: String,
    description: String,
    level: String,
    template: String,
    libraries: String,
    author: String,
}

fn write_root_toml(path: &Path, bites: &[Bite]) -> std::io::Result<()> {
    let workspace_members = bites
        .iter()
        .map(|bite| {
            String::from("    \"")
                + bite.level.clone().as_str()
                + "/"
                + bite.slug.clone().as_str()
                + "\",\n"
        })
        .collect::<String>();

    // main Cargo.toml template
    let content = "[workspace]
resolver = \"3\"
members = [\nworkspace_members]"
        .replace("workspace_members", &workspace_members);

    let filename = path.join("Cargo.toml");
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn write_root_readme(path: &Path, bites: &[Bite]) -> std::io::Result<()> {
    // main README.md sections
    let static_content = "# Pybites Rust\n
https://rustplatform.com/
\n
## Exercises\n\n";
    let content_by_level = "### Level: bite_level\nbites_by_level\n";

    let filename = path.join("README.md");
    let mut file = File::create(filename)?;
    file.write_all(static_content.as_bytes())?;

    let levels = vec!["intro", "easy", "medium"];
    for level in levels {
        let bites_by_level = bites
            .iter()
            .filter(|bite| bite.level == level)
            .map(|bite| {
                "- [bite_level/bite_slug](bite_level/bite_slug/bite.md)\n"
                    .replace("bite_level", bite.level.clone().as_str())
                    .replace("bite_slug", bite.slug.clone().as_str())
            })
            .collect::<String>();

        file.write_all(
            content_by_level
                .replace("bite_level", level)
                .replace("bites_by_level", &bites_by_level)
                .as_bytes(),
        )?;
    }

    Ok(())
}

fn write_toml(path: &Path, slug: &str, libraries: &String) -> std::io::Result<()> {
    // exercise Cargo.toml template
    let content = "[package]
name = \"package_name\"
version = \"0.1.0\"
edition = \"2024\"\n
[dependencies]\n"
        .replace("package_name", slug);

    let filename = path.join("Cargo.toml");
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    file.write_all(libraries.as_bytes())?;

    Ok(())
}

fn write_exercise(path: &Path, template: &String) -> std::io::Result<()> {
    let src_dir = path.join("src");
    fs::create_dir_all(&src_dir)?;
    let filename = src_dir.join("lib.rs");

    if fs::exists(&filename)? {
        // backup the original lib.rs (exercise file) by adding a UNIX_EPOCH timestamp after .rs
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let new_filename = &filename.with_extension(format!("rs.{}", now));
        fs::rename(&filename, new_filename)?;
    }

    let mut file = File::create(filename)?;
    file.write_all(template.as_bytes())?;

    Ok(())
}

fn write_markdown(
    path: &Path,
    name: &str,
    description: &str,
    level: &str,
    author: &str,
) -> std::io::Result<()> {
    // exercise markdown template
    let content = "# package_name

- Level: package_level
- Author: package_author\n
## Instructions
package_description\n"
        .replace("package_name", name)
        .replace("package_description", description)
        .replace("package_level", level)
        .replace("package_author", author);

    let filename = path.join("bite.md");
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // define the base_path (current directory / exercises)
    let base_path = env::current_dir().unwrap().join("exercises");

    let api_key = env::var("PYBITES_API_KEY").ok();

    let client = reqwest::blocking::Client::new();
    let mut request = client.get("https://rustplatform.com/api/");
    if let Some(ref key) = api_key {
        println!("Authenticating with API key");
        request = request.header("X-API-Key", key);
    } else {
        println!("No API key set (PYBITES_API_KEY), downloading free exercises only");
    }

    print!("Downloading the exercises from Pybites Rust (rustplatform.com)");
    let response = request.send()?;
    println!(" ✅");
    println!(
        "'exercises' will be created in the current directory ({})",
        base_path.display()
    );

    // collect the arguments
    let args: Vec<String> = env::args().collect();
    // just testing, print out the status and headers and exit
    if args.contains(&String::from("--test")) {
        println!("Status: {}", response.status());
        println!("Headers:\n{:#?}", response.headers());
        return Ok(());
    }

    // extract the exercises from the response
    let bites: Vec<Bite> = response.json()?;
    println!("{:#?} exercises found!", bites.len());
    println!();

    // make sure the base path (exercises) exists
    fs::create_dir_all(&base_path)?;

    for bite in &bites {
        print!("{:#?}", bite.name);
        let slug = &bite.slug;
        let exercise_path = &base_path.join(&bite.level).join(slug);

        // make sure the exercise directory exists
        fs::create_dir_all(exercise_path)?;
        // re-write/update the toml and md files but make a backup copy of the
        // exercise file if it exists, in case it was already solved
        write_toml(exercise_path, slug, &bite.libraries)?;
        write_markdown(
            exercise_path,
            &bite.name,
            &bite.description,
            &bite.level,
            &bite.author,
        )?;
        write_exercise(exercise_path, &bite.template)?;
        println!(" ✅");
    }

    write_root_toml(&base_path, &bites)?;
    write_root_readme(&base_path, &bites)?;

    Ok(())
}
