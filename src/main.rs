#![allow(non_snake_case)]
use leptos::*;

fn main() {
	console_error_panic_hook::set_once();
	mount_to_body(|| view! { <App/> })
}
#[component]
fn app() -> impl IntoView {
	let (count, set_count) = create_signal(0);
	let double_count = move || count() * 2;
	let (value, set_value) = create_signal("B".to_string());
	view! {
		<ButtonComp count=count set_count=set_count/>
		<ProgressBar progress=count/>
		<ProgressBarWithOptional progress=count/>
		<ProgressBarWithOptional max=200 progress=count/>
		<GenericProgressBarWithDefault progress=count/>
		<ProgressBarWithRandomSignals progress=count/>
		<ProgressBarWithRandomSignals progress=Signal::derive(double_count)/>
		<ProgressBarWithOptionalGeneric/>
		<StaticIteration/>
		<StaticIterWithHelper/>
		<DynamicItemsAsPartOfAStaticList/>
		<DynamicList initial_length=5/>
		<ControlledInput/>
		<UncontrolledInput/>
		<select on:change=move |ev| {
			let new_value = event_target_value(&ev);
			set_value(new_value);
		}>
			<SelectOption value is="A"/>
			<SelectOption value is="B"/>
			<SelectOption value is="C"/>
		</select>
		<Flow/>
		<NumericInput/>
		<NumericInputWithBoundry/>
		<ParentChildComm1/>
		<ParentChildWithCallback/>
		<ParentChildWithClosure/>
		<ParentChildWithListener/>
		<ParentChildWithContext/>
		<ParentChildWithContextApi/>
		<TakesChildren render_prop=|| {
			view! { <p>"Hi, there!"</p> }
		}>
			// these get passed to `children`
			"Some text " <span>"A span"</span>
		</TakesChildren>
		<WrapsChildren>"A" "B" "C"</WrapsChildren>
	}
}
#[component]
fn ButtonComp(count: ReadSignal<i32>, set_count: WriteSignal<i32>) -> impl IntoView {
	view! {
		<button
			on:click=move |_| {
				set_count.update(|n| *n += 10);
			}

			// set the `style` attribute
			style="position: absolute"
			// and toggle individual CSS properties with `style:`
			style:left=move || format!("{}px", count() + 100)
			style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
			style:max-width="400px"
			// Set a CSS variable for stylesheet use
			style=("--columns", count)
		>
			"Click to Move"
		</button>
	}
}
#[allow(unused)]
fn button_func(count: ReadSignal<i32>, set_count: WriteSignal<i32>) -> impl IntoView {
	html::button()
		.on(ev::click, move |_| set_count.update(|n| *n += 10))
		.child("Count")
}
#[component]
fn ProgressBar(progress: ReadSignal<i32>) -> impl IntoView {
	view! {
		<progress
			max="50"
			// now this works
			value=progress
		></progress>
	}
}
#[component]
fn ProgressBarWithOptional(#[prop(optional)] max: u16, progress: ReadSignal<i32>) -> impl IntoView {
	view! { <progress max=max value=progress></progress> }
}
#[component]
fn GenericProgressBarWithDefault<F: Fn() -> i32 + 'static>(
	#[prop(default = 100)] max: u16,
	progress: F,
) -> impl IntoView {
	view! { <progress max=max value=progress></progress> }
}
/// Shows progress toward a goal.
#[component]
fn ProgressBarWithRandomSignals(
	/// The maximum value of the progress bar.
	#[prop(default = 100)]
	max: u16,
	/// How much progress should be displayed.
	#[prop(into)]
	progress: Signal<i32>,
) -> impl IntoView {
	view! { <progress max=max value=progress></progress> }
}
#[component]
fn ProgressBarWithOptionalGeneric(#[prop(optional)] progress: Option<Box<dyn Fn() -> i32>>) -> impl IntoView {
	progress.map(|progress| {
		view! { <progress max=100 value=progress></progress> }
	})
}
#[component]
fn StaticIteration() -> impl IntoView {
	let values = vec![0, 1, 2];
	view! {
		<p>{values.clone()}</p>
		// or we can wrap them in <li>
		<ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect::<Vec<_>>()}</ul>
	}
}
#[component]
fn StaticIterWithHelper() -> impl IntoView {
	let values = vec![0, 1, 2];
	view! {
		<p>{values.clone()}</p>
		// or we can wrap them in <li>
		<ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
	}
}
#[component]
fn DynamicItemsAsPartOfAStaticList() -> impl IntoView {
	// create a list of 5 signals
	let length = 5;
	let counters = (1..=length).map(|idx| create_signal(idx));

	// each item manages a reactive view
	// but the list itself will never change
	let counter_buttons = counters
		.map(|(count, set_count)| {
			view! {
				<li>
					<button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
				</li>
			}
		})
		.collect_view();

	view! { <ul>{counter_buttons}</ul> }
}
/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
	/// The number of counters to begin with.
	initial_length: usize,
) -> impl IntoView {
	// This dynamic list will use the <For/> component.
	// <For/> is a keyed list. This means that each row
	// has a defined key. If the key does not change, the row
	// will not be re-rendered. When the list changes, only
	// the minimum number of changes will be made to the DOM.

	// `next_counter_id` will let us generate unique IDs
	// we do this by simply incrementing the ID by one
	// each time we create a counter
	let mut next_counter_id = initial_length;

	// we generate an initial list as in <StaticList/>
	// but this time we include the ID along with the signal
	let initial_counters = (0..initial_length).map(|id| (id, create_signal(id + 1))).collect::<Vec<_>>();

	// now we store that initial list in a signal
	// this way, we'll be able to modify the list over time,
	// adding and removing counters, and it will change reactively
	let (counters, set_counters) = create_signal(initial_counters);

	let add_counter = move |_| {
		// create a signal for the new counter
		let sig = create_signal(next_counter_id + 1);
		// add this counter to the list of counters
		set_counters.update(move |counters| {
			// since `.update()` gives us `&mut T`
			// we can just use normal Vec methods like `push`
			counters.push((next_counter_id, sig))
		});
		// increment the ID so it's always unique
		next_counter_id += 1;
	};

	view! {
		<div>
			<button on:click=add_counter>"Add Counter"</button>
			<ul>
				// The <For/> component is central here
				// This allows for efficient, key list rendering
				<For
					// `each` takes any function that returns an iterator
					// this should usually be a signal or derived signal
					// if it's not reactive, just render a Vec<_> instead of <For/>
					each=counters
					// the key should be unique and stable for each row
					// using an index is usually a bad idea, unless your list
					// can only grow, because moving items around inside the list
					// means their indices will change and they will all rerender
					key=|counter| counter.0
					// `children` receives each item from your `each` iterator
					// and returns a view
					children=move |(id, (count, set_count))| {
						view! {
							<li>
								<button on:click=move |_| {
									set_count.update(|n| *n += 1)
								}>{count}</button>
								<button on:click=move |_| {
									set_counters
										.update(|counters| {
											counters
												.retain(|(counter_id, (signal, _))| {
													if counter_id == &id {
														signal.dispose();
													}
													counter_id != &id
												})
										});
								}>

									"Remove"
								</button>
							</li>
						}
					}
				/>

			</ul>
		</div>
	}
}
#[component]
fn ControlledInput() -> impl IntoView {
	let (name, set_name) = create_signal("Controlled".to_string());

	view! {
		<input
			type="text"
			on:input=move |ev| {
				set_name(event_target_value(&ev));
			}

			// the `prop:` syntax lets you update a DOM property,
			// rather than an attribute.
			prop:value=name
		/>
		<p>"Name is: " {name}</p>
	}
}
#[component]
fn UncontrolledInput() -> impl IntoView {
	let (name, set_name) = create_signal("Uncontrolled".to_string());

	let input_element: NodeRef<html::Input> = create_node_ref();

	let on_submit = move |ev: leptos::ev::SubmitEvent| {
		// stop the page from reloading!
		ev.prevent_default();

		// here, we'll extract the value from the input
		let value = input_element()
			// event handlers can only fire after the view
			// is mounted to the DOM, so the `NodeRef` will be `Some`
			.expect("<input> should be mounted")
			// `leptos::HtmlElement<html::Input>` implements `Deref`
			// to a `web_sys::HtmlInputElement`.
			// this means we can call`HtmlInputElement::value()`
			// to get the current value of the input
			.value();
		set_name(value);
	};

	view! {
		<form on:submit=on_submit>
			<input type="text" value=name node_ref=input_element/>
			<input type="submit" value="Submit"/>
		</form>
		<p>"Name is: " {name}</p>
	}
}
#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
	view! {
		<option value=is selected=move || value() == is>
			{is}
		</option>
	}
}
#[component]
fn Flow() -> impl IntoView {
	let (value, set_value) = create_signal(0);
	let is_odd = move || value() & 1 == 1;
	let odd_text = move || if is_odd() { Some("How odd!") } else { None };

	view! {
		<h1>"Control Flow"</h1>

		// Simple UI to update and show a value
		<button on:click=move |_| set_value.update(|n| *n += 1)>"+1"</button>
		<p>"Value is: " {value}</p>

		<hr/>

		<h2>
			<code>"Option<T>"</code>
		</h2>
		// For any `T` that implements `IntoView`,
		// so does `Option<T>`

		<p>{odd_text}</p>
		// This means you can use `Option` methods on it
		<p>{move || odd_text().map(|text| text.len())}</p>

		<h2>"Conditional Logic"</h2>
		// You can do dynamic conditional if-then-else
		// logic in several ways
		//
		// a. An "if" expression in a function
		// This will simply re-render every time the value
		// changes, which makes it good for lightweight UI
		<p>{move || if is_odd() { "Odd" } else { "Even" }}</p>

		// b. Toggling some kind of class
		// This is smart for an element that's going to
		// toggled often, because it doesn't destroy
		// it in between states
		// (you can find the `hidden` class in `index.html`)
		<p class:hidden=is_odd>"Appears if even."</p>

		// c. The <Show/> component
		// This only renders the fallback and the child
		// once, lazily, and toggles between them when
		// needed. This makes it more efficient in many cases
		// than a {move || if ...} block
		<Show when=is_odd fallback=|| view! { <p>"Even steven"</p> }>
			<p>"Oddment"</p>
		</Show>

		// d. Because `bool::then()` converts a `bool` to
		// `Option`, you can use it to create a show/hide toggled
		{move || is_odd().then(|| view! { <p>"Oddity!"</p> })}

		<h2>"Converting between Types"</h2>
		// e. Note: if branches return different types,
		// you can convert between them with
		// `.into_any()` (for different HTML element types)
		// or `.into_view()` (for all view types)
		{move || match is_odd() {
			true if value() == 1 => {
				view! {
					// <pre> returns HtmlElement<Pre>
					<pre>"One"</pre>
				}
					.into_any()
			}
			false if value() == 2 => {
				view! {
					// <pre> returns HtmlElement<Pre>

					// <pre> returns HtmlElement<Pre>
					// <p> returns HtmlElement<P>
					// so we convert into a more generic type
					<p>"Two"</p>
				}
					.into_any()
			}
			_ => {
				view! {
					// <pre> returns HtmlElement<Pre>

					// <pre> returns HtmlElement<Pre>
					// <p> returns HtmlElement<P>
					// so we convert into a more generic type

					// <pre> returns HtmlElement<Pre>
					// <p> returns HtmlElement<P>
					// so we convert into a more generic type

					<textarea>{value()}</textarea>
				}
					.into_any()
			}
		}}
	}
}
#[component]
fn NumericInput() -> impl IntoView {
	let (value, set_value) = create_signal(Ok(0));

	// when input changes, try to parse a number from the input
	let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

	view! {
		<label>
			"Type a number (or not!)" <input type="number" on:input=on_input/>
			<p>"You entered " <strong>{value}</strong></p>
		</label>
	}
}
#[component]
fn NumericInputWithBoundry() -> impl IntoView {
	let (value, set_value) = create_signal(Ok(0));

	let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

	view! {
		<h1>"Error Handling"</h1>
		<label>
			"Type a number (or something that's not a number!)"
			<input type="number" on:input=on_input/>
			// the fallback receives a signal containing current errors
			<ErrorBoundary fallback=|errors| {
				view! {
					<div class="error">
						<p>"Not a number! Errors: "</p>
						// we can render a list of errors as strings, if we'd like
						<ul>
							{move || {
								errors
									.get()
									.into_iter()
									.map(|(_, e)| view! { <li>{e.to_string()}</li> })
									.collect_view()
							}}

						</ul>
					</div>
				}
			}>

				<p>"You entered " <strong>{value}</strong></p>
			</ErrorBoundary>
		</label>
	}
}
#[component]
pub fn ParentChildComm1() -> impl IntoView {
	let (toggled, set_toggled) = create_signal(false);
	view! {
		<p>"Toggled? " {toggled}</p>
		<ButtonA setter=set_toggled/>
	}
}

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
	view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}
