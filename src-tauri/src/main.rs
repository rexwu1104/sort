#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use rand::Rng;

use src_macro::{generate_set, generate_get};
use tauri::State;

mod sorts;

fn to_string<T: ToString>(value: T) -> String { value.to_string() }

#[tauri::command]
#[allow(non_snake_case)]
async fn set_sortType(sortType: String, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut app = state.lock().map_err(to_string)?;
    app.current_sort_type = match &*sortType {
        "Bubble Sort"    => SortType::Bubble,
        "Insert Sort"    => SortType::Insert,
        "Cocktail Sort"  => SortType::Cocktail,
        "Merge Sort"     => SortType::Merge,
        "Bucket Sort"    => SortType::Bucket,
        "Radix Sort"     => SortType::Radix,
        "Selection Sort" => SortType::Selection,
        "Comb Sort"      => SortType::Comb,
        "Shell Sort"     => SortType::Shell,
        "Heap Sort"      => SortType::Heap,
        "Quick Sort"     => SortType::Quick,
        _ => return Err("unknown sort method.".into())
    };

    Ok(())
}

#[tauri::command]
async fn generate_random_numbers(state: State<'_, Mutex<AppState>>) -> Result<Vec<usize>, String> {
    let mut app = state.lock().map_err(to_string)?;
    let mut rng = rand::thread_rng();
    app.randomBuffer = (0..app.n).map(|_| rng.gen_range(0..=app.range)).collect();
    Ok(app.randomBuffer.clone())
}

#[tauri::command]
async fn sort(state: State<'_, Mutex<AppState>>) -> Result<(Vec<usize>, f64), String> {
    let mut app = state.lock().map_err(to_string)?;
    // println!("{app:#?}");
    let sorted = match app.current_sort_type {
        SortType::Bubble    => sorts::bubble_sort(app.randomBuffer.clone()),
        SortType::Insert    => sorts::insert_sort(app.randomBuffer.clone()),
        SortType::Cocktail  => sorts::cocktail_sort(app.randomBuffer.clone()),
        SortType::Merge     => sorts::merge_sort(app.randomBuffer.clone()),
        SortType::Bucket    => sorts::bucket_sort(app.randomBuffer.clone()),
        SortType::Radix     => sorts::radix_sort(app.randomBuffer.clone()),
        SortType::Selection => sorts::selection_sort(app.randomBuffer.clone()),
        SortType::Comb      => sorts::comb_sort(app.randomBuffer.clone()),
        SortType::Shell     => sorts::shell_sort(app.randomBuffer.clone()),
        SortType::Heap      => sorts::heap_sort(app.randomBuffer.clone()),
        SortType::Quick     => sorts::quick_sort(app.randomBuffer.clone())
    };

    app.sorted_buffer = sorted.0.clone();
    Ok(sorted)
}

#[tauri::command]
async fn search(state: State<'_, Mutex<AppState>>) -> Result<(usize, bool), String> {
    let app = state.lock().map_err(to_string)?;
    let slice = app.sorted_buffer.as_slice();
    let mut i = 0;
    let mut j = slice.len() - 1;
    let result = loop {
        if slice[i] == app.target {
            break (i, true);
        } else if slice[j] == app.target {
            break (j, true);
        } else if j - 1 == i {
            break (0, false);
        }

        let mid = (i + j) / 2;
        if slice[mid] <= app.target {
            i = mid;
        } else if slice[mid] >= app.target {
            j = mid;
        }
    };

    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState { ..Default::default() }))
        .invoke_handler(tauri::generate_handler![
            set_n,
            get_n,
            set_range,
            get_range,
            set_repeatition,
            get_repeatition,
            set_step,
            get_step,
            set_target,
            get_target,
            set_echoPrint,
            get_echoPrint,
            set_selfCheck,
            get_selfCheck,
            set_randomBuffer,
            get_randomBuffer,
            set_sortType,
            generate_random_numbers,
            sort,
            search
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Default, Debug)]
enum SortType {
    #[default]
    Bubble,
    Insert,
    Cocktail,
    Merge,
    Bucket,
    Radix,
    Selection,
    Comb,
    Shell,
    Heap,
    Quick
}

#[derive(Default, Debug)]
#[generate_set]
#[generate_get]
#[allow(unused_attributes)]
struct AppState {
    n: usize,
    range: usize,
    repeatition: usize,
    step: usize,
    target: usize,
    echoPrint: bool,
    selfCheck: bool,
    randomBuffer: Vec<usize>,
    #[ignore]
    current_sort_type: SortType,
    #[ignore]
    sorted_buffer: Vec<usize>
}
