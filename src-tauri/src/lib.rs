// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust! asshole", name)
}

enum SetType {
    WarmUp,
    Working(u8),
    Drop,
}

enum Muscle {
    Chest,
    Shoulders,
    Biceps,
    Triceps,
    UpperBack,
    LowerBack,
    Lats,
    Forearms,
    Abs,
    Quads,
    Hamstrings,
    Glutes,
    Traps,
    Calves,
}

struct Workout {
    workout_number: u32,
    workout_name: String,
    exercises: Vec<Exercise>,
    working_sets: Vec<u8>,
    rep_range: Vec<(u8, u8)>,
}

struct Exercise {
    name: String,
    muscle: Muscle,
    sets: Vec<Set>,
}

struct Set {
    set_type: SetType,
    weight: u32,
    reps: u8,
    completed: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
