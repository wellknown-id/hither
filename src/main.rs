use std::collections::HashSet;
use std::path::{Path, PathBuf};

use wasmtime::*;
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: hither [--install [--alias=NAME]] | <command> [args...]");
        std::process::exit(1);
    }

    if args[1] == "--install" {
        let alias = args
            .iter()
            .find(|a| a.starts_with("--alias="))
            .map(|a| a.trim_start_matches("--alias=").trim());
        if let Err(e) = install(alias) {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
        return;
    }

    let command = &args[1];
    let guest_args = &args[2..];

    let wasm_path = match find_wasm(command) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let wasm_bytes = match std::fs::read(&wasm_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error reading {}: {e}", wasm_path.display());
            std::process::exit(1);
        }
    };

    let full_args = if command == "list" {
        let mut a = vec!["list".to_string()];
        a.extend(find_all_modules());
        a
    } else {
        let mut a = vec![command.clone()];
        a.extend(guest_args.iter().cloned());
        a
    };

    if let Err(e) = run_wasm(&wasm_bytes, &full_args) {
        match e.downcast::<wasmtime_wasi::I32Exit>() {
            Ok(exit) => std::process::exit(exit.0),
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    }
}

pub fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(PathBuf::from)
}

pub fn find_wasm(command: &str) -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let home = home_dir();
    find_wasm_in(command, &cwd, home.as_deref())
}

pub fn find_wasm_in(command: &str, cwd: &Path, home: Option<&Path>) -> Result<PathBuf, String> {
    let local = cwd.join(".hither").join(format!("{command}.wasm"));
    if local.exists() {
        return Ok(local);
    }

    if let Some(h) = home {
        let home_path = h.join(".hither").join(format!("{command}.wasm"));
        if home_path.exists() {
            return Ok(home_path);
        }
    }

    Err(format!("no .hither/{command}.wasm found in . or ~"))
}

pub fn find_all_modules() -> Vec<String> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let home = home_dir();
    find_all_modules_in(&cwd, home.as_deref())
}

pub fn find_all_modules_in(cwd: &Path, home: Option<&Path>) -> Vec<String> {
    let mut modules = HashSet::new();

    let mut dirs: Vec<PathBuf> = vec![cwd.join(".hither")];
    if let Some(h) = home {
        dirs.push(h.join(".hither"));
    }

    for dir in dirs {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "wasm") {
                    if let Some(name) = path.file_stem() {
                        modules.insert(name.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    let mut result: Vec<String> = modules.into_iter().collect();
    result.sort();
    result
}

pub fn run_wasm(wasm_bytes: &[u8], args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let module = Module::from_binary(&engine, wasm_bytes)?;

    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    p1::add_to_linker_sync(&mut linker, |cx| cx)?;

    let cwd = std::env::current_dir()?;
    let wasi = WasiCtxBuilder::new()
        .args(args)
        .inherit_stdio()
        .preopened_dir(&cwd, ".", DirPerms::all(), FilePerms::all())?
        .build_p1();

    let mut store = Store::new(&engine, wasi);
    let instance = linker.instantiate(&mut store, &module)?;

    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    start.call(&mut store, ())?;

    Ok(())
}

fn install(alias: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let exe = std::env::current_exe()?;
    let bin_dir = install_dir()?;
    std::fs::create_dir_all(&bin_dir)?;

    let dest = bin_dir.join(exe.file_name().unwrap());
    std::fs::copy(&exe, &dest)?;
    println!("Installed hither to {}", bin_dir.display());

    if let Some(name) = alias {
        if !name.is_empty() {
            let alias_path = bin_dir.join(name);
            #[cfg(unix)]
            {
                if alias_path.exists() {
                    std::fs::remove_file(&alias_path)?;
                }
                std::os::unix::fs::symlink(&dest, &alias_path)?;
            }
            #[cfg(windows)]
            {
                std::fs::copy(&exe, alias_path.with_extension("exe"))?;
            }
            println!("Created alias '{}' -> 'hither'", name);
        }
    }

    Ok(())
}

fn install_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        if let Some(home) = home_dir() {
            Ok(home.join(".local").join("bin"))
        } else {
            Err("cannot determine home directory".into())
        }
    }
    #[cfg(windows)]
    {
        let app_data = std::env::var("LOCALAPPDATA")?;
        Ok(PathBuf::from(app_data).join("hither").join("bin"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn make_wasm(dir: &Path, name: &str) -> PathBuf {
        let hither_dir = dir.join(".hither");
        fs::create_dir_all(&hither_dir).unwrap();
        let wasm_file = hither_dir.join(format!("{name}.wasm"));
        fs::write(&wasm_file, b"\x00asm").unwrap();
        wasm_file
    }

    #[test]
    fn test_find_wasm_current_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let wasm_file = make_wasm(tmp.path(), "echo");

        let p = find_wasm_in("echo", tmp.path(), None).unwrap();
        assert_eq!(p, wasm_file);
    }

    #[test]
    fn test_find_wasm_home_dir() {
        let home_tmp = tempfile::tempdir().unwrap();
        let wasm_file = make_wasm(home_tmp.path(), "echo");

        let work_tmp = tempfile::tempdir().unwrap();

        let p = find_wasm_in("echo", work_tmp.path(), Some(home_tmp.path())).unwrap();
        assert_eq!(p, wasm_file);
    }

    #[test]
    fn test_find_wasm_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let result = find_wasm_in("nope", tmp.path(), Some(tmp.path()));
        assert!(result.is_err());
    }

    #[test]
    fn test_find_all_modules() {
        let tmp = tempfile::tempdir().unwrap();
        let hither_dir = tmp.path().join(".hither");
        fs::create_dir_all(&hither_dir).unwrap();
        fs::write(hither_dir.join("echo.wasm"), b"\x00asm").unwrap();
        fs::write(hither_dir.join("list.wasm"), b"\x00asm").unwrap();
        fs::write(hither_dir.join("readme.txt"), b"not wasm").unwrap();

        let modules = find_all_modules_in(tmp.path(), None);
        assert_eq!(modules, vec!["echo", "list"]);
    }

    #[test]
    fn test_run_wasm_echo() {
        let wasm_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(".hither")
            .join("echo.wasm");
        let wasm_bytes = match fs::read(&wasm_path) {
            Ok(b) => b,
            Err(_) => return,
        };

        run_wasm(&wasm_bytes, &["echo".to_string(), "hello".to_string()]).unwrap();
    }
}
