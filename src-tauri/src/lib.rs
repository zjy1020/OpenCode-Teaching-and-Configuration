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
    let prefix = format!("{}/", name);
    let mut count = 0u32;
    for path in SkillsAsset::iter() {
        let path_str = path.as_ref();
        if path_str == name.as_str() || path_str.starts_with(&prefix) {
            count += 1;
            let relative = path_str.strip_prefix(&prefix).unwrap_or("");
            let opencode = find_opencode_dir()?;
            let target = opencode.join("skills").join(&name).join(relative);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let data = SkillsAsset::get(&path).ok_or("读取嵌入技能文件失败")?;
            fs::write(&target, data.data).map_err(|e| e.to_string())?;
        }
    }
    if count == 0 {
        return Err(format!("技能 '{}' 不存在", name));
    }
    Ok(())
}

#[tauri::command]
fn remove_skill(name: String) -> Result<(), String> {
    let opencode = find_opencode_dir()?;
    let dir = opencode.join("skills").join(&name);
    if !dir.exists() {
        return Err(format!("技能 '{}' 未安装", name));
    }
    fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn import_command(name: String) -> Result<(), String> {
    let filename = format!("{}.md", name);
    let data = CommandsAsset::get(&filename).ok_or(format!("命令 '{}' 不存在", name))?;
    let opencode = find_opencode_dir()?;
    let dst_dir = opencode.join("commands");
    fs::create_dir_all(&dst_dir).map_err(|e| e.to_string())?;
    fs::write(dst_dir.join(&filename), data.data).map_err(|e| e.to_string())?;
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
    let opencode = find_opencode_dir()?;
    let file = opencode.join("commands").join(format!("{}.md", name));
    if !file.exists() {
        return Err(format!("命令 '{}' 未安装", name));
    }
    fs::remove_file(&file).map_err(|e| e.to_string())?;
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
    let opencode = find_opencode_dir()?;

    let path = opencode.join("tui.json");
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;

    let theme_name = config.theme.trim_end_matches(".json");
    let theme_file = format!("{}.json", theme_name);

    let dst_dir = opencode.join("themes");
    fs::create_dir_all(&dst_dir).map_err(|e| e.to_string())?;
    let dst_path = dst_dir.join(&theme_file);

    if !dst_path.exists() {
        if let Some(data) = ThemesAsset::get(&theme_file) {
            fs::write(&dst_path, data.data).map_err(|e| e.to_string())?;
        }
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
fn read_theme_content(name: String) -> Result<String, String> {
    let file = format!("{}.json", name.trim_end_matches(".json"));
    let data = ThemesAsset::get(&file).ok_or_else(|| format!("主题 '{}' 不存在", name))?;
    let text = std::str::from_utf8(data.data.as_ref()).map_err(|e| e.to_string())?;
    Ok(text.to_string())
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
            read_theme_content,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
