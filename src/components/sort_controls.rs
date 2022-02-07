use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::pages::sorting_algorithms::{SortConfig, SORTING_ALGORITHMS};
use crate::utils::gen_i32_vec;

#[derive(Properties, Clone, PartialEq)]
pub struct SortControlsProps {
    pub config: SortConfig,
    pub update_input: Callback<Vec<i32>>,
    pub update_config: Callback<SortConfig>,
}

#[function_component(SortControls)]
pub fn sort_controls(props: &SortControlsProps) -> Html {
    let SortControlsProps {
        config,
        update_input,
        update_config,
    } = props.clone();

    let gen_input = {
        let config = config.clone();
        let update_input = update_input.clone();

        move |_e: MouseEvent| {
            update_input.emit(gen_i32_vec(
                config.input_len,
                config.min_val,
                config.max_val,
            ));
        }
    };
    let change_input_len = {
        let config = config.clone();
        let update_config = update_config.clone();

        move |e: InputEvent| {
            let el: HtmlInputElement = e.target_unchecked_into();
            if let Ok(input_len) = el.value().parse::<usize>() {
                if input_len > 1 {
                    update_config.emit(SortConfig {
                        input_len,
                        ..config.clone()
                    });
                }
            }
        }
    };
    let change_min_val = {
        let config = config.clone();
        let update_config = update_config.clone();

        move |e: InputEvent| {
            let el: HtmlInputElement = e.target_unchecked_into();
            if let Ok(min_val) = el.value().parse::<isize>() {
                update_config.emit(SortConfig {
                    min_val,
                    ..config.clone()
                });
            }
        }
    };
    let change_max_val = {
        let config = config.clone();
        let update_config = update_config.clone();

        move |e: InputEvent| {
            let el: HtmlInputElement = e.target_unchecked_into();
            if let Ok(max_val) = el.value().parse::<isize>() {
                update_config.emit(SortConfig {
                    max_val,
                    ..config.clone()
                });
            }
        }
    };
    let change_algorithm = {
        let config = config.clone();
        let update_config = update_config.clone();

        move |e: Event| {
            let el: HtmlInputElement = e.target_unchecked_into();
            let sorting_algorithm = SORTING_ALGORITHMS
                .iter()
                .find(|algorithm| algorithm.name == el.value())
                .unwrap()
                .clone();
            update_config.emit(SortConfig {
                sorting_algorithm,
                ..config
            });
        }
    };

    html! {
        <>
            <button onclick={gen_input}>{"Generate list of i32"}</button>
            <input type="number"
                placeholder="Input length"
                min=1
                value={props.config.input_len.to_string()}
                oninput={change_input_len}
            />
            <input type="number"
                placeholder="Minimum value"
                value={props.config.min_val.to_string()}
                oninput={change_min_val}
            />
            <input type="number"
                placeholder="Maximum value"
                value={props.config.max_val.to_string()}
                oninput={change_max_val}
            />
            <select name="Sorting algorithm" onchange={change_algorithm}>
                { for SORTING_ALGORITHMS.iter().map(|a| view_sorting_algorithm_option(config.sorting_algorithm.name, a.name)) }
            </select>
        </>
    }
}

fn view_sorting_algorithm_option(curr_algorithm: &str, name: &str) -> Html {
    html! {
        <option value={name.to_string()} selected={curr_algorithm == name}>{name}</option>
    }
}