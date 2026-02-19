use godot::prelude::*;

struct PresentationExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PresentationExtension {}

#[derive(GodotClass)]
#[class(base=Node, init)]
struct PresentationInfo {
    base: Base<Node>,

    #[export]
    deck: Array<Gd<PackedScene>>,
    current_slide: usize,

    #[export]
    target: Option<Gd<Node>>,
}

#[godot_api]
impl INode for PresentationInfo {
    fn ready(&mut self) {
        if self.target.is_none() {
            godot_print!("no target defined");
            return
        }

        if self.deck.len() == 0 {
            godot_print!("no slides in deck");
            return
        }

        self.set_slide(0);
    }
}


#[godot_api]
impl PresentationInfo {
    #[func]
    fn next_slide(&mut self) {
        let slide_count = self.deck.len();
        if self.current_slide + 1 >= slide_count {
            return
        }

        self.current_slide += 1;
        self.set_slide(self.current_slide);
    }

    #[func]
    fn last_slide(&mut self) {
        if self.current_slide == 0 {
            return
        }
        self.current_slide -= 1;
        self.set_slide(self.current_slide);
    }

    fn set_slide(&mut self, slide: usize) {
        let target = self.target.as_mut().unwrap();
        for child in target.get_children().iter_shared() {
            target.remove_child(&child)
        }

        let current = self.deck.get(slide).unwrap();
        let instanced = current.instantiate().unwrap();

        target.add_child(&instanced);
    }
}




