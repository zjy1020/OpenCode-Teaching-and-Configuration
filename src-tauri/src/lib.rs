use std::fs;
use std::path::PathBuf;
use rust_embed::Embed;
use serde::{Serialize, Deserialize};

#[derive(Embed)]
#[folder = "../skills"]
struct SkillsAsset;

#[derive(Embed)]
#[folder = "../commands"]
struct CommandsAsset;

#[derive(Embed)]
#[folder = "../themes"]
struct ThemesAsset;

#[derive(Serialize, Deserialize, Clone)]
struct AttentionConfig {
    enabled: bool,
    notifications: bool,
    sound: bool,
    volume: f32,
}

#[derive(Serialize, Deserialize, Clone)]
struct TuiConfig {
    theme: String,
    diff_style: String,
    attention: AttentionConfig,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            theme: "ember-glow".into(),
            diff_style: "auto".into(),
            attention: AttentionConfig {
                enabled: true,
                notifications: true,
                sound: true,
                volume: 0.4,
            },
        }
    }
}

fn find_opencode_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("无法获取用户目录")?;
    let legacy = home.join(".opencode");
    let xdg = home.join(".config").join("opencode");
    if xdg.exists() {
        Ok(xdg)
    } else if legacy.exists() {
        Ok(legacy)
    } else {
        fs::create_dir_all(&legacy).map_err(|e| e.to_string())?;
        Ok(legacy)
    }
}

fn both_opencode_dirs() -> Result<(PathBuf, PathBuf), String> {
    let home = dirs::home_dir().ok_or("无法获取用户目录")?;
    let xdg = home.join(".config").join("opencode");
    let legacy = home.join(".opencode");
    fs::create_dir_all(&xdg).map_err(|e| e.to_string())?;
    fs::create_dir_all(&legacy).map_err(|e| e.to_string())?;
    Ok((xdg, legacy))
}

#[tauri::command]
fn get_opencode_dir() -> Result<String, String> {
    let path = find_opencode_dir()?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn scan_skills() -> Result<Vec<String>, String> {
    let opencode = find_opencode_dir()?;
    let dir = opencode.join("skills");
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut items = vec![];
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            items.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    items.sort();
    Ok(items)
}

#[tauri::command]
fn scan_commands() -> Result<Vec<String>, String> {
    let opencode = find_opencode_dir()?;
    let dir = opencode.join("commands");
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut items = vec![];
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().map_err(|e| e.to_string())?.is_file() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".md") {
                items.push(name.trim_end_matches(".md").to_string());
            }
        }
    }
    items.sort();
    Ok(items)
}

#[tauri::command]
fn import_skill(name: String) -> Result<(), String> {
    let (xdg, legacy) = both_opencode_dirs()?;
    let prefix = format!("{}/", name);
    let mut count = 0u32;
    for path in SkillsAsset::iter() {
        let path_str = path.as_ref();
        if path_str == name.as_str() || path_str.starts_with(&prefix) {
            count += 1;
            let relative = path_str.strip_prefix(&prefix).unwrap_or("");
            let data = SkillsAsset::get(&path).ok_or("读取嵌入技能文件失败")?;
            for base in [&xdg, &legacy] {
                let target = base.join("skills").join(&name).join(relative);
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                fs::write(&target, data.data.as_ref()).map_err(|e| e.to_string())?;
            }
        }
    }
    if count == 0 {
        return Err(format!("技能 '{}' 不存在", name));
    }
    Ok(())
}

#[tauri::command]
fn remove_skill(name: String) -> Result<(), String> {
    let (xdg, legacy) = both_opencode_dirs()?;
    let mut found = false;
    for base in [&xdg, &legacy] {
        let dir = base.join("skills").join(&name);
        if dir.exists() {
            fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
            found = true;
        }
    }
    if !found {
        return Err(format!("技能 '{}' 未安装", name));
    }
    Ok(())
}

