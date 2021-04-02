use cargo_metadata::{MetadataCommand};
use regex::Regex;

fn main() {
    let metadata = MetadataCommand::new()
    .exec()
    .unwrap();
    for package in metadata.packages {
        println!("====================================================");
        let name = package.name;
        println!("{} {}", name, package.version);
        package.homepage.or(package.repository)
            .map(|url| println!("{}", url));
        println!("by {}", package.authors.join(", "));
        package.description.map(|desc| println!("{}", desc));
        let mut path = package.manifest_path;
        path.pop();
        print_license(path, package.license).unwrap_or_else(
            |()| eprintln!("No license file found for {}", name)
        );
        println!("");
    }
}

fn print_license(path: cargo_metadata::camino::Utf8PathBuf, licenses: Option<String>) -> Result<(), ()> {
    let filename_regex = Regex::new(r"^(unlicense.*|copying.*|license.*|notice|notice.txt)").unwrap();
    let license: String;
    match licenses {
        Some(l) => {
            license = l
        },
        None => return Err(())
    }
    println!("License: {}", license);
    let mut haslicense = false;
    for f in path.read_dir().map_err(|_| ())? {
        match f {
            Ok(file) => {
                let wfilename = file.file_name();
                let filename = wfilename.to_str().unwrap();
                let path = file.path();
                if filename_regex.is_match(filename.to_lowercase().as_str()) {
                    haslicense = true;
                    println!("----------------------------------------------------");
                    println!("{}:\n\n{}", filename, std::fs::read_to_string(path).map_err(|_| ())?);
                }
            },
            Err(_) => (),
        };
    }
    if haslicense {
        return Ok(())
    } else {
        return Err(())
    }
}