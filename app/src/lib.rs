use leptos::*;
use neutron_leptos::Button;

#[component]
pub fn App() -> impl IntoView {
  view! {
    <div>
      <h1>Demo App</h1>
      <Button>Click Me</Button>
    </div>
  }
}