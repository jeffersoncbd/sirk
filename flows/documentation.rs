use sirk_sdk::Sirk;
use std::{fs, path::Path};

pub fn run(sirk: &Sirk) -> Result<(), Box<dyn std::error::Error>> {
    let docs_exists = Path::new("docs").is_dir();
    let files: Vec<String> = if docs_exists {
        sirk.tools().git().status()?
    } else {
        fs::create_dir("docs")?;
        sirk.tools().tree()?
    };

    for file_name in &files {
        let doc_name = format!("docs/{file_name}.md");
        let doc_exists = Path::new(&doc_name).is_file();

        let file_exists = Path::new(file_name).is_file();

        if file_exists {
            let file_content = fs::read_to_string(file_name)?;

            let input = if doc_exists {
                let doc_content = fs::read_to_string(doc_name.clone())?;
                r#"
                    ## Nome do arquivo: ${fileName}

                    ## Conteúdo atual do documento:
                    ${docContent}

                    ## Conteúdo do arquivo:
                    ${fileContent}
                "#
                .replace("${fileName}", file_name)
                .replace("${docContent}", &doc_content)
                .replace("${fileContent}", &file_content)
            } else {
                r#"
                    ## Nome do arquivo: ${fileName}

                    ## Conteúdo do arquivo:
                    ${fileContent}
                "#
                .replace("${fileName}", file_name)
                .replace("${fileContent}", &file_content)
            };
            let explain = sirk.agent("code-explainer", &input)?;
            let resume = sirk.agent("explain-resume", &explain)?.trim().to_string();

            if let Some(parent) = Path::new(&doc_name).parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(doc_name, &explain)?;

            let tree_exists = Path::new("docs/TREE.md").is_file();
            if tree_exists {
                let tree_content = fs::read_to_string("docs/TREE.md")?;
                let mut updated = false;
                let splited_content: Vec<&str> = tree_content.split("\n").collect();
                let mut updated_content: Vec<String> = splited_content
                    .iter()
                    .map(|line| {
                        if line.starts_with(&format!("{file_name} - ")) {
                            updated = true;
                            format!("{file_name} - {resume}")
                        } else {
                            line.to_string()
                        }
                    })
                    .collect();
                if !updated {
                    updated_content.push(format!("{file_name} - {resume}"));
                }
                updated_content.sort();
                let body = updated_content.join("\n").to_string();
                fs::write("docs/TREE.md", body)?;
            } else {
                fs::write("docs/TREE.md", format!("{file_name} - {resume}"))?;
            }
        } else {
            if doc_exists {
                fs::remove_file(doc_name)?;
            }

            let tree_exists = Path::new("docs/TREE.md").is_file();
            if tree_exists {
                let tree_content = fs::read_to_string("docs/TREE.md")?;
                let splited_content: Vec<&str> = tree_content.split("\n").collect();
                let mut updated_content: Vec<String> = splited_content
                    .iter()
                    .filter(|line| !line.starts_with(&format!("{file_name} - ")))
                    .map(|l| l.to_string())
                    .collect();
                updated_content.sort();
                let body = updated_content.join("\n").to_string();

                fs::write("docs/TREE.md", body)?;
            }
        }
    }

    sirk.tools().await_confirm()?;
    sirk.tools().git().add()?;

    Ok(())
}