#[component]
pub fn ParentChildWithCallback() -> impl IntoView {
	let (toggled, set_toggled) = create_signal(false);
	view! {
		<p>"Toggled? " {toggled}</p>
		<ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
	}
}

#[component]
pub fn ButtonB(#[prop(into)] on_click: Callback<ev::MouseEvent>) -> impl IntoView {
	view! { <button on:click=on_click>"Toggle"</button> }
}
#[component]
pub fn ParentChildWithClosure() -> impl IntoView {
	let (toggled, set_toggled) = create_signal(false);
	view! {
		<p>"Toggled? " {toggled}</p>
		<ButtonC on_click=move |_| set_toggled.update(|value| *value = !*value)/>
	}
}

#[component]
pub fn ButtonC<F>(on_click: F) -> impl IntoView
where
	F: Fn(ev::MouseEvent) + 'static,
{
	view! { <button on:click=on_click>"Toggle"</button> }
}
#[component]
pub fn ParentChildWithListener() -> impl IntoView {
	let (toggled, set_toggled) = create_signal(false);
	view! {
		<p>"Toggled? " {toggled}</p>
		// note the on:click instead of on_click
		// this is the same syntax as an HTML element event listener
		<ButtonD on:click=move |_| set_toggled.update(|value| *value = !*value)/>
	}
}

