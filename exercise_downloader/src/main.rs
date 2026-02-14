// Download the exercises from https://rustplatform.com/
// and make them available locally.

use serde::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

#[derive(Deserialize, Debug, Clone)]
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

fn auth_status_message(api_key: &Option<String>) -> &'static str {
    if api_key.is_some() {
        "Authenticating with API key"
    } else {
        "No API key set (PYBITES_API_KEY), downloading free exercises only"
    }
}

fn build_request(
    client: &reqwest::blocking::Client,
    url: &str,
    api_key: Option<&str>,
) -> reqwest::blocking::RequestBuilder {
    let mut request = client.get(url);
    if let Some(key) = api_key {
        request = request.header("X-API-Key", key);
    }
    request
}

fn write_all_exercises(base_path: &Path, bites: &[Bite]) -> std::io::Result<()> {
    fs::create_dir_all(base_path)?;

    for bite in bites {
        let exercise_path = base_path.join(&bite.level).join(&bite.slug);
        fs::create_dir_all(&exercise_path)?;
        write_toml(&exercise_path, &bite.slug, &bite.libraries)?;
        write_markdown(
            &exercise_path,
            &bite.name,
            &bite.description,
            &bite.level,
            &bite.author,
        )?;
        write_exercise(&exercise_path, &bite.template)?;
    }

    write_root_toml(base_path, bites)?;
    write_root_readme(base_path, bites)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = env::current_dir().unwrap().join("exercises");

    let api_key = env::var("PYBITES_API_KEY").ok();

    let client = reqwest::blocking::Client::new();
    let request = build_request(&client, "https://rustplatform.com/api/", api_key.as_deref());
    println!("{}", auth_status_message(&api_key));

    print!("Downloading the exercises from Pybites Rust (rustplatform.com)");
    let response = request.send()?;
    println!(" ✅");
    println!(
        "'exercises' will be created in the current directory ({})",
        base_path.display()
    );

    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--test")) {
        println!("Status: {}", response.status());
        println!("Headers:\n{:#?}", response.headers());
        return Ok(());
    }

    let bites: Vec<Bite> = response.json()?;
    println!("{:#?} exercises found!", bites.len());
    println!();

    write_all_exercises(&base_path, &bites)?;

    for bite in &bites {
        println!("{:#?} ✅", bite.name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn sample_bite(name: &str, slug: &str, level: &str) -> Bite {
        Bite {
            name: name.to_string(),
            slug: slug.to_string(),
            description: "A test exercise".to_string(),
            level: level.to_string(),
            template: "fn main() {}".to_string(),
            libraries: "serde = \"1.0\"\n".to_string(),
            author: "testauthor".to_string(),
        }
    }

    #[test]
    fn test_bite_deserialize() {
        let json = r#"{
            "name": "Hello",
            "slug": "hello",
            "description": "desc",
            "level": "intro",
            "template": "fn main() {}",
            "libraries": "",
            "author": "bob"
        }"#;
        let bite: Bite = serde_json::from_str(json).unwrap();
        assert_eq!(bite.name, "Hello");
        assert_eq!(bite.slug, "hello");
        assert_eq!(bite.level, "intro");
    }

    #[test]
    fn test_bite_deserialize_list() {
        let json = r#"[
            {"name":"A","slug":"a","description":"d","level":"intro","template":"t","libraries":"","author":"x"},
            {"name":"B","slug":"b","description":"d","level":"easy","template":"t","libraries":"","author":"x"}
        ]"#;
        let bites: Vec<Bite> = serde_json::from_str(json).unwrap();
        assert_eq!(bites.len(), 2);
        assert_eq!(bites[0].slug, "a");
        assert_eq!(bites[1].slug, "b");
    }

    #[test]
    fn test_write_toml() {
        let dir = TempDir::new().unwrap();
        let libs = "serde = \"1.0\"\n".to_string();
        write_toml(dir.path(), "my-exercise", &libs).unwrap();

        let content = fs::read_to_string(dir.path().join("Cargo.toml")).unwrap();
        assert!(content.contains("name = \"my-exercise\""));
        assert!(content.contains("edition = \"2024\""));
        assert!(content.contains("[dependencies]"));
        assert!(content.contains("serde = \"1.0\""));
    }

    #[test]
    fn test_write_toml_empty_libraries() {
        let dir = TempDir::new().unwrap();
        let libs = String::new();
        write_toml(dir.path(), "bare-exercise", &libs).unwrap();

        let content = fs::read_to_string(dir.path().join("Cargo.toml")).unwrap();
        assert!(content.contains("name = \"bare-exercise\""));
        assert!(content.contains("[dependencies]"));
    }

    #[test]
    fn test_write_exercise_creates_src_dir_and_lib() {
        let dir = TempDir::new().unwrap();
        let template = "fn main() { println!(\"hello\"); }".to_string();
        write_exercise(dir.path(), &template).unwrap();

        let lib_path = dir.path().join("src").join("lib.rs");
        assert!(lib_path.exists());
        let content = fs::read_to_string(lib_path).unwrap();
        assert_eq!(content, template);
    }

    #[test]
    fn test_write_exercise_backs_up_existing() {
        let dir = TempDir::new().unwrap();
        let original = "fn original() {}".to_string();
        let updated = "fn updated() {}".to_string();

        write_exercise(dir.path(), &original).unwrap();
        write_exercise(dir.path(), &updated).unwrap();

        let lib_content = fs::read_to_string(dir.path().join("src").join("lib.rs")).unwrap();
        assert_eq!(lib_content, updated);

        let backup_files: Vec<_> = fs::read_dir(dir.path().join("src"))
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().to_string_lossy().contains("lib.rs."))
            .collect();
        assert_eq!(backup_files.len(), 1);

        let backup_content = fs::read_to_string(backup_files[0].path()).unwrap();
        assert_eq!(backup_content, original);
    }

    #[test]
    fn test_write_markdown() {
        let dir = TempDir::new().unwrap();
        write_markdown(dir.path(), "Test Bite", "Do the thing", "easy", "bob").unwrap();

        let content = fs::read_to_string(dir.path().join("bite.md")).unwrap();
        assert!(content.contains("# Test Bite"));
        assert!(content.contains("Level: easy"));
        assert!(content.contains("Author: bob"));
        assert!(content.contains("Do the thing"));
    }

    #[test]
    fn test_write_root_toml() {
        let dir = TempDir::new().unwrap();
        let bites = vec![
            sample_bite("Hello", "hello", "intro"),
            sample_bite("Advanced", "advanced", "medium"),
        ];
        write_root_toml(dir.path(), &bites).unwrap();

        let content = fs::read_to_string(dir.path().join("Cargo.toml")).unwrap();
        assert!(content.contains("[workspace]"));
        assert!(content.contains("resolver = \"3\""));
        assert!(content.contains("\"intro/hello\""));
        assert!(content.contains("\"medium/advanced\""));
    }

    #[test]
    fn test_write_root_toml_empty() {
        let dir = TempDir::new().unwrap();
        let bites: Vec<Bite> = vec![];
        write_root_toml(dir.path(), &bites).unwrap();

        let content = fs::read_to_string(dir.path().join("Cargo.toml")).unwrap();
        assert!(content.contains("[workspace]"));
        assert!(content.contains("members = ["));
    }

    #[test]
    fn test_write_root_readme() {
        let dir = TempDir::new().unwrap();
        let bites = vec![
            sample_bite("Hello", "hello", "intro"),
            sample_bite("Strings", "strings", "easy"),
        ];
        write_root_readme(dir.path(), &bites).unwrap();

        let content = fs::read_to_string(dir.path().join("README.md")).unwrap();
        assert!(content.contains("# Pybites Rust"));
        assert!(content.contains("### Level: intro"));
        assert!(content.contains("### Level: easy"));
        assert!(content.contains("[intro/hello](intro/hello/bite.md)"));
        assert!(content.contains("[easy/strings](easy/strings/bite.md)"));
    }

    #[test]
    fn test_write_root_readme_skips_unlisted_levels() {
        let dir = TempDir::new().unwrap();
        let bites = vec![sample_bite("Hard One", "hard-one", "hard")];
        write_root_readme(dir.path(), &bites).unwrap();

        let content = fs::read_to_string(dir.path().join("README.md")).unwrap();
        assert!(!content.contains("hard-one"));
    }

    #[test]
    fn test_auth_status_message_with_key() {
        let key = Some("abc-123".to_string());
        assert_eq!(auth_status_message(&key), "Authenticating with API key");
    }

    #[test]
    fn test_auth_status_message_without_key() {
        let key: Option<String> = None;
        assert!(auth_status_message(&key).contains("No API key set"));
    }

    #[test]
    fn test_build_request_without_api_key() {
        let client = reqwest::blocking::Client::new();
        let request = build_request(&client, "https://example.com/api/", None);
        let built = request.build().unwrap();
        assert!(built.headers().get("X-API-Key").is_none());
    }

    #[test]
    fn test_build_request_with_api_key() {
        let client = reqwest::blocking::Client::new();
        let request = build_request(&client, "https://example.com/api/", Some("test-key-123"));
        let built = request.build().unwrap();
        assert_eq!(
            built.headers().get("X-API-Key").unwrap().to_str().unwrap(),
            "test-key-123"
        );
    }

    #[test]
    fn test_build_request_url() {
        let client = reqwest::blocking::Client::new();
        let request = build_request(&client, "https://example.com/api/", None);
        let built = request.build().unwrap();
        assert_eq!(built.url().as_str(), "https://example.com/api/");
    }

    #[test]
    fn test_build_request_is_get() {
        let client = reqwest::blocking::Client::new();
        let request = build_request(&client, "https://example.com/api/", None);
        let built = request.build().unwrap();
        assert_eq!(built.method(), reqwest::Method::GET);
    }

    #[test]
    fn test_write_all_exercises() {
        let dir = TempDir::new().unwrap();
        let base = dir.path().join("exercises");
        let bites = vec![
            sample_bite("Hello", "hello", "intro"),
            sample_bite("Strings", "strings", "easy"),
        ];
        write_all_exercises(&base, &bites).unwrap();

        assert!(base.join("intro").join("hello").join("Cargo.toml").exists());
        assert!(base.join("intro").join("hello").join("bite.md").exists());
        assert!(
            base.join("intro")
                .join("hello")
                .join("src")
                .join("lib.rs")
                .exists()
        );
        assert!(
            base.join("easy")
                .join("strings")
                .join("Cargo.toml")
                .exists()
        );
        assert!(base.join("Cargo.toml").exists());
        assert!(base.join("README.md").exists());

        let root_toml = fs::read_to_string(base.join("Cargo.toml")).unwrap();
        assert!(root_toml.contains("\"intro/hello\""));
        assert!(root_toml.contains("\"easy/strings\""));
    }

    #[test]
    fn test_write_all_exercises_empty() {
        let dir = TempDir::new().unwrap();
        let base = dir.path().join("exercises");
        let bites: Vec<Bite> = vec![];
        write_all_exercises(&base, &bites).unwrap();

        assert!(base.join("Cargo.toml").exists());
        assert!(base.join("README.md").exists());
    }

    #[test]
    fn test_write_all_exercises_preserves_existing_work() {
        let dir = TempDir::new().unwrap();
        let base = dir.path().join("exercises");
        let bites = vec![sample_bite("Hello", "hello", "intro")];

        write_all_exercises(&base, &bites).unwrap();

        let lib_path = base.join("intro").join("hello").join("src").join("lib.rs");
        fs::write(&lib_path, "fn my_solution() {}").unwrap();

        write_all_exercises(&base, &bites).unwrap();

        let lib_content = fs::read_to_string(&lib_path).unwrap();
        assert_eq!(lib_content, "fn main() {}");

        let backup_files: Vec<_> = fs::read_dir(base.join("intro").join("hello").join("src"))
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().to_string_lossy().contains("lib.rs."))
            .collect();
        assert_eq!(backup_files.len(), 1);
        let backup_content = fs::read_to_string(backup_files[0].path()).unwrap();
        assert_eq!(backup_content, "fn my_solution() {}");
    }
}
