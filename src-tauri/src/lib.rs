// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust! asshole", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod gym {

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

    struct Workout {
        workout_number: u32,
        workout_name: String,
        exercises: Vec<Exercise>,
        working_sets: Vec<u8>,
        rep_range: Vec<(u8, u8)>,
    }

    impl Workout {
        fn new() -> Workout {
            Workout {
                workout_number: 0,
                workout_name: "Unamed".to_string(),
                exercises: Vec::new(),
                working_sets: Vec::new(),
                rep_range: Vec::new(),
            }
        }

        fn from_workout(previous_workout: Workout, workout_number: u32) -> Workout {
            let mut current = Workout {
                workout_number,
                workout_name: previous_workout.workout_name,
                exercises: previous_workout.exercises,
                working_sets: previous_workout.working_sets,
                rep_range: previous_workout.rep_range,
            };

            for exercise in current.exercises.iter_mut() {
                exercise.sets.clear();
            }

            current
        }

        fn add_exercise(&mut self, exercise: Exercise) {
            self.exercises.push(exercise);
        }
    }

    impl Exercise {
        fn new(name: String, muscle: Muscle) -> Exercise {
            Exercise {
                name,
                muscle,
                sets: Vec::new(),
            }
        }

        fn add_set(&mut self, set: Set) {
            self.sets.push(set);
        }
    }

    impl Set {
        fn new(set_type: SetType, weight: u32, reps: u8, completed: bool) -> Set {
            Set {
                set_type,
                weight,
                reps,
                completed,
            }
        }
    }
}