#[component]
pub fn ButtonD() -> impl IntoView {
	view! { <button>"Toggle"</button> }
}
#[component]
pub fn ParentChildWithContext() -> impl IntoView {
	let (toggled, set_toggled) = create_signal(false);
	view! {
		<p>"Toggled? " {toggled}</p>
		<Layout set_toggled/>
	}
}

#[component]
pub fn Layout(set_toggled: WriteSignal<bool>) -> impl IntoView {
	view! {
		<header>
			<h1>"My Page"</h1>
		</header>
		<main>
			<Content set_toggled/>
		</main>
	}
}

#[component]
pub fn Content(set_toggled: WriteSignal<bool>) -> impl IntoView {
	view! {
		<div class="content">
			<ButtonF set_toggled/>
		</div>
	}
}

#[component]
pub fn ButtonF(set_toggled: WriteSignal<bool>) -> impl IntoView {
	view! { <button on:click=move |_| set_toggled.update(|value| *value = !*value)>"Toggle"</button> }
}
#[component]
pub fn ParentChildWithContextApi() -> impl IntoView {
	let (toggled, set_toggled) = create_signal(false);

	// share `set_toggled` with all children of this component
	provide_context(set_toggled);

	view! {
		<p>"Toggled? " {toggled}</p>
		<Layout2/>
	}
}
#[component]
pub fn Layout2() -> impl IntoView {
	view! {
		<header>
			<h1>"My Page"</h1>
		</header>
		<main>
			<Content2/>
		</main>
	}
}
#[component]
pub fn Content2() -> impl IntoView {
	view! {
		<div class="content">
			<ButtonE/>
		</div>
	}
}

// <Layout/> and <Content/> omitted
// To work in this version, drop their references to set_toggled

#[component]
pub fn ButtonE() -> impl IntoView {
	// use_context searches up the context tree, hoping to
	// find a `WriteSignal<bool>`
	// in this case, I .expect() because I know I provided it
	let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided");

	view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}
#[component]
pub fn TakesChildren<F, IV>(
	/// Takes a function (type F) that returns anything that can be
	/// converted into a View (type IV)
	render_prop: F,
	/// `children` takes the `Children` type
	children: Children,
) -> impl IntoView
where
	F: Fn() -> IV,
	IV: IntoView,
{
	view! {
		<h2>"Render Prop"</h2>
		{render_prop()}

		<h2>"Children"</h2>
		{children()}
	}
}
#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
	// Fragment has `nodes` field that contains a Vec<View>
	let children = children()
		.nodes
		.into_iter()
		.map(|child| view! { <li>{child}</li> })
		.collect_view();

	view! { <ul>{children}</ul> }
}