#[tauri::command]
fn import_command(name: String) -> Result<(), String> {
    let (xdg, legacy) = both_opencode_dirs()?;
    let filename = format!("{}.md", name);
    let data = CommandsAsset::get(&filename).ok_or(format!("命令 '{}' 不存在", name))?;
    for base in [&xdg, &legacy] {
        let dst_dir = base.join("commands");
        fs::create_dir_all(&dst_dir).map_err(|e| e.to_string())?;
        fs::write(dst_dir.join(&filename), data.data.as_ref()).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    std::process::Command::new("explorer")
        .arg(path)
        .spawn()
        .map_err(|e| format!("打开文件夹失败: {}", e))?;
    Ok(())
}

#[tauri::command]
fn remove_command(name: String) -> Result<(), String> {
    let (xdg, legacy) = both_opencode_dirs()?;
    let mut found = false;
    for base in [&xdg, &legacy] {
        let file = base.join("commands").join(format!("{}.md", name));
        if file.exists() {
            fs::remove_file(&file).map_err(|e| e.to_string())?;
            found = true;
        }
    }
    if !found {
        return Err(format!("命令 '{}' 未安装", name));
    }
    Ok(())
}

fn tui_config_path() -> Result<PathBuf, String> {
    Ok(find_opencode_dir()?.join("tui.json"))
}

#[tauri::command]
fn read_tui_config() -> Result<TuiConfig, String> {
    let path = tui_config_path()?;
    if !path.exists() {
        return Ok(TuiConfig::default());
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_tui_config(config: TuiConfig) -> Result<(), String> {
    let (xdg, legacy) = both_opencode_dirs()?;
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    for base in [&xdg, &legacy] {
        fs::write(base.join("tui.json"), &json).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn list_embedded_themes() -> Vec<String> {
    let mut themes: Vec<String> = ThemesAsset::iter()
        .map(|p| p.as_ref().trim_end_matches(".json").to_string())
        .collect();
    themes.sort();
    themes
}

#[tauri::command]
fn check_theme_installed() -> bool {
    let opencode = find_opencode_dir().ok();
    match opencode {
        Some(d) => d.join("themes").join("ember-glow.json").exists(),
        None => false,
    }
}

#[tauri::command]
fn import_theme() -> Result<(), String> {
    let (xdg, legacy) = both_opencode_dirs()?;
    let data = ThemesAsset::get("ember-glow.json").ok_or("读取嵌入主题失败")?;
    let mut installed = false;
    for base in [&xdg, &legacy] {
        let dst = base.join("themes").join("ember-glow.json");
        let parent = dst.parent().unwrap();
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        fs::write(&dst, data.data.as_ref()).map_err(|e| e.to_string())?;
        if dst.exists() { installed = true; }
    }
    if !installed {
        return Err("写入主题失败".into());
    }
    Ok(())
}

#[tauri::command]
fn remove_theme() -> Result<(), String> {
    let (xdg, legacy) = both_opencode_dirs()?;
    let mut found = false;
    for base in [&xdg, &legacy] {
        let file = base.join("themes").join("ember-glow.json");
        if file.exists() {
            fs::remove_file(&file).map_err(|e| e.to_string())?;
            found = true;
        }
    }
    if !found {
        return Err("主题未安装，无法卸载".into());
    }
    Ok(())
}

#[tauri::command]
fn sync_author_config() -> Result<(), String> {
    let (xdg, legacy) = both_opencode_dirs()?;
    let data = ThemesAsset::get("ember-glow.json").ok_or("读取嵌入主题失败")?;
    let config = serde_json::json!({
        "theme": "ember-glow",
        "diff_style": "stacked",
        "attention": {
            "enabled": true,
            "notifications": true,
            "sound": true,
            "volume": 0.4
        }
    });
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    for base in [&xdg, &legacy] {
        let dst = base.join("themes").join("ember-glow.json");
        fs::create_dir_all(dst.parent().unwrap()).map_err(|e| e.to_string())?;
        fs::write(&dst, data.data.as_ref()).map_err(|e| e.to_string())?;
        fs::write(base.join("tui.json"), &json).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn reset_tui_config() -> Result<(), String> {
    let (xdg, legacy) = both_opencode_dirs()?;
    let config = serde_json::json!({
        "theme": "ember-glow",
        "diff_style": "auto",
        "attention": {
            "enabled": false,
            "notifications": false,
            "sound": false,
            "volume": 0.4
        }
    });
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    for base in [&xdg, &legacy] {
        fs::write(base.join("tui.json"), &json).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_opencode_dir,
            scan_skills,
            scan_commands,
            import_skill,
            import_command,
            remove_skill,
            remove_command,
            open_folder,
            read_tui_config,
            write_tui_config,
            list_embedded_themes,
            check_theme_installed,
            import_theme,
            remove_theme,
            sync_author_config,
            reset_tui_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
