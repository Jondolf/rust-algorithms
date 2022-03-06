use crate::components::input_items::input_title_to_id;

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SelectInputProps {
    pub title: String,
    pub options: Vec<String>,
    pub selected_value: String,
    pub onchange: Callback<Event>,
}

#[function_component(SelectInput)]
pub fn select_input(props: &SelectInputProps) -> Html {
    let SelectInputProps {
        title,
        options,
        selected_value,
        onchange,
    } = props.clone();
    let id = input_title_to_id(&title);

    html! {
        <div class="input select-input">
            <label for={id.clone()}>{title.to_string()}</label>
            <select {id} {onchange}>
                {
                    options.into_iter().map(|option| {
                        html!{
                            <option
                                value={option.to_string()}
                                selected={selected_value == option}
                                option={option.to_string()}
                            >{option}</option>
                        }
                    }).collect::<Html>()
                }
            </select>
        </div>
    }
}
