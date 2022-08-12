use runtime::*;
use runtime::wasm_bindgen::JsCast;

ctx! {
    count: u32 = 0,
}

fn handle_click(ctx: &mut Ctx) {
    *ctx.count += 1;
}

pub struct Comp {
    ctx: Ctx,
    button0: web::HtmlButtonElement,
    t0: web::Text,
    t1: web::Text,
    t2: web::Text,
}

impl Component for Comp {
    fn create() -> Self {
        let ctx = Ctx::new();

        Self {
            button0: web::window().unwrap().document().unwrap().create_element("button").unwrap().dyn_into().unwrap(),
            t0: web::Text::new_with_data("Clicked ").unwrap(),
            t1: web::Text::new_with_data(&ctx.count.to_string()).unwrap(),
            t2: web::Text::new_with_data(" time(s)").unwrap(),
            ctx,
        }
    }

    fn make_fragment(&mut self) -> web::DocumentFragment {
        let frag = web::DocumentFragment::new().unwrap();
        self.button0.append_with_node_1(&self.t0).unwrap();
        self.button0.append_with_node_1(&self.t1).unwrap();
        self.button0.append_with_node_1(&self.t2).unwrap();

        frag.append_with_node_1(&self.button0).unwrap();

        frag
    }

    fn update(&mut self) {
        if self.ctx.count.is_dirty() {
            self.t1.set_text_content(Some(&self.ctx.count.to_string()));
        }
    }
}
